use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::Error;
use reqwest::{Response};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use twilight_model::gateway::event::DispatchEvent;

use crate::constants::{CLIENT, DISPATCH_STRATEGIES, HTTP_ENDPOINT, QUEUE_ENDPOINT};
use crate::middleware::{MiddlewareResponse, get_middlewares};

pub enum DispatchStrategy {
    Smart { queue_delay: u64 },         // based on rate-limiter/429
    
    AlwaysWorker,                       // Use always worker and do not care about 429
    AlwaysQueue { queue_delay: u64 },   // Use Always queue

    WorkerOnly                          // Use worker only when unavailable
}

pub struct Dispatcher {
    in_fallback_mode: AtomicBool,
    last_429_time: Arc<Mutex<Option<Instant>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            in_fallback_mode: AtomicBool::new(false),
            last_429_time: Arc::new(Mutex::new(None))
        }
    }
}

impl Dispatcher {
    pub async fn dispatch(&self, event: &DispatchEvent) -> Result<(), anyhow::Error> {
        let kind = event.kind();
        
        let strategy = DISPATCH_STRATEGIES
            .iter()
            .find(|(flags, _)| flags.contains(kind.into()))
            .map(|(_, strategy)| strategy)
            .unwrap_or(&DispatchStrategy::Smart { queue_delay: 0 });

        println!("📥 [EVENT] {:?}", kind);

        let event_value = serde_json::to_value(event)?;

        let mut accumulated_metadata = serde_json::Map::new();

        for middleware in get_middlewares(kind.into()) {
            println!("  ⚙️  [MIDDLEWARE] Running: {}", middleware.name());
            
            match middleware.execute(event, strategy)? {
                MiddlewareResponse::Discard => {
                    println!("  🛑 [MIDDLEWARE] Event discarded by {}", middleware.name());
                    return Ok(());
                }
                MiddlewareResponse::SendWithMetadata(new_metadata) => {
                    if let serde_json::Value::Object(obj) = new_metadata {
                        accumulated_metadata.extend(obj);
                    } else {
                        println!(
                            "  ⚠️  [MIDDLEWARE] Warning: {} returned metadata that is not a JSON Object. Ignored.", 
                            middleware.name()
                        );
                    }
                }
                MiddlewareResponse::Send => {}
            }
        }

        let metadata = if accumulated_metadata.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::Value::Object(accumulated_metadata)
        };

        let payload = json!({
            "kind" : kind,
            "event": event_value,
            "metadata": metadata
        });

        match strategy {
            DispatchStrategy::AlwaysQueue { queue_delay } => {
                println!("  ↳ 📥 [ROUTE] Strategia: AlwaysQueue (Delay: {}s)", queue_delay);
                self.send_to_queue(&payload, *queue_delay).await
            },
            DispatchStrategy::AlwaysWorker => {
                println!("  ↳ ⚡ [ROUTE] Strategia: AlwaysWorker");
                self.send_to_worker(&payload).await
            },
            DispatchStrategy::Smart { queue_delay } => {
                self.send(&payload, *queue_delay).await
            },
            DispatchStrategy::WorkerOnly => {
                println!("  ↳ ⚡ [ROUTE] Strategia: WorkerOnly");

                if !self.in_fallback_mode.load(Ordering::SeqCst) {
                    return Err(Error::msg("Could not send event because gateway is not on fallback mode"));
                };

                self.send_to_worker(&payload).await
            }
        }?;

        Ok(())
    }

    async fn trigger_fallback(&self) {
        self.in_fallback_mode.store(true, Ordering::SeqCst);
        let mut last_time = self.last_429_time.lock().await;
        *last_time = Some(Instant::now());
    }

    async fn send(&self, payload: &Value, queue_delay: u64) -> Result<Response, anyhow::Error> {
        let now = Instant::now();
        
        if self.in_fallback_mode.load(Ordering::SeqCst) {
            let last_time = self.last_429_time.lock().await;
            if let Some(time) = *last_time {
                if now.duration_since(time) > Duration::from_secs(60) {
                    println!("  ↳ ♻️ [SMART] Cooldown terminato. Tento ripristino verso Worker.");
                    self.in_fallback_mode.store(false, Ordering::SeqCst);
                } else {
                    println!("  ↳ ⚠️ [SMART] Fallback attivo (Cooldown). Routing verso Queue.");
                    return self.send_to_queue(&payload, queue_delay).await;
                }
            }
        }

        let res = self.send_to_worker(&payload).await;

        match res {
            Ok(response) if response.status().is_success() => {
                println!("    ✅ [WORKER] Inviato con successo");
                Ok(response)
            },
            Ok(response) if response.status().as_u16() == 429 => {
                eprintln!("    🛑 [RATELIMIT] 429 ricevuto dal Worker! Attivo Fallback Mode.");
                self.trigger_fallback().await;
                self.send_to_queue(&payload, queue_delay).await
            },
            _ => {
                eprintln!("    ❌ [ERROR] Worker non raggiungibile. Backup su Queue.");
                self.send_to_queue(&payload, queue_delay).await
            }
        }
    }

    async fn send_to_worker(&self, payload: &Value) -> Result<Response, anyhow::Error> {
        let event_kind = payload
            .get("kind")
            .and_then(|k| k.as_str())
            .unwrap_or("unknown");

        let response = CLIENT
            .post(&*HTTP_ENDPOINT)
            .header("X-Event-Kind", event_kind)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    async fn send_to_queue(&self, payload: &Value, queue_delay: u64) -> Result<Response, anyhow::Error> {
        let payload = json!({
            "body" : payload, 
            "content_type" : "json",
            "delay_seconds": queue_delay
        });

        let response = CLIENT
            .post(&*QUEUE_ENDPOINT)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }
}
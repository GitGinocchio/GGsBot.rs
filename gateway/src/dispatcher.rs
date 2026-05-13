use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tokio::sync::Mutex;

use crate::constants::{CLIENT, HTTP_ENDPOINT, QUEUE_ENDPOINT};

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
    pub async fn send_event(&self, payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let now = Instant::now();
        
        // Verifica se dobbiamo "raffreddare" il fallback
        if self.in_fallback_mode.load(Ordering::SeqCst) {
            let last_time = self.last_429_time.lock().await;
            if let Some(time) = *last_time {
                if now.duration_since(time) > Duration::from_secs(60) {
                    // Reset: proviamo a tornare al worker
                    self.in_fallback_mode.store(false, Ordering::SeqCst);
                } else {
                    // Siamo ancora in cooldown, vai alla coda
                    return self.send_to_queue(payload, 0).await;
                }
            }
        }

        // Tentativo primario: Worker
        let res = CLIENT.post(&*HTTP_ENDPOINT).json(payload).send().await;

        match res {
            Ok(response) if response.status().is_success() => Ok(response.json().await?),
            Ok(response) if response.status().as_u16() == 429 => {
                self.trigger_fallback().await;
                self.send_to_queue(payload, 0).await
            },
            _ => {
                // Errore generico o timeout: mandiamo in coda per sicurezza ma senza bloccare il traffico futuro
                self.send_to_queue(payload, 0).await
            }
        }
    }

    async fn trigger_fallback(&self) {
        self.in_fallback_mode.store(true, Ordering::SeqCst);
        let mut last_time = self.last_429_time.lock().await;
        *last_time = Some(Instant::now());
    }

    async fn send_to_queue(&self, payload: &Value, delay_seconds: u8) -> Result<Value, Box<dyn std::error::Error>> {
        let payload = json!({
            "body" : payload, 
            "content_type" : "json",
            "delay_seconds": delay_seconds
        });

        let response: Value = CLIENT
            .post(&*QUEUE_ENDPOINT)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        println!("queue send message response: {response:?}");

        Ok(response)
    }
}
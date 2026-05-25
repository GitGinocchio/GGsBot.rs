use std::{collections::HashMap, env, sync::LazyLock};

use reqwest::{Client, ClientBuilder, header::{self, HeaderValue}};
use twilight_gateway::{EventType, EventTypeFlags, Intents};

use crate::{dispatcher::{DispatchStrategy, Dispatcher}, middleware::EventMiddleware, middlewares::{discard_self::DiscardSelfEventsMiddleware, voice_state::VoiceStateMiddleware}};

pub static DISPATCH_STRATEGIES: LazyLock<HashMap<EventType, DispatchStrategy>> = LazyLock::new(|| {
    let map = HashMap::new();
    
    map
});

pub static MIDDLEWARES: LazyLock<Vec<(EventTypeFlags, Box<dyn EventMiddleware>)>> = LazyLock::new(|| { 
    let mut list: Vec<(EventTypeFlags, Box<dyn EventMiddleware>)> = Vec::new();

    list.push((EventTypeFlags::all(), Box::new(DiscardSelfEventsMiddleware::new())));
    list.push((EventTypeFlags::VOICE_STATE_UPDATE, Box::new(VoiceStateMiddleware::new())));

    list
});

pub static WANTED_EVENTS: LazyLock<EventTypeFlags> = LazyLock::new(|| {
    EventTypeFlags::all()
        .difference(EventTypeFlags::GATEWAY_HEARTBEAT_ACK)
        .difference(EventTypeFlags::GATEWAY_HEARTBEAT)
        .difference(EventTypeFlags::GATEWAY_RECONNECT)
        .difference(EventTypeFlags::GATEWAY_INVALIDATE_SESSION)
        .difference(EventTypeFlags::GATEWAY_HELLO)
        .difference(EventTypeFlags::RESUMED)
});

pub const INTENTS: Intents = Intents::empty()
    .union(Intents::GUILD_MESSAGES)
    .union(Intents::MESSAGE_CONTENT)
    .union(Intents::DIRECT_MESSAGES)
    .union(Intents::GUILD_VOICE_STATES);

pub static DISPATCHER: LazyLock<Dispatcher> = LazyLock::new(|| Dispatcher::new());

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE,HeaderValue::from_static("application/json"));

    let auth = AUTHORIZATION_TOKEN.as_str();

    if auth.len() > 0 {
        headers.insert(
            header::AUTHORIZATION, 
            HeaderValue::from_str(&format!("Bearer {}", auth)).unwrap()
        );
    }

    ClientBuilder::new()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("failed to create http client")
});

pub static HTTP_ENDPOINT: LazyLock<String> = LazyLock::new(|| {
    env::var("HTTP_ENDPOINT").expect("missing HTTP_ENDPOINT")
});

pub static QUEUE_ENDPOINT: LazyLock<String> = LazyLock::new(|| {
    env::var("QUEUE_ENDPOINT").expect("missing QUEUE_ENDPOINT")
});

pub static AUTHORIZATION_TOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTHORIZATION_TOKEN").unwrap_or_default()
});

pub static DISCORD_TOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN")
});

pub static BOT_ID: LazyLock<String> = LazyLock::new(|| {
    env::var("BOT_ID").expect("missing BOT_ID")
});
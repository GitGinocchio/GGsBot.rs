use std::env;

use once_cell::sync::Lazy;
use serenity::all::GatewayIntents;
use reqwest::{Client, ClientBuilder, header::{self, HeaderValue}};

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
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

pub static HTTP_ENDPOINT: Lazy<String> = Lazy::new(|| {
    env::var("HTTP_ENDPOINT").expect("missing HTTP_ENDPOINT")
});

pub static AUTHORIZATION_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("AUTHORIZATION_TOKEN").unwrap_or_default()
});

pub static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN")
});

pub const INTENTS: GatewayIntents = GatewayIntents::empty()
    .union(GatewayIntents::GUILD_MESSAGES)
    .union(GatewayIntents::MESSAGE_CONTENT)
    .union(GatewayIntents::DIRECT_MESSAGES)
    .union(GatewayIntents::GUILD_VOICE_STATES);
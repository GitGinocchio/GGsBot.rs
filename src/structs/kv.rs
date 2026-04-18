use std::collections::HashMap;

use worker::kv::{KvStore, ListResponse};
use worker::web_sys::ReadableStream;
use worker::KvError;

pub struct NamespacedKv {
    store: KvStore,
    pub prefix: String,
}

impl NamespacedKv {
    pub fn new(store: KvStore, prefix: String) -> Self {
        Self { store, prefix }
    }

    fn format_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }

    #[allow(unused)]
    pub async fn list(&self, prefix: Option<String>, limit: Option<u64>, cursor: Option<String>) -> Result<ListResponse, KvError> {
        let mut builder = self.store.list().prefix(format!("{}:{}", self.prefix, prefix.unwrap_or_default()));
        
        if let Some(l) = limit { builder = builder.limit(l); }
        if let Some(c) = cursor { builder = builder.cursor(c); }
        
        builder.execute().await
    }

    #[allow(unused)]
    pub async fn get(&self, key: &str) -> Result<Option<String>, KvError> {
        self.store
            .get(&self.format_key(key))
            .text()
            .await
    }

    #[allow(unused)]
    pub async fn get_bulk(&self, keys: &[impl AsRef<str>]) -> Result<HashMap<String, Option<String>>, KvError> {
        let formatted_keys: Vec<String> = keys
            .iter()
            .map(|k| self.format_key(k.as_ref()))
            .collect();

        self.store
            .get_bulk(&formatted_keys)
            .text()
            .await
    }

    #[allow(unused)]
    pub async fn put(&self, key: &str, value: impl Into<String>, ttl: Option<u64>) -> Result<(), KvError> {
        let mut builder = self.store.put(&self.format_key(key), value.into())?;
        
        if let Some(seconds) = ttl {
            builder = builder.expiration_ttl(seconds);
        }

        builder.execute().await
    }

    #[allow(unused)]
    pub async fn put_bytes(&self, key: &str, bytes: &[u8], ttl: Option<u64>) -> Result<(), KvError> {
        let mut builder = self.store.put_bytes(&self.format_key(key), bytes)?;

        if let Some(seconds) = ttl {
            builder = builder.expiration_ttl(seconds);
        }

        builder.execute().await
    }

    #[allow(unused)]
    pub async fn put_stream(&self, key: &str, stream: ReadableStream, ttl: Option<u64>) -> Result<(), KvError> {
        let mut builder = self.store.put_stream(&self.format_key(key), stream)?;

        if let Some(seconds) = ttl {
            builder = builder.expiration_ttl(seconds);
        }

        builder.execute().await
    }

    #[allow(unused)]
    pub async fn delete(&self, key: &str) -> Result<(), KvError> {
        self.store.delete(&self.format_key(key)).await
    }
}
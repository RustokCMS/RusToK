use std::sync::Arc;

use async_trait::async_trait;

use crate::cache::CacheStats;
use crate::events::EventTransport;
use crate::Result;

#[async_trait]
pub trait CacheBackend: Send + Sync {
    async fn health(&self) -> Result<()>;
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: String, value: Vec<u8>) -> Result<()>;
    async fn invalidate(&self, key: &str) -> Result<()>;
    fn stats(&self) -> CacheStats;
}

#[async_trait]
pub trait SearchBackend: Send + Sync {
    async fn health(&self) -> Result<()>;
}

pub struct AppContext {
    pub events: Arc<dyn EventTransport>,
    pub cache: Arc<dyn CacheBackend>,
    pub search: Arc<dyn SearchBackend>,
}

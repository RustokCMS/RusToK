use std::time::Duration;

use async_trait::async_trait;
use moka::future::Cache;

use crate::context::CacheBackend;
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub entries: u64,
}

pub struct InMemoryCacheBackend {
    cache: Cache<String, Vec<u8>>,
}

impl InMemoryCacheBackend {
    pub fn new(ttl: Duration, max_capacity: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(ttl)
            .max_capacity(max_capacity)
            .record_stats()
            .build();
        Self { cache }
    }
}

#[async_trait]
impl CacheBackend for InMemoryCacheBackend {
    async fn health(&self) -> Result<()> {
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.cache.get(key).await)
    }

    async fn set(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.cache.insert(key, value).await;
        Ok(())
    }

    async fn invalidate(&self, key: &str) -> Result<()> {
        self.cache.invalidate(key).await;
        Ok(())
    }

    fn stats(&self) -> CacheStats {
        let stats = self.cache.stats();
        CacheStats {
            hits: stats.hit_count(),
            misses: stats.miss_count(),
            evictions: stats.eviction_count(),
            entries: self.cache.entry_count(),
        }
    }
}

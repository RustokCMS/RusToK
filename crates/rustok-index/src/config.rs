#[derive(Debug, Clone)]
pub struct IndexConfig {
    /// Reindex batch size
    pub batch_size: usize,

    /// Parallel workers for reindexing
    pub workers: usize,

    /// Enable real-time sync via events
    pub realtime_sync: bool,

    /// Full reindex schedule (cron)
    pub reindex_schedule: Option<String>,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            workers: 4,
            realtime_sync: true,
            reindex_schedule: Some("0 3 * * *".to_string()),
        }
    }
}

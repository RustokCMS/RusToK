#[derive(Debug, Default, Clone, Copy)]
pub struct IndexStats {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
}

impl IndexStats {
    pub fn record_success(&mut self) {
        self.success += 1;
        self.total += 1;
    }

    pub fn record_failure(&mut self) {
        self.failed += 1;
        self.total += 1;
    }
}

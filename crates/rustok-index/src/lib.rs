pub mod config;
pub mod indexers;
pub mod stats;

use async_trait::async_trait;
use rustok_core::{EventListener, MigrationSource, RusToKModule};
use sea_orm_migration::MigrationTrait;

pub struct IndexModule;

#[async_trait]
impl RusToKModule for IndexModule {
    fn slug(&self) -> &'static str {
        "index"
    }

    fn name(&self) -> &'static str {
        "Index"
    }

    fn description(&self) -> &'static str {
        "CQRS read models and search indexing"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn event_listeners(&self) -> Vec<Box<dyn EventListener>> {
        Vec::new()
    }
}

impl MigrationSource for IndexModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}

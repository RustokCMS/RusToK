//! RusToK Index - CQRS Read Model
//!
//! Denormalized indexes for fast reads and full-text search.

use async_trait::async_trait;
use rustok_core::{MigrationSource, RusToKModule};
use sea_orm_migration::MigrationTrait;

pub mod content;
pub mod error;
pub mod product;
pub mod search;
pub mod traits;

pub use error::{IndexError, IndexResult};
pub use traits::{Indexer, IndexerContext, LocaleIndexer};

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
        "CQRS Read Model (Fast Search)"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl MigrationSource for IndexModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_metadata() {
        let module = IndexModule;
        assert_eq!(module.slug(), "index");
        assert_eq!(module.name(), "Index");
        assert_eq!(module.description(), "CQRS Read Model (Fast Search)");
        assert_eq!(module.version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn module_migrations_empty() {
        let module = IndexModule;
        assert!(module.migrations().is_empty());
    }
}

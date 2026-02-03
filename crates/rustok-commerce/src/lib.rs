use async_trait::async_trait;
use rustok_core::{MigrationSource, RusToKModule};
use sea_orm_migration::MigrationTrait;

pub mod dto;
pub mod entities;
pub mod error;
pub mod services;

pub use dto::*;
pub use error::{CommerceError, CommerceResult};
pub use services::{CatalogService, InventoryService, PricingService};

pub struct CommerceModule;

#[async_trait]
impl RusToKModule for CommerceModule {
    fn slug(&self) -> &'static str {
        "commerce"
    }

    fn name(&self) -> &'static str {
        "Commerce"
    }

    fn description(&self) -> &'static str {
        "Products, Orders, Cart, Checkout"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl MigrationSource for CommerceModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_metadata() {
        let module = CommerceModule;
        assert_eq!(module.slug(), "commerce");
        assert_eq!(module.name(), "Commerce");
        assert_eq!(module.description(), "Products, Orders, Cart, Checkout");
        assert_eq!(module.version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn module_migrations_empty() {
        let module = CommerceModule;
        assert!(module.migrations().is_empty());
    }
}

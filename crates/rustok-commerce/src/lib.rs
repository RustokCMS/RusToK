use async_trait::async_trait;
use rustok_core::RusToKModule;
use sea_orm_migration::MigrationTrait;

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

    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}

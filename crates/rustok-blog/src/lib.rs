use async_trait::async_trait;
use rustok_core::{MigrationSource, RusToKModule};
use sea_orm_migration::MigrationTrait;

pub mod dto;
pub mod entities;
pub mod services;

pub use dto::CreatePostInput;
pub use services::PostService;

pub struct BlogModule;

#[async_trait]
impl RusToKModule for BlogModule {
    fn slug(&self) -> &'static str {
        "blog"
    }

    fn name(&self) -> &'static str {
        "Blog"
    }

    fn description(&self) -> &'static str {
        "Posts, Pages, Comments"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl MigrationSource for BlogModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_metadata() {
        let module = BlogModule;
        assert_eq!(module.slug(), "blog");
        assert_eq!(module.name(), "Blog");
        assert_eq!(module.description(), "Posts, Pages, Comments");
        assert_eq!(module.version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn module_migrations_empty() {
        let module = BlogModule;
        assert!(module.migrations().is_empty());
    }
}

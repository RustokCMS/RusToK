#![allow(elided_lifetimes_in_paths)]

pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_tenants;
mod m20250101_000002_create_users;
mod m20250101_000003_create_tenant_modules;
mod m20250101_000004_create_sessions;
mod m20250101_000005_create_roles_and_permissions;
mod m20250101_000006_add_metadata_to_tenants_and_users;
mod m20250130_000004_create_tenant_locales;
mod m20250130_000005_create_nodes;
mod m20250130_000006_create_categories;
mod m20250130_000007_create_tags;
mod m20250130_000008_create_meta;
mod m20250130_000009_create_media;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_tenants::Migration),
            Box::new(m20250101_000002_create_users::Migration),
            Box::new(m20250101_000003_create_tenant_modules::Migration),
            Box::new(m20250130_000004_create_tenant_locales::Migration),
            Box::new(m20250130_000005_create_nodes::Migration),
            Box::new(m20250130_000006_create_categories::Migration),
            Box::new(m20250130_000007_create_tags::Migration),
            Box::new(m20250130_000008_create_meta::Migration),
            Box::new(m20250130_000009_create_media::Migration),
            Box::new(m20250101_000004_create_sessions::Migration),
            Box::new(m20250101_000005_create_roles_and_permissions::Migration),
            Box::new(m20250101_000006_add_metadata_to_tenants_and_users::Migration),
        ]
    }
}

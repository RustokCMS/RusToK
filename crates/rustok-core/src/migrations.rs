use sea_orm_migration::MigrationTrait;

pub struct ModuleMigration {
    pub module_slug: &'static str,
    pub migrations: Vec<Box<dyn MigrationTrait>>,
}

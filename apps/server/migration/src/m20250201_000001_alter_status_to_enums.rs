use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Product Status Enum
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TYPE product_status_enum AS ENUM ('draft', 'active', 'archived');",
            )
            .await?;

        // Alter Products table
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE products 
                 ALTER COLUMN status DROP DEFAULT,
                 ALTER COLUMN status TYPE product_status_enum USING status::product_status_enum,
                 ALTER COLUMN status SET DEFAULT 'draft'::product_status_enum;",
            )
            .await?;

        // Create Content Status Enum
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TYPE content_status_enum AS ENUM ('draft', 'published', 'archived');",
            )
            .await?;

        // Alter Nodes table
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE nodes 
                 ALTER COLUMN status DROP DEFAULT,
                 ALTER COLUMN status TYPE content_status_enum USING status::content_status_enum,
                 ALTER COLUMN status SET DEFAULT 'draft'::content_status_enum;",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert Nodes table
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE nodes 
                 ALTER COLUMN status DROP DEFAULT,
                 ALTER COLUMN status TYPE varchar(32) USING status::varchar,
                 ALTER COLUMN status SET DEFAULT 'draft';",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TYPE content_status_enum;")
            .await?;

        // Revert Products table
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE products 
                 ALTER COLUMN status DROP DEFAULT,
                 ALTER COLUMN status TYPE varchar(32) USING status::varchar,
                 ALTER COLUMN status SET DEFAULT 'draft';",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TYPE product_status_enum;")
            .await?;

        Ok(())
    }
}

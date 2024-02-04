use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        const SQL: &str = include_str!("../../sql/init-sql/setup.sql");

        let stmt = Statement::from_string(manager.get_database_backend(), SQL);

        manager.get_connection().execute(stmt).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        const SQL: &str = include_str!("../../sql/init-sql/rollback.sql");

        let stmt = Statement::from_string(manager.get_database_backend(), SQL);

        manager.get_connection().execute(stmt).await?;
        Ok(())
    }
}

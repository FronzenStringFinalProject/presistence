use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Parent::Table)
                    .col(
                        ColumnDef::new(Parent::Pid)
                            .auto_increment()
                            .integer()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Parent::Name).not_null().string_len(255))
                    .take(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Parent::Table).take())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Parent {
    Table,
    Pid,
    Name,
}

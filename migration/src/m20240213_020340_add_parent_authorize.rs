use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_parent_table::Parent;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Parent::Table)
                    .add_column(
                        ColumnDef::new(LocalParent::UniqueId)
                            .unique_key()
                            .not_null()
                            .string_len(255)
                            .default("INNER_PARENT"),
                    )
                    .add_column(
                        ColumnDef::new(LocalParent::Password)
                            .not_null()
                            .string_len(255)
                            .default("INNER_PWD"),
                    )
                    .add_column(
                        ColumnDef::new(LocalParent::Secret)
                            .not_null()
                            .string_len(255)
                            .default("INNER_SECRET"),
                    )
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Parent::Table)
                    .drop_column(LocalParent::UniqueId)
                    .drop_column(LocalParent::Password)
                    .drop_column(LocalParent::Secret)
                    .take(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum LocalParent {
    UniqueId,
    Password,
    Secret,
}

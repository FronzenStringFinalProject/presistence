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
                        ColumnDef::new(LocalParent::PwdVer)
                            .integer()
                            .default(0)
                            .not_null(),
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
                    .drop_column(LocalParent::PwdVer)
                    .take(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum LocalParent {
    PwdVer,
}

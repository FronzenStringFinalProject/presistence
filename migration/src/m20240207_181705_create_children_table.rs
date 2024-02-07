use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_parent_table::Parent;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Children::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Children::Cid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Children::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Children::Ability)
                            .double()
                            .not_null()
                            .default(0.0),
                    )
                    .col(ColumnDef::new(Children::Parent).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .to(Parent::Table, Parent::Pid)
                            .from(Children::Table, Children::Parent),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Children::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Children {
    Table,
    Cid,
    Name,
    Parent,
    Ability,
}

use crate::m20240207_181705_create_children_table::Children;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChildCheck::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChildCheck::Cid).integer().not_null())
                    .col(
                        ColumnDef::new(ChildCheck::Date)
                            .date()
                            .not_null()
                            .default(Expr::cust("NOW()")),
                    )
                    .primary_key(
                        Index::create()
                            .table(ChildCheck::Table)
                            .col(ChildCheck::Cid)
                            .col(ChildCheck::Date),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChildCheck::Table, ChildCheck::Cid)
                            .to(Children::Table, Children::Cid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChildCheck::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChildCheck {
    Table,
    Cid,
    Date,
}

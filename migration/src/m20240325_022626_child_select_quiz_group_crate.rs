use crate::m20240207_181705_create_children_table::Children;
use crate::m20240207_183220_create_quiz_table::QuizGroups;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChildQuizGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChildQuizGroup::Cid).integer().not_null())
                    .col(ColumnDef::new(ChildQuizGroup::Gid).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(ChildQuizGroup::Cid)
                            .col(ChildQuizGroup::Gid)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChildQuizGroup::Table, ChildQuizGroup::Cid)
                            .to(Children::Table, Children::Cid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChildQuizGroup::Table, ChildQuizGroup::Gid)
                            .to(QuizGroups::Table, QuizGroups::Gid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChildQuizGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChildQuizGroup {
    Table,
    Cid,
    Gid,
}

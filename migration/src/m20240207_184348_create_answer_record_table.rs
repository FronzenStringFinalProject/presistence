use sea_orm_migration::prelude::*;

use crate::{
    m20240207_181705_create_children_table::Children, m20240207_183220_create_quiz_table::Quizes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AnswerRecord::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AnswerRecord::Qid).integer().not_null())
                    .col(ColumnDef::new(AnswerRecord::Cid).integer().not_null())
                    .col(
                        ColumnDef::new(AnswerRecord::Correct)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AnswerRecord::Date)
                            .date()
                            .not_null()
                            .default(Expr::cust("NOW()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AnswerRecord::Table, AnswerRecord::Cid)
                            .to(Children::Table, Children::Cid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AnswerRecord::Table, AnswerRecord::Qid)
                            .to(Quizes::Table, Quizes::Qid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(AnswerRecord::Cid)
                            .col(AnswerRecord::Qid)
                            .col(AnswerRecord::Date),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AnswerRecord::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AnswerRecord {
    Table,
    Qid,
    Cid,
    Correct,
    Date,
}

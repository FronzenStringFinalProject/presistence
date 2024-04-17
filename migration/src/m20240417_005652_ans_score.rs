use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(AnswerRecord::Table)
                    .add_column(
                        ColumnDef::new(AnswerRecord::QuizScore)
                            .integer()
                            .not_null()
                            .default(100),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(AnswerRecord::Table)
                    .drop_column(AnswerRecord::QuizScore)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AnswerRecord {
    Table,
    QuizScore,
}

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
                    .table(QuizGroups::Table)
                    .col(
                        ColumnDef::new(QuizGroups::Gid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(QuizGroups::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Quizes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quizes::Qid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Quizes::Quiz).string_len(255).not_null())
                    .col(ColumnDef::new(Quizes::Answer).integer().not_null())
                    .col(ColumnDef::new(Quizes::Group).integer().not_null())
                    .col(
                        ColumnDef::new(Quizes::Diff)
                            .double()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Quizes::Disc)
                            .double()
                            .not_null()
                            .default(1.0),
                    )
                    .col(
                        ColumnDef::new(Quizes::Lambda)
                            .double()
                            .not_null()
                            .default(0.0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Quizes::Table, Quizes::Group)
                            .to(QuizGroups::Table, QuizGroups::Gid),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quizes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuizGroups::Table).take())
            .await
    }
}
#[derive(DeriveIden)]
pub enum QuizGroups {
    Table,
    Gid,
    Name,
}

#[derive(DeriveIden)]
pub enum Quizes {
    Table,
    Qid,
    Quiz,
    Answer,
    Group,
    Diff,
    Disc,
    Lambda,
}

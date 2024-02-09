use crate::entities::{answer_record, children, prelude::*, quizes};
use sea_orm::{
    sea_query::Expr, DbErr, EntityTrait, FromQueryResult, QuerySelect, RelationTrait, StreamTrait,
};

pub struct QuizRecord;

#[derive(Debug, FromQueryResult)]
pub struct ChildQuizAns {
    pub diff: f64,
    pub quiz: String,
    pub ans: i32,
    pub disc: f64,
    pub lambdas: f64,
    pub correct: bool,
    pub ability: f64,
    pub pred: f64,
}

impl QuizRecord {
    pub async fn get_ans_quiz_by_child_id(
        conn: &(impl StreamTrait + sea_orm::ConnectionTrait),
        child_id: i32,
    ) -> Result<Vec<ChildQuizAns>, DbErr> {
        let child = Children::find_by_id(child_id)
            .select_only()
            .column(children::Column::Ability)
            .join(
                sea_orm::JoinType::Join,
                children::Relation::AnswerRecord.def(),
            )
            .join(sea_orm::JoinType::Join, answer_record::Relation::Quizes.def())
            .columns([answer_record::Column::Correct])
            .columns([
                quizes::Column::Diff,
                quizes::Column::Disc,
                quizes::Column::Lambda,
                quizes::Column::Quiz,
                quizes::Column::Answer,
            ])
            .column_as(
                Expr::col(quizes::Column::Lambda).add(
                    Expr::expr(Expr::val(1).sub(Expr::col(quizes::Column::Lambda))).div(
                        Expr::val(1).add(Expr::cust_with_expr(
                            "exp($1)",
                            Expr::val(-1.702).mul(Expr::col(quizes::Column::Disc)).mul(
                                Expr::col(children::Column::Ability)
                                    .sub(Expr::col(quizes::Column::Diff)),
                            ),
                        )),
                    ),
                ),
                "pred",
            )
            .into_model::<ChildQuizAns>()
            .all(conn)
            .await?;

        Ok(child)
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{
        sea_query::Expr, ConnectOptions, Database, EntityTrait, QuerySelect, QueryTrait,
        RelationTrait,
    };

    use crate::entities::{answer_record, children, prelude::*, quizes};

    use super::QuizRecord;
    #[test]
    fn test_sql() {
        let sql = Children::find_by_id(1)
            .select_only()
            .column(children::Column::Ability)
            .join(
                sea_orm::JoinType::Join,
                children::Relation::AnswerRecord.def(),
            )
            .join(sea_orm::JoinType::Join, answer_record::Relation::Quizes.def())
            .columns([answer_record::Column::Correct])
            .columns([
                quizes::Column::Diff,
                quizes::Column::Disc,
                quizes::Column::Lambda,
                quizes::Column::Quiz,
                quizes::Column::Answer,
            ])
            .column_as(
                Expr::col(quizes::Column::Lambda).add(
                    Expr::expr(Expr::val(1).sub(Expr::col(quizes::Column::Lambda))).div(
                        Expr::val(1).add(Expr::cust_with_expr(
                            "exp($1)",
                            Expr::val(-1.702).mul(Expr::col(quizes::Column::Disc)).mul(
                                Expr::col(children::Column::Ability)
                                    .sub(Expr::col(quizes::Column::Diff)),
                            ),
                        )),
                    ),
                ),
                "pred",
            )
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        println!("{sql}")
    }
    use tokio;

    #[tokio::test]
    async fn test_query() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/mydb",
        ))
        .await
        .expect("cannot connect Db");

        let ret = QuizRecord::get_ans_quiz_by_child_id(&conn, 2)
            .await
            .expect("Query Error");

        println!("{ret:?}")
    }
}

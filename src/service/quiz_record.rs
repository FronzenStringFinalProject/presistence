use crate::entities::{ans_records, children, prelude::*, quiz};
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
                children::Relation::AnsRecords.def(),
            )
            .join(sea_orm::JoinType::Join, ans_records::Relation::Quiz.def())
            .columns([ans_records::Column::Correct])
            .columns([
                quiz::Column::Diff,
                quiz::Column::Disc,
                quiz::Column::Lambdas,
                quiz::Column::Quiz,
                quiz::Column::Ans,
            ])
            .column_as(
                Expr::col(quiz::Column::Lambdas).add(
                    Expr::expr(Expr::val(1).sub(Expr::col(quiz::Column::Lambdas))).div(
                        Expr::val(1).add(Expr::cust_with_expr(
                            "exp($1)",
                            Expr::val(-1.702).mul(Expr::col(quiz::Column::Disc)).mul(
                                Expr::col(children::Column::Ability)
                                    .sub(Expr::col(quiz::Column::Diff)),
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

    use crate::entities::{ans_records, children, prelude::*, quiz};

    use super::QuizRecord;
    #[test]
    fn test_sql() {
        let sql = Children::find_by_id(1)
            .select_only()
            .column(children::Column::Ability)
            .join(
                sea_orm::JoinType::Join,
                children::Relation::AnsRecords.def(),
            )
            .join(sea_orm::JoinType::Join, ans_records::Relation::Quiz.def())
            .columns([ans_records::Column::Correct])
            .columns([
                quiz::Column::Diff,
                quiz::Column::Disc,
                quiz::Column::Lambdas,
                quiz::Column::Quiz,
                quiz::Column::Ans,
            ])
            .column_as(
                Expr::col(quiz::Column::Lambdas).add(
                    Expr::expr(Expr::val(1).sub(Expr::col(quiz::Column::Lambdas))).div(
                        Expr::val(1).add(Expr::cust_with_expr(
                            "exp($1)",
                            Expr::val(-1.702).mul(Expr::col(quiz::Column::Disc)).mul(
                                Expr::col(children::Column::Ability)
                                    .sub(Expr::col(quiz::Column::Diff)),
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

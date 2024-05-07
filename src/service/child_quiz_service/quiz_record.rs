use super::ChildQuizService;
use crate::entities::{answer_record, children, prelude::*, quizes};
use crate::service::DatabaseServiceTrait;
use sea_orm::{
    sea_query::Expr, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, QuerySelect,
    RelationTrait,
};

#[derive(Debug, FromQueryResult)]
pub struct ChildQuizAns {
    pub diff: f64,
    pub quiz: String,
    pub answer: i32,
    pub disc: f64,
    pub lambda: f64,
    pub correct: bool,
    pub ability: f64,
    pub pred: f64,
}

impl<D: ConnectionTrait> ChildQuizService<D> {
    pub async fn get_ans_quiz_by_child_id(
        &self,
        child_id: i32,
        number: u64,
    ) -> Result<Vec<ChildQuizAns>, DbErr> {
        let child = Children::find_by_id(child_id)
            .select_only()
            .column(children::Column::Ability)
            .join(
                sea_orm::JoinType::Join,
                children::Relation::AnswerRecord.def(),
            )
            .join(
                sea_orm::JoinType::Join,
                answer_record::Relation::Quizes.def(),
            )
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
            .limit(number)
            .into_model::<ChildQuizAns>()
            .all(self.db())
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

    use crate::{
        entities::{answer_record, children, prelude::*, quizes},
        service::ChildQuizService,
    };

    #[test]
    fn test_sql() {
        let sql = Children::find_by_id(501)
            .select_only()
            .column(children::Column::Ability)
            .join(
                sea_orm::JoinType::Join,
                children::Relation::AnswerRecord.def(),
            )
            .join(
                sea_orm::JoinType::Join,
                answer_record::Relation::Quizes.def(),
            )
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
            .limit(1)
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        println!("{sql}")
    }
    use crate::service::DatabaseServiceTrait;
    use tokio;

    #[tokio::test]
    async fn test_query() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/mydb",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildQuizService::with_db(conn)
            .get_ans_quiz_by_child_id(2, 25)
            .await
            .expect("Query Error");

        println!("{ret:?}")
    }
}

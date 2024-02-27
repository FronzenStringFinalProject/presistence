use crate::service::DatabaseServiceTrait;
use crate::{entities::children::Entity, utils::predict_correct::predict_correct_expr};
use sea_orm::{
    sea_query::Expr, Condition, DbErr, DerivePartialModel, EntityTrait, FromQueryResult,
    QueryFilter, TransactionTrait,
};
use sea_orm::{Order, QueryOrder};
use typed_builder::TypedBuilder;

use super::ChildQuizService;

#[derive(Debug, TypedBuilder)]
pub struct QuizFetched {
    pub id: i32,
    pub quiz: String,
}

impl<D: TransactionTrait> ChildQuizService<D> {
    pub async fn next_quiz(
        &self,
        child_id: i32,
        min_correct: f64,
        max_correct: f64,
    ) -> Result<Option<QuizFetched>, DbErr> {
        let ctx = self.db().begin().await?;

        // get ability of child
        #[derive(Debug, FromQueryResult, DerivePartialModel)]
        #[sea_orm(entity = "Entity")]
        struct ChildAbility {
            ability: f64,
        }

        let child = Entity::find_by_id(child_id)
            .into_partial_model::<ChildAbility>()
            .one(&ctx)
            .await?
            .map(|ChildAbility { ability }| ability);

        let Some(ability) = child else {
            return Ok(None);
        };

        // get the quiz in the ability
        use crate::entities::quizes;
        mod local_quiz {
            use crate::entities::quizes::Entity;
            use sea_orm::DerivePartialModel;
            use sea_orm::FromQueryResult;
            #[derive(Debug, FromQueryResult, DerivePartialModel)]
            #[sea_orm(entity = "Entity")]
            pub(super) struct Quiz {
                pub(super) qid: i32,
                pub(super) quiz: String,
            }
        }

        let ret = quizes::Entity::find()
            .filter(
                Condition::all()
                    .add(
                        Expr::expr(predict_correct_expr(Expr::val(ability)))
                            .gte(Expr::val(min_correct)),
                    )
                    .add(
                        Expr::expr(predict_correct_expr(Expr::val(ability)))
                            .lte(Expr::val(max_correct)),
                    ), // .add(
                       //     Expr::col(quizes::Column::Qid).not_in_subquery(
                       //         Query::select()
                       //             .column(answer_record::Column::Qid)
                       //             .from(answer_record::Entity)
                       //             .and_where(answer_record::Column::Cid.eq(child_id))
                       //             .take(),
                       //     ),
                       // ),
            )
            .order_by(Expr::cust("random()"), Order::Asc)
            .into_partial_model::<local_quiz::Quiz>()
            .one(&ctx)
            .await?;

        let Some(local_quiz::Quiz { qid, quiz }) = ret else {
            return Ok(None);
        };
        ctx.commit().await?;
        Ok(Some(QuizFetched::builder().id(qid).quiz(quiz).build()))
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{ConnectOptions, Database};

    use crate::service::{ChildQuizService, DatabaseServiceTrait};

    #[tokio::test]
    async fn test_get_quiz() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let quiz = ChildQuizService::with_db(conn)
            .next_quiz(22, 0.2, 1.0)
            .await
            .unwrap()
            .unwrap();
        println!("{quiz:?}")
    }
}

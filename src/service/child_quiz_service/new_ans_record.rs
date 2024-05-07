use sea_orm::sea_query::{Alias, Query};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, JoinType,
    PaginatorTrait, QueryFilter, Set, StatementBuilder, TransactionTrait,
};

use crate::entities::{answer_record, children, quizes};
use crate::service::DatabaseServiceTrait;
use crate::utils::predict_correct::score_update;

use super::ChildQuizService;

#[derive(Debug, FromQueryResult)]
struct GetCurrentScore {
    score: i32,
}

impl<D: TransactionTrait> ChildQuizService<D> {
    pub async fn new_ans_record(
        &self,
        child_id: i32,
        quiz_id: i32,
        quiz_ans: Option<i32>,
    ) -> Result<bool, DbErr> {
        let ctx = self.db().begin().await?;
        let correct = if let Some(ans) = quiz_ans {
            // check ans correct
            quizes::Entity::find_by_id(quiz_id)
                .filter(quizes::Column::Answer.eq(ans))
                .count(&ctx)
                .await?
                > 0
        } else {
            false
        };

        // score
        let score = Query::select()
            .expr_as(score_update(false), Alias::new("score"))
            .from(children::Entity)
            .join(
                JoinType::Join,
                quizes::Entity,
                Condition::all()
                    .add(children::Column::Cid.eq(child_id))
                    .add(quizes::Column::Qid.eq(quiz_id)),
            )
            .to_owned();

        let stat = StatementBuilder::build(&score, &ctx.get_database_backend());
        let Some(result) = ctx.query_one(stat).await? else {
            return Ok(false);
        };
        let score = GetCurrentScore::from_query_result(&result, "")?.score;

        // write
        let active = answer_record::ActiveModel {
            qid: Set(quiz_id),
            cid: Set(child_id),
            correct: Set(correct),
            quiz_score: Set(if correct { score } else { 0 }),
            ..Default::default()
        };
        answer_record::Entity::insert(active).exec(&ctx).await?;
        ctx.commit().await?;
        Ok(correct)
    }
}

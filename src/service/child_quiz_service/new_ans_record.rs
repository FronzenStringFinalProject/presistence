use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set, TransactionTrait,
};

use crate::entities::{answer_record, quizes};
use crate::service::DatabaseServiceTrait;

use super::ChildQuizService;

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

        // write
        let active = answer_record::ActiveModel {
            qid: Set(quiz_id),
            cid: Set(child_id),
            correct: Set(correct),
            ..Default::default()
        };
        answer_record::Entity::insert(active).exec(&ctx).await?;
        ctx.commit().await?;
        Ok(correct)
    }
}

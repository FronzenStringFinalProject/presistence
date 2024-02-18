use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::entities::{answer_record, quizes};

use super::ChildQuizService;

impl ChildQuizService {
    pub async fn new_ans_record(
        db: &impl TransactionTrait,
        child_id: i32,
        quiz_id: i32,
        quiz_ans: i32,
    ) -> Result<bool, DbErr> {
        let ctx = db.begin().await?;

        // check ans correct
        let correct = quizes::Entity::find_by_id(quiz_id)
            .filter(quizes::Column::Answer.eq(quiz_ans))
            .count(&ctx)
            .await?
            > 0;

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

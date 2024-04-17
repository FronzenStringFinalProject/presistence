use crate::sea_orm::prelude::Date;
use crate::sea_orm::ActiveValue::Set;
use crate::sea_orm::{
    ActiveModelTrait, DerivePartialModel, EntityTrait, FromQueryResult, QuerySelect, RelationTrait,
};
use presistence::entities::{answer_record, children};
use presistence::utils::predict_correct::score_update;
use sea_orm_migration::prelude::*;

#[derive(Debug, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "answer_record::Entity")]
struct UpdateScore {
    cid: i32,
    qid: i32,
    date: Date,
    #[sea_orm(from_expr = "score_update(true)")]
    new_score: i32,
}

impl UpdateScore {
    fn into_active_model(self) -> answer_record::ActiveModel {
        answer_record::ActiveModel {
            qid: Set(self.qid),
            cid: Set(self.cid),
            date: Set(self.date),
            quiz_score: Set(self.new_score),
            ..Default::default()
        }
    }
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let all = children::Entity::find()
            .join(JoinType::Join, children::Relation::AnswerRecord.def())
            .join(JoinType::Join, answer_record::Relation::Quizes.def())
            .into_partial_model::<UpdateScore>()
            .all(manager.get_connection())
            .await?;

        for update_score in all {
            update_score
                .into_active_model()
                .save(manager.get_connection())
                .await?;
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

use super::model;
use super::Update;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};

impl Update {
    pub async fn update_quiz_params(
        &self,
        db: &impl ConnectionTrait,
        quiz_id: i32,
        diff: f64,
        disc: f64,
        lambda: f64,
    ) -> Result<(), DbErr> {
        let activate = model::ActiveModel {
            qid: Set(quiz_id),
            diff: Set(diff),
            disc: Set(disc),
            lambda: Set(lambda),
            ..Default::default()
        };

        activate.save(db).await?;
        Ok(())
    }
}

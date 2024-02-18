use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Set};

impl super::Update {
    pub async fn ability(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
        ability: f64,
    ) -> Result<(), DbErr> {
        let active = super::model::ActiveModel {
            cid: Set(child_id),
            ability: Set(ability),
            ..Default::default()
        };
        super::model::Entity::update(active).exec(db).await?;
        Ok(())
    }

    pub async fn name(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
        name: String,
    ) -> Result<(), DbErr> {
        let active = super::model::ActiveModel {
            cid: Set(child_id),
            name: Set(name),
            ..Default::default()
        };
        super::model::Entity::update(active).exec(db).await?;
        Ok(())
    }
}

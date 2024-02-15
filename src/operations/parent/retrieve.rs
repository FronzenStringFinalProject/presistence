use super::model;
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

impl super::Retrieve {
    pub async fn by_id(db: &impl ConnectionTrait, id: i32) -> Result<Option<model::Model>, DbErr> {
        let data = model::Entity::find_by_id(id).one(db).await?;
        Ok(data)
    }

    pub async fn by_id_and_pwd_version(
        db: &impl ConnectionTrait,
        id: i32,
        pwd_version: i32,
    ) -> Result<Option<model::Model>, DbErr> {
        model::Entity::find_by_id(id)
            .filter(model::Column::PwdVer.eq(pwd_version))
            .one(db)
            .await
    }
}

use crate::sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use presistence::entities::parent as model;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let all_parent = model::Entity::find().all(manager.get_connection()).await?;
        let hasher = Argon2::default();
        for parent in all_parent {
            let salt = SaltString::generate(&mut OsRng);
            let hashed_pwd = hasher
                .hash_password(parent.password.as_bytes(), &salt)
                .expect("Failure To Encoder Pwd");
            let hashed_secret = hasher
                .hash_password(parent.secret.as_bytes(), &salt)
                .expect("Failure to Encode Secret");

            let activate = model::ActiveModel {
                password: Set(hashed_pwd.to_string()),
                secret: Set(hashed_secret.to_string()),
                ..parent.into_active_model()
            };

            activate.save(manager.get_connection()).await?;
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        // 对密码加密不可回滚
        Ok(())
    }
}

use crate::sea_orm::ActiveValue::Set;
use crate::sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter};
use presistence::entities::quizes::{ActiveModel, Column, Entity};
use presistence::sea_orm::DerivePartialModel;
use sea_orm_migration::prelude::*;
use std::collections::HashMap;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let ret = Entity::find()
            .filter(Column::Quiz.like("%��%"))
            .into_partial_model::<FixQuiz>()
            .all(manager.get_connection())
            .await?;

        let map = generate_map_dict();

        for FixQuiz { qid, key_quiz } in ret {
            if let Some(fixed) = map.get(&key_quiz) {
                let active = ActiveModel {
                    qid: Set(qid),
                    quiz: Set(fixed.clone()),
                    ..Default::default()
                };
                Entity::update(active)
                    .exec(manager.get_connection())
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

fn generate_map_dict() -> HashMap<String, String> {
    include_str!("../../data/questions.txt")
        .lines()
        .filter(|line| line.contains('÷') || line.contains('×'))
        .map(|line| line.split('=').next().unwrap().trim())
        .map(|str| {
            (
                str.replace(['÷', '×'], "@"),
                str.replace('×', "*").replace('÷', "/"),
            )
        })
        .collect::<HashMap<_, _>>()
}

#[test]
fn test() {
    println!("{:?}", generate_map_dict())
}

#[derive(Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Entity")]
struct FixQuiz {
    qid: i32,
    #[sea_orm(from_expr = "replace_error_char()")]
    key_quiz: String,
}

fn replace_error_char() -> SimpleExpr {
    Expr::cust_with_expr("REPLACE($1, '��','@')", Expr::col(Column::Quiz))
}

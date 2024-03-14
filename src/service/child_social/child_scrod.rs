use crate::entities::children;
use crate::service::child_social::ChildSocialService;
use crate::service::DatabaseServiceTrait;
use sea_orm::sea_query::{Asterisk, Expr};
use sea_orm::{
    DbErr, DerivePartialModel, EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait,
};

#[derive(Debug, FromQueryResult, DerivePartialModel)]
pub struct ChildScore {
    #[sea_orm(from_expr = "Expr::col(Asterisk).count().mul(Expr::value(100))")]
    pub total_score: i64,
    #[sea_orm(from_expr = "Expr::col(Asterisk).count().mul(Expr::value(100))\
        .div(Expr::val(1000))")]
    pub current_level: i64,
    #[sea_orm(
        from_expr = "Expr::expr(Expr::col(Asterisk).count().mul(Expr::value(100)))\
    .modulo(Expr::val(1000))"
    )]
    pub current_level_score: i64,
}

impl ChildSocialService {
    pub async fn get_child_score(&self, child_id: i32) -> Result<Option<ChildScore>, DbErr> {
        children::Entity::find_by_id(child_id)
            .join(JoinType::Join, children::Relation::AnswerRecord.def())
            .group_by(children::Column::Cid)
            .into_partial_model::<ChildScore>()
            .one(self.db())
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::service::child_social::ChildSocialService;
    use crate::service::DatabaseServiceTrait;
    use sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test_query() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildSocialService::with_db(conn)
            .get_child_score(2)
            .await
            .expect("Query Error");

        println!("{ret:?}")
    }
}

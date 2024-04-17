use sea_orm::sea_query::Expr;
use sea_orm::{
    ColumnTrait, DbErr, DerivePartialModel, EntityTrait, FromQueryResult, JoinType, QuerySelect,
    RelationTrait,
};

use crate::entities::{answer_record, children};
use crate::service::child_social::ChildSocialService;
use crate::service::DatabaseServiceTrait;

#[derive(Debug, FromQueryResult, DerivePartialModel, Default)]
pub struct ChildScore {
    #[sea_orm(from_expr = "answer_record::Column::QuizScore.sum()")]
    pub total_score: i64,
    #[sea_orm(from_expr = "answer_record::Column::QuizScore.sum().div(Expr::val(1000))")]
    pub current_level: i64,
    #[sea_orm(
        from_expr = "Expr::expr(answer_record::Column::QuizScore.sum()).modulo(Expr::val(1000))"
    )]
    pub current_level_score: i64,
}

impl ChildSocialService {
    pub async fn get_child_score(&self, child_id: i32) -> Result<ChildScore, DbErr> {
        children::Entity::find_by_id(child_id)
            .join(JoinType::Join, children::Relation::AnswerRecord.def())
            .group_by(children::Column::Cid)
            .into_partial_model::<ChildScore>()
            .one(self.db())
            .await
            .map(Option::unwrap_or_default)
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{ConnectOptions, Database};

    use crate::service::child_social::ChildSocialService;
    use crate::service::DatabaseServiceTrait;

    #[tokio::test]
    async fn test_query() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildSocialService::with_db(conn)
            .get_child_score(501)
            .await
            .expect("Query Error");

        println!("{ret:?}")
    }
}

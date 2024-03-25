use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, QueryTrait,
};
use serde::Serialize;

use crate::entities::{child_quiz_group, quiz_groups};
use crate::service::DatabaseServiceTrait;

#[derive(FromQueryResult, Debug, Serialize)]
pub struct QuizGroupItem {
    pub gid: i32,
    pub name: String,
    pub select: bool,
}

impl super::ChildQuizService {
    pub async fn fetch_all_quiz_group(&self, child_id: i32) -> Result<Vec<QuizGroupItem>, DbErr> {
        let select_child_gid = child_quiz_group::Entity::find()
            .select_only()
            .filter(child_quiz_group::Column::Cid.eq(child_id))
            .column(child_quiz_group::Column::Gid)
            .into_query();

        quiz_groups::Entity::find()
            .column_as(
                quiz_groups::Column::Gid.in_subquery(select_child_gid),
                "select",
            )
            .into_model::<QuizGroupItem>()
            .all(self.db())
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::service::{ChildQuizService, DatabaseServiceTrait};
    use sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test_get_selects() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let a = ChildQuizService::with_db(conn)
            .fetch_all_quiz_group(501)
            .await
            .unwrap();
        println!("{a:?}")
    }
}

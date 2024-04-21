use super::model;
use crate::output_models::answer_record::AnsRecordItem;
use sea_orm::prelude::Date;
use sea_orm::sea_query::{Asterisk, Expr, SimpleExpr};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, DerivePartialModel, EntityTrait,
    FromQueryResult, QueryFilter, QuerySelect,
};
use std::collections::HashMap;

#[derive(Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "model::Entity")]
struct LastYearRecordItem {
    date: Date,
    #[sea_orm(from_expr = "count()")]
    number: i64,
}

fn count() -> SimpleExpr {
    Expr::col(Asterisk).count()
}

impl super::Retrieve {
    pub async fn last_year_record(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
    ) -> Result<HashMap<Date, i64>, DbErr> {
        let ret = model::Entity::find()
            .filter(
                Condition::all().add(model::Column::Cid.eq(child_id)).add(
                    model::Column::Date
                        .into_expr()
                        // 373 = 366  + 7 可以确保热力图完整
                        .gte(Expr::cust("current_date - 373")),
                ),
            )
            .group_by(model::Column::Date)
            .into_partial_model::<LastYearRecordItem>()
            .all(db)
            .await?;

        Ok(ret
            .into_iter()
            .map(|LastYearRecordItem { date, number }| (date, number))
            .collect())
    }

    pub async fn all_child_records(
        &self,
        db: &impl ConnectionTrait,
    ) -> Result<Vec<AnsRecordItem>, DbErr> {
        model::Entity::find()
            .group_by(model::Column::Cid)
            .into_partial_model()
            .all(db)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::operations::{AnswerRecordOperate, OperateTrait};
    use sea_orm::{ConnectOptions, Database};
    #[tokio::test]
    async fn test_get_all_child_record() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let output = AnswerRecordOperate
            .retrieve()
            .all_child_records(&conn)
            .await
            .expect("Db Error");

        println!("{}", output.len());
        println!("{}", output[0].answered_quiz.len())
    }
}

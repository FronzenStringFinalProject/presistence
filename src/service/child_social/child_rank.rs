use sea_orm::sea_query::{Expr, PostgresQueryBuilder, Query, SelectStatement};
use sea_orm::{
    ConnectionTrait, DbErr, DeriveIden, EntityTrait, FromQueryResult, Iden, IntoSimpleExpr,
    QueryFilter, QuerySelect, QueryTrait, StatementBuilder,
};
use serde::Serialize;

use crate::entities::{answer_record, child_check};
use crate::service::DatabaseServiceTrait;

#[derive(DeriveIden)]
enum RankTmpTable {
    Table,
    Cid,
    Rank,
    RankRange,
}

#[derive(Debug, FromQueryResult, Serialize)]
pub struct ChildRank {
    pub rank: i64,
    pub rank_range: f64,
}

impl super::ChildSocialService {
    pub async fn score_rank(&self, child_id: i32) -> Result<Option<ChildRank>, DbErr> {
        let sub_query = answer_record::Entity::find()
            .select_only()
            .column(answer_record::Column::Cid)
            .column_as(
                Expr::cust("RANK() OVER(ORDER BY COUNT(*) DESC)"),
                RankTmpTable::Rank.to_string(),
            )
            .column_as(
                Expr::cust("1 - PERCENT_RANK() OVER(ORDER BY COUNT(*) DESC)"),
                RankTmpTable::RankRange.to_string(),
            )
            .filter(answer_record::Column::Correct.into_simple_expr())
            .group_by(answer_record::Column::Cid)
            .into_query();

        let stmt = Query::select()
            .columns([RankTmpTable::Rank, RankTmpTable::RankRange])
            .from_subquery(sub_query, RankTmpTable::Table)
            .and_where(Expr::col(RankTmpTable::Cid).eq(child_id))
            .to_owned();

        println!("{}", stmt.to_string(PostgresQueryBuilder));

        let ret = self
            .db()
            .query_one(<SelectStatement as StatementBuilder>::build(
                &stmt,
                &self.db().get_database_backend(),
            ))
            .await?;

        ret.map(|query| ChildRank::from_query_result(&query, ""))
            .transpose()
    }

    pub async fn check_rank(&self, child_id: i32) -> Result<Option<ChildRank>, DbErr> {
        let sub_query = child_check::Entity::find()
            .select_only()
            .column(child_check::Column::Cid)
            .column_as(
                Expr::cust("RANK() OVER(ORDER BY COUNT(*) DESC)"),
                RankTmpTable::Rank.to_string(),
            )
            .column_as(
                Expr::cust("1 - PERCENT_RANK() OVER(ORDER BY COUNT(*) DESC)"),
                RankTmpTable::RankRange.to_string(),
            )
            .group_by(child_check::Column::Cid)
            .into_query();

        let stmt = Query::select()
            .columns([RankTmpTable::Rank, RankTmpTable::RankRange])
            .from_subquery(sub_query, RankTmpTable::Table)
            .and_where(Expr::col(RankTmpTable::Cid).eq(child_id))
            .to_owned();

        println!("{}", stmt.to_string(PostgresQueryBuilder));

        let ret = self
            .db()
            .query_one(<SelectStatement as StatementBuilder>::build(
                &stmt,
                &self.db().get_database_backend(),
            ))
            .await?;

        ret.map(|query| ChildRank::from_query_result(&query, ""))
            .transpose()
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{ConnectOptions, Database};

    use crate::service::{ChildSocialService, DatabaseServiceTrait};

    #[tokio::test]
    async fn test_get_rank() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let a = ChildSocialService::with_db(conn)
            .check_rank(441)
            .await
            .expect("ERR");

        println!("{a:?}")
    }

    #[tokio::test]
    async fn test_check_rank() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let a = ChildSocialService::with_db(conn)
            .score_rank(441)
            .await
            .expect("ERR");

        println!("{a:?}")
    }
}

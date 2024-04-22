use sea_orm::sea_query::{Asterisk, Expr, Query, SelectStatement, SimpleExpr};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbBackend, DbErr, DeriveIden, EntityTrait,
    FromQueryResult, JoinType, QuerySelect, QueryTrait, Related, Statement, StatementBuilder,
};
use serde::Serialize;

use crate::entities::{answer_record, child_check, children};
use crate::service::DatabaseServiceTrait;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct ChildRank {
    pub cid: i32,
    pub name: String,
    pub rank: i64,
    pub rank_range: f64,
    pub value: i64,
    pub is_child: bool,
}

impl super::ChildSocialService {
    pub async fn score_rank(&self, child_id: i32) -> Result<Vec<ChildRank>, DbErr> {
        let stmt = rank_query(
            answer_record::Column::QuizScore.sum(),
            answer_record::Entity,
            self.db().get_database_backend(),
            child_id,
            20,
        );

        let result = self.db().query_all(stmt).await?;
        let mut ret = Vec::with_capacity(result.len());

        for rank in result {
            let rank = ChildRank::from_query_result(&rank, "")?;
            ret.push(rank);
        }

        Ok(ret)
    }
    /// 总榜单 一页
    /// 当前排名
    pub async fn check_rank(&self, child_id: i32) -> Result<Vec<ChildRank>, DbErr> {
        let stmt = rank_query(
            Expr::col(Asterisk).count(),
            child_check::Entity,
            self.db().get_database_backend(),
            child_id,
            20,
        );

        let result = self.db().query_all(stmt).await?;
        let mut ret = Vec::with_capacity(result.len());

        for rank in result {
            let rank = ChildRank::from_query_result(&rank, "")?;
            ret.push(rank);
        }

        Ok(ret)
    }
}
#[derive(DeriveIden)]
enum RankTmpTable {
    Table,
    Cid,
    Name,
    Rank,
    RankRange,
    Value,
    IsChild,
}
fn rank_query<E>(
    order_expr: SimpleExpr,
    _: E,
    db_backend: DbBackend,
    child_id: i32,
    rank_num: i64,
) -> Statement
where
    E: Related<children::Entity>,
    E: EntityTrait,
{
    let select_stat = E::find()
        .join(JoinType::Join, <E as Related<children::Entity>>::to())
        .select_only()
        .column(children::Column::Cid)
        .column(children::Column::Name)
        .column_as(
            Expr::cust_with_expr("RANK() OVER(ORDER BY $1 DESC)", order_expr.clone()),
            "rank",
        )
        .column_as(
            Expr::cust_with_expr(
                "1 - PERCENT_RANK() OVER(ORDER BY $1 DESC)",
                order_expr.clone(),
            ),
            "rank_range",
        )
        .column_as(order_expr, "value")
        .group_by(children::Column::Cid)
        .into_query();

    let query = Query::select()
        .from_subquery(select_stat, RankTmpTable::Table)
        .columns([
            RankTmpTable::Cid,
            RankTmpTable::Name,
            RankTmpTable::Rank,
            RankTmpTable::RankRange,
            RankTmpTable::Value,
        ])
        .expr_as(
            Expr::custom_keyword(RankTmpTable::Cid).eq(child_id),
            RankTmpTable::IsChild,
        )
        .cond_where(
            Condition::any()
                .add(Expr::custom_keyword(RankTmpTable::Cid).eq(child_id))
                .add(Expr::custom_keyword(RankTmpTable::Rank).lte(Expr::value(rank_num))),
        )
        .to_owned();

    <SelectStatement as StatementBuilder>::build(&query, &db_backend)
}

#[cfg(test)]
mod test {
    use sea_orm::{ColumnTrait, ConnectOptions, Database, DatabaseBackend};

    use crate::entities::answer_record;
    use crate::service::child_social::child_rank::rank_query;
    use crate::service::{ChildSocialService, DatabaseServiceTrait};

    #[test]
    fn test_rank_sql() {
        let stat = rank_query(
            answer_record::Column::QuizScore.sum(),
            answer_record::Entity,
            DatabaseBackend::Postgres,
            501,
            20,
        );
        println!("{}", stat.to_string())
    }
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

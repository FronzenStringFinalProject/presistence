use super::{model, Retrieve};
use crate::output_models::child_check::MonthlyCheckItem;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Asterisk, Query};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, DeriveIden, EntityTrait, FromQueryResult,
    Identity, IntoSimpleExpr, Order, PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, Select,
    SelectColumns, StatementBuilder,
};

impl Retrieve {
    ///
    /// 获取连续打卡天数
    /// 参考：
    /// - https://ikddm.com/868.html/
    /// - https://www.postgresql.org/docs/current/datatype-numeric.html
    /// - https://www.postgresql.org/docs/current/functions-window.html
    pub async fn continual_check_days(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
    ) -> Result<Option<i64>, DbErr> {
        let sub_select = model::Entity::find()
            .filter(model::Column::Cid.eq(child_id))
            .select_only()
            .select_column_as(
                Expr::cust_with_exprs(
                    "$1 - CAST(ROW_NUMBER() OVER(PARTITION BY $2 ORDER BY $1) as INT4)",
                    [model::Column::Date, model::Column::Cid]
                        .into_iter()
                        .map(Expr::col)
                        .map(|expr| expr.into_simple_expr()),
                ),
                DIFF_COLUMN,
            )
            .into_query();

        let stmt = Query::select()
            .column(DiffSubSelect::Diff)
            .expr_as(Expr::col(Asterisk).count(), DiffSubSelect::ContinualDays)
            .from_subquery(sub_select, DiffSubSelect::Table)
            .group_by_col(DiffSubSelect::Diff)
            .order_by(DiffSubSelect::Diff, Order::Desc)
            .limit(1)
            .take();

        let stmt = StatementBuilder::build(&stmt, &db.get_database_backend());

        let ret = db
            .query_one(stmt)
            .await?
            .map(|result| result.try_get("", "continual_days"))
            .transpose()?;

        Ok(ret)
    }

    /// 获取指定月份的打卡情况
    ///
    /// 参考：
    /// - https://www.postgresql.org/docs/current/functions-datetime.html
    pub async fn spec_month_check(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
        month: Option<i32>,
    ) -> Result<Vec<MonthlyCheckItem>, DbErr> {
        let condition = if let Some(month) = month {
            Expr::cust_with_exprs(
                "DATE_PART('month', $1) = $2",
                [
                    model::Column::Date.into_simple_expr(),
                    Expr::val(month as f64).into_simple_expr(),
                ],
            )
        } else {
            Expr::cust_with_expr(
                "DATE_PART('month', $1) = DATE_PART('month', current_timestamp)",
                model::Column::Date.into_simple_expr(),
            )
        };

        model::Entity::find()
            .filter(
                Condition::all()
                    .add(model::Column::Cid.eq(child_id))
                    .add(condition),
            )
            .into_partial_model::<MonthlyCheckItem>()
            .all(db)
            .await
    }

    pub async fn total_check(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
    ) -> Result<u64, DbErr> {
        model::Entity::find()
            .filter(model::Column::Cid.eq(child_id))
            .count(db)
            .await
    }
}

#[derive(DeriveIden)]
enum DiffSubSelect {
    Table,
    Diff,
    ContinualDays,
}

const DIFF_COLUMN: &str = "diff";

#[cfg(test)]
mod test {
    use crate::operations::child_check::ChildCheckOperate;
    use crate::operations::OperateTrait;

    use sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test_continual_days() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let days = ChildCheckOperate
            .retrieve()
            .continual_check_days(&conn, 501)
            .await
            .expect("Error");

        assert_eq!(days, Some(3));
    }

    #[tokio::test]
    async fn test_month_record() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildCheckOperate
            .retrieve()
            .spec_month_check(&conn, 501, None)
            .await
            .expect("Error");
        assert_eq!(ret, vec![]);
        println!("{ret:?}");

        let ret = ChildCheckOperate
            .retrieve()
            .spec_month_check(&conn, 501, Some(2))
            .await
            .expect("Error");
        println!("{ret:?}")
    }
    #[tokio::test]
    async fn test_total_check_days() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildCheckOperate
            .retrieve()
            .total_check(&conn, 501)
            .await
            .expect("Error");
        println!("{ret:?}");
    }
}

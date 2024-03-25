use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Asterisk, Query};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, DeriveIden, EntityTrait, IntoSimpleExpr,
    PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, SelectColumns, StatementBuilder,
};

use crate::output_models::child_check::MonthlyCheckItem;

use super::{model, Retrieve};

impl Retrieve {
    pub async fn can_check(&self, db: &impl ConnectionTrait, child_id: i32) -> Result<bool, DbErr> {
        Ok(model::Entity::find()
            .filter(
                Condition::all()
                    .add(model::Column::Cid.eq(child_id))
                    .add(Expr::col(model::Column::Date).eq(Expr::cust("DATE(now())"))),
            )
            .count(db)
            .await?
            == 0)
    }

    ///
    /// 获取连续打卡天数
    /// 参考：
    /// - https://ikddm.com/868.html/
    /// - https://www.postgresql.org/docs/current/datatype-numeric.html
    /// - https://www.postgresql.org/docs/current/functions-window.html
    ///
    ///
    /// ```sql no-run
    /// select count(*) from (
    ///     SELECT
    ///     DATE(Now())
    ///         -date
    ///         +CAST(ROW_NUMBER() OVER(PARTITION BY cid ORDER BY date) as INT4) as ii,
    ///     CAST(ROW_NUMBER() OVER(PARTITION BY cid ORDER BY date) as INT4) as iii
    ///     FROM public.child_check where cid = 501
    /// )
    /// group by ii
    /// having ii-max(iii) =0
    /// ```
    pub async fn continual_check_days(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
    ) -> Result<i64, DbErr> {
        const DIFF_COLUMN: &str = "diff";
        const DAYS_FROM_TODAY: &str = "days_from_today";
        let sub_select = model::Entity::find()
            .filter(model::Column::Cid.eq(child_id))
            .select_only()
            .select_column_as(
                Expr::cust_with_exprs(
                    "CAST(ROW_NUMBER() OVER(PARTITION BY $2 ORDER BY $1) as INT4)",
                    [model::Column::Date, model::Column::Cid]
                        .into_iter()
                        .map(Expr::col)
                        .map(|expr| expr.into_simple_expr()),
                ),
                DIFF_COLUMN,
            )
            .select_column_as(Expr::cust_with_exprs(
                "DATE(NOW()) - $1 + CAST(ROW_NUMBER() OVER(PARTITION BY $2 ORDER BY $1) as INT4)",
                [model::Column::Date, model::Column::Cid]
                    .into_iter()
                    .map(Expr::col)
                    .map(|expr| expr.into_simple_expr()),
            ),DAYS_FROM_TODAY)
            .into_query();

        let stmt = Query::select()
            .expr_as(Expr::col(Asterisk).count(), DiffSubSelect::ContinualDays)
            .from_subquery(sub_select, DiffSubSelect::Table)
            .group_by_col(DiffSubSelect::DaysFromToday)
            .and_having(
                Expr::col(DiffSubSelect::DaysFromToday).sub(Expr::col(DiffSubSelect::Diff).max()),
            )
            .limit(1)
            .take();

        let stmt = StatementBuilder::build(&stmt, &db.get_database_backend());

        #[cfg(test)]
        println!("{:?}", stmt.to_string());

        let ret = db
            .query_one(stmt)
            .await?
            .map(|result| result.try_get("", "continual_days"))
            .transpose()?
            .unwrap_or(0);

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
        year: Option<i32>,
    ) -> Result<Vec<MonthlyCheckItem>, DbErr> {
        let month_condition = if let Some(month) = month {
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

        let year_condition = if let Some(year) = year {
            Expr::cust_with_exprs(
                "DATE_PART('year', $1) = $2",
                [
                    model::Column::Date.into_simple_expr(),
                    Expr::val(year as f64).into_simple_expr(),
                ],
            )
        } else {
            Expr::cust_with_expr(
                "DATE_PART('year', $1) = DATE_PART('year', current_timestamp)",
                model::Column::Date.into_simple_expr(),
            )
        };

        model::Entity::find()
            .filter(
                Condition::all()
                    .add(model::Column::Cid.eq(child_id))
                    .add(month_condition)
                    .add(year_condition),
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
    DaysFromToday,
    ContinualDays,
}

#[cfg(test)]
mod test {
    use sea_orm::{ConnectOptions, Database};

    use crate::operations::child_check::ChildCheckOperate;
    use crate::operations::OperateTrait;

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

        assert_eq!(days, 3);
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
            .spec_month_check(&conn, 501, None, None)
            .await
            .expect("Error");
        println!("{ret:?}");

        let ret = ChildCheckOperate
            .retrieve()
            .spec_month_check(&conn, 501, Some(2), Some(2024))
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

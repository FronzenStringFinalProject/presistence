use sea_orm::sea_query::{Expr, SimpleExpr};
use sea_orm::{ColumnTrait, IntoSimpleExpr};

use crate::entities::{answer_record, children, quizes};

pub fn predict_correct_expr(ability: Expr) -> SimpleExpr {
    Expr::col(quizes::Column::Lambda).add(
        Expr::expr(Expr::val(1).sub(Expr::col(quizes::Column::Lambda))).div(
            Expr::val(1).add(Expr::cust_with_expr(
                "exp($1)",
                Expr::val(-1.702)
                    .mul(Expr::col(quizes::Column::Disc))
                    .mul(ability.sub(Expr::col(quizes::Column::Diff))),
            )),
        ),
    )
}

///
/// ```sql no-run
/// cast(
///     round(
///     (
///         1 - (
///             lambda + (1-lambda) * 1 / (1 + exp(-1.702 * disc * (ability-diff))) ))*50 + 75
///      ) *
///         cast(correct as INT4)
///     as Int4
/// )
/// ```
pub fn score_update(with_correct: bool) -> SimpleExpr {
    Expr::cust_with_exprs(
        "CAST(ROUND((1 - $1) * 50 + 75) * $2 AS INT4)",
        [
            predict_correct_expr(children::Column::Ability.into_expr()),
            if with_correct {
                Expr::cust_with_expr(
                    "CAST($1 AS INT4)",
                    answer_record::Column::Correct.into_expr(),
                )
            } else {
                Expr::val(1).into_simple_expr()
            },
        ],
    )
}

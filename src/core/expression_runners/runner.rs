use derive_new::new;
use sqlparser::ast::Expr;
use core::fmt::Display;

use crate::core::{
    expression_runners::identifier_runner::IdentifierRunner,
    expression_runners::value_runner::ValueRunner,
     row::ValueContainer};

//NOTA: how to improve the structs import?

#[derive(new)]
#[derive(Eq)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ExpressionParserError {
    pub message: String
}

impl Display for ExpressionParserError {
    fn fmt(&self, formater: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formater, "Expression Parse Error: {}", self.message)
    }
}

#[enum_dispatch::enum_dispatch]
pub trait RunnerTrait {
    fn execute(&mut self, expression: &Expr) -> Result<ValueContainer, ExpressionParserError>;
}

#[enum_dispatch::enum_dispatch(RunnerTrait)]
pub enum Runner {
    IdentifierRunner,
    ValueRunner,
    // CompoundIdentifierRunner,
    // JsonAccessRunner,
    // CompositeAccessRunner,
    // IsFalseRunner,
    // IsNotFalseRunner,
    // IsTrueRunner,
    // IsNotTrueRunner,
    // IsNullRunner,
    // IsNotNullRunner,
    // IsUnknownRunner,
    // IsNotUnknownRunner,
    // IsDistinctFromRunner,
    // IsNotDistinctFromRunner,
    // InListRunner,
    // InSubqueryRunner,
    // InUnnestRunner,
    // BetweenRunner,
    // BinaryOpRunner,
    // LikeRunner,
    // ILikeRunner,
    // SimilarToRunner,
    // RLikeRunner,
    // AnyOpRunner,
    // AllOpRunner,
    // UnaryOpRunner,
    // ConvertRunner,
    // CastRunner,
    // TryCastRunner,
    // SafeCastRunner,
    // AtTimeZoneRunner,
    // ExtractRunner,
    // CeilRunner,
    // FloorRunner,
    // PositionRunner,
    // SubstringRunner,
    // TrimRunner,
    // OverlayRunner,
    // CollateRunner,
    // NestedRunner,
    // ValueRunner,
    // IntroducedStringRunner,
    // TypedStringRunner,
    // MapAccessRunner,
    // FunctionRunner,
    // AggregateExpressionWithFilterRunner,
    // CaseRunner,
    // ExistsRunner,
    // SubqueryRunner,
    // ArraySubqueryRunner,
    // ListAggRunner,
    // ArrayAggRunner,
    // GroupingSetsRunner,
    // CubeRunner,
    // RollupRunner,
    // TupleRunner,
    // StructRunner,
    // NamedRunner,
    // ArrayIndexRunner,
    // ArrayRunner,
    // IntervalRunner,
    // MatchAgainstRunner,
}

pub fn parse_expression(expression: &Expr) -> Runner {

    match expression {
        Expr::Identifier(_) => Runner::from(IdentifierRunner),
        Expr::CompoundIdentifier(_) => todo!(),
        Expr::JsonAccess { left, operator, right } => todo!(),
        Expr::CompositeAccess { expr, key } => todo!(),
        Expr::IsFalse(_) => todo!(),
        Expr::IsNotFalse(_) => todo!(),
        Expr::IsTrue(_) => todo!(),
        Expr::IsNotTrue(_) => todo!(),
        Expr::IsNull(_) => todo!(),
        Expr::IsNotNull(_) => todo!(),
        Expr::IsUnknown(_) => todo!(),
        Expr::IsNotUnknown(_) => todo!(),
        Expr::IsDistinctFrom(_, _) => todo!(),
        Expr::IsNotDistinctFrom(_, _) => todo!(),
        Expr::InList { expr, list, negated } => todo!(),
        Expr::InSubquery { expr, subquery, negated } => todo!(),
        Expr::InUnnest { expr, array_expr, negated } => todo!(),
        Expr::Between { expr, negated, low, high } => todo!(),
        Expr::BinaryOp { left, op, right } => todo!(),
        Expr::Like { negated, expr, pattern, escape_char } => todo!(),
        Expr::ILike { negated, expr, pattern, escape_char } => todo!(),
        Expr::SimilarTo { negated, expr, pattern, escape_char } => todo!(),
        Expr::RLike { negated, expr, pattern, regexp } => todo!(),
        Expr::AnyOp { left, compare_op, right } => todo!(),
        Expr::AllOp { left, compare_op, right } => todo!(),
        Expr::UnaryOp { op, expr } => todo!(),
        Expr::Convert { expr, data_type, charset, target_before_value } => todo!(),
        Expr::Cast { expr, data_type, format } => todo!(),
        Expr::TryCast { expr, data_type, format } => todo!(),
        Expr::SafeCast { expr, data_type, format } => todo!(),
        Expr::AtTimeZone { timestamp, time_zone } => todo!(),
        Expr::Extract { field, expr } => todo!(),
        Expr::Ceil { expr, field } => todo!(),
        Expr::Floor { expr, field } => todo!(),
        Expr::Position { expr, r#in } => todo!(),
        Expr::Substring { expr, substring_from, substring_for, special } => todo!(),
        Expr::Trim { expr, trim_where, trim_what, trim_characters } => todo!(),
        Expr::Overlay { expr, overlay_what, overlay_from, overlay_for } => todo!(),
        Expr::Collate { expr, collation } => todo!(),
        Expr::Nested(_) => todo!(),
        Expr::Value(_) =>  Runner::from(ValueRunner),
        Expr::IntroducedString { introducer, value } => todo!(),
        Expr::TypedString { data_type, value } => todo!(),
        Expr::MapAccess { column, keys } => todo!(),
        Expr::Function(_) => todo!(),
        Expr::AggregateExpressionWithFilter { expr, filter } => todo!(),
        Expr::Case { operand, conditions, results, else_result } => todo!(),
        Expr::Exists { subquery, negated } => todo!(),
        Expr::Subquery(_) => todo!(),
        Expr::ArraySubquery(_) => todo!(),
        Expr::ListAgg(_) => todo!(),
        Expr::ArrayAgg(_) => todo!(),
        Expr::GroupingSets(_) => todo!(),
        Expr::Cube(_) => todo!(),
        Expr::Rollup(_) => todo!(),
        Expr::Tuple(_) => todo!(),
        Expr::Struct { values, fields } => todo!(),
        Expr::Named { expr, name } => todo!(),
        Expr::ArrayIndex { obj, indexes } => todo!(),
        Expr::Array(_) => todo!(),
        Expr::Interval(_) => todo!(),
        Expr::MatchAgainst { columns, match_value, opt_search_modifier } => todo!(),
    }
}
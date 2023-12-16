use sqlparser::ast::Expr;

use crate::core::row::ValueContainer;

use super::runner::{RunnerTrait, ExpressionParserError};


pub struct IdentifierRunner;

impl RunnerTrait for IdentifierRunner {
    fn execute(&mut self, expression: &Expr) -> Result<ValueContainer, ExpressionParserError> {
        
        let Expr::Identifier(identifier) = expression else {
            return Err(ExpressionParserError::new(format!("Could not convert unknown expression {expression}")));
        };

        if identifier.quote_style.is_some() {
            Ok(ValueContainer::create_string(&identifier.value))
        } else {
            let number: i32 = identifier.value.parse().unwrap();
            Ok(ValueContainer::create_int(number))
        }
    }
}
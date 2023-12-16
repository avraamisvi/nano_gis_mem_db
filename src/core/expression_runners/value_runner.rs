use sqlparser::ast::Expr;

use crate::core::row::ValueContainer;

use super::runner::{RunnerTrait, ExpressionParserError};

pub struct ValueRunner;

impl RunnerTrait for ValueRunner {
    fn execute(&mut self, expression: &Expr) -> Result<ValueContainer, ExpressionParserError> {
        
        let Expr::Value(value) = expression else {
            return Err(ExpressionParserError::new(format!("Could not convert unknown expression {expression}")));
        };

        match value {
            sqlparser::ast::Value::Number(data, _) => Ok(ValueContainer::create_int(data.parse().unwrap())),
            sqlparser::ast::Value::SingleQuotedString(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::DollarQuotedString(data) => Ok(ValueContainer::create_string(&data.value)),
            sqlparser::ast::Value::EscapedStringLiteral(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::SingleQuotedByteStringLiteral(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::DoubleQuotedByteStringLiteral(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::RawStringLiteral(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::NationalStringLiteral(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::HexStringLiteral(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::DoubleQuotedString(data) => Ok(ValueContainer::create_string(data)),
            sqlparser::ast::Value::Boolean(data) => todo!(),
            sqlparser::ast::Value::Null => todo!(),
            sqlparser::ast::Value::Placeholder(_) => todo!(),
            sqlparser::ast::Value::UnQuotedString(data) => Ok(ValueContainer::create_string(data)),
        }

        // if value.quote_style.is_some() {
        //     Ok(ValueContainer::create_string(&identifier.value))
        // } else {
        //     let number: i32 = identifier.value.parse().unwrap();
        //     Ok(ValueContainer::create_int(number))
        // }
    }
}
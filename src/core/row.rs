use super::column::DataType;
use core::fmt::Display;

#[derive(Ord)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Clone)]
pub enum Value {
    StringValue(String),
    IntegerValue(i32)
}

#[derive(Ord)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Clone)]
pub struct ValueContainer {
    pub data_type: DataType,
    pub value: Value
}

impl ValueContainer {

    pub fn create_string(data: &str)-> Self {
        return ValueContainer {
            data_type: DataType::STRING,
            value: Value::StringValue(data.to_string())
        };
    }

    pub fn create_int(data: i32)-> Self {
        return ValueContainer {
            data_type: DataType::INT,
            value: Value::IntegerValue(data)
        };
    }
}

impl Display for ValueContainer {
    fn fmt(&self, formater: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self.value {
            Value::StringValue(data) => write!(formater, "{}", data),
            Value::IntegerValue(data) => write!(formater, "{}", data),
        }
    }
}

#[derive(Clone)]
pub struct Row {
    pub row_id: u64,
    pub values: Vec<ValueContainer>
}
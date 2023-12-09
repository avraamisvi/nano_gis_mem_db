use core::fmt::Display;

use super::row::ValueContainer;

#[derive(Ord)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Clone)]
pub enum DataType {
    INT, 
    STRING
}

impl Display for DataType {
    fn fmt(&self, formater: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            DataType::INT => write!(formater, "Integer"),
            DataType::STRING => write!(formater, "String"),
        }
    }
}

pub struct Column {
    pub name: String,
    pub data_type: DataType
}

impl Column {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Column { name: name.to_string(), data_type: data_type }
    }
}

pub struct DataTypeIncompatibleWithColumn {
    pub column_name: String,
    pub column_data_type: DataType,
    pub value: ValueContainer,
    pub value_data_type: DataType 
}

impl DataTypeIncompatibleWithColumn {
    
    pub fn new(column: &Column, 
               value: &ValueContainer) -> Self {

        return DataTypeIncompatibleWithColumn {
            column_name: column.name.clone(),
            column_data_type: column.data_type.clone(),
            value: value.clone(),
            value_data_type: value.data_type.clone() 
        };
    }
}

impl Display for DataTypeIncompatibleWithColumn {
    fn fmt(&self, formater: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(formater,
            "
            Column is incompatible with Type.
            {{
                Column: {},
                Column Type: {},
                Value: {},
                Value Type: {} 
            }}
            ",
            self.column_name.clone(),
            self.column_data_type.clone(),
            self.value.clone(),
            self.value_data_type.clone()             
        )
    }
}
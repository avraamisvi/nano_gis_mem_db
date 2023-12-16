
use sqlparser::ast::{ColumnDef, ObjectName, Statement};
use crate::core::{database::Database, data_parser::parse_data_type, table::Table, column::Column};
use super::{QueryExecutionError, QueryExecutor};

pub struct CreateTableExecutor {
    name: ObjectName,
    columns: Vec<ColumnDef>
}

impl CreateTableExecutor {

    pub fn new(name: ObjectName, columns: Vec<ColumnDef>) -> Self {
        CreateTableExecutor { name: name, columns: columns }
    }

    fn create_table(&mut self) -> Table {

        let ObjectName(identifier) = self.name.clone();
        let proper_name = &identifier[0].value;
        
        Table::new(&proper_name, self.parse_columns(&self.columns.clone()))
    }

    fn parse_columns(&mut self, columns: &Vec<ColumnDef>) -> Vec<Column>{
        return columns.into_iter().map(|col| {
            Column{
                name: col.name.value.clone(),
                data_type: parse_data_type(col.data_type.clone())
            }
        }).collect();
    }    
}

impl QueryExecutor for CreateTableExecutor {
    fn execute(&mut self, database: &mut Database) -> Result<u64, QueryExecutionError> {
        let table = self.create_table();
        database.add_table(table);
        Ok(0_u64)
    }
}

impl From<Statement> for CreateTableExecutor {
    fn from(value: Statement) -> Self {
        let Statement::CreateTable { 
            name, 
            columns, ..} = value else {
                panic!("Another panic, is this good practice?");
            };
        
        CreateTableExecutor::new(name.clone(), columns.clone())
    }
}
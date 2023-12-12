use std::collections::HashMap;

use sqlparser::ast::{ColumnDef, ObjectName, Ident, Query, Statement};

use super::{database::Database, table::Table, column::Column, data_parser::parse_data_type};


#[derive(Eq)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct QueryExecutionError;

// pub struct Executors {
//     map: HashMap<Statement, Box<dyn QueryExecutor>>
// }

// impl

pub trait QueryExecutor {
    fn execute(&mut self, database: &mut Database) -> Result<u64, QueryExecutionError>;
}


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
        database.addTable(table);
        Ok(0_u64)
    }
}

pub struct InsertExecutor {
    name: ObjectName,
    columns: Vec<Ident>,
    query: Box<Query>
}

impl InsertExecutor {
    pub fn new(name: ObjectName, columns: Vec<Ident>, query: Box<Query>) -> Self {
        InsertExecutor { name, columns, query }
    }
}

impl QueryExecutor for InsertExecutor {
    fn execute(&mut self, database: &mut Database) -> Result<u64, QueryExecutionError> {
        todo!()
    }
}
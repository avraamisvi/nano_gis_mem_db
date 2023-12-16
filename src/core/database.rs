use std::collections::HashMap;
use std::sync::Mutex;

use sqlparser::ast::Statement;
use sqlparser::parser::Parser;
use sqlparser::dialect::PostgreSqlDialect;

use super::executors::{QueryExecutionError, QueryExecutor, Executor};
use super::parser::parse_statement;
use super::table::Table;

#[cfg(test)]
mod tests;

struct TablesContainer {
    map: HashMap<String, Mutex<Table>>
}

impl TablesContainer {
    fn add(&mut self, table: Table) {
        self.map.insert(table.name.clone(), Mutex::from(table));
    }

    fn get_table(&mut self, name: &String) -> &Mutex<Table> {
        self.map.get(name).unwrap()
    }
}

pub struct Database {
    dialect: PostgreSqlDialect,
    tables: TablesContainer
}

impl Database {
    
    pub fn new() -> Self {
        let tables = TablesContainer{map: HashMap::new()};
        Database{ dialect: PostgreSqlDialect{}, tables: tables}
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.add(table);
    }

    pub fn get_table(&mut self, name: &String) -> &Mutex<Table> {
        self.tables.get_table(name)
    }

    pub fn execute(&mut self, query: &str) -> Result<u64, QueryExecutionError> {

        let result = Parser::parse_sql(&self.dialect, query);

       return match result {
            Ok(ast) => self.execute_ast(&ast),
            Err(_) => Err(QueryExecutionError)
        }
    } 

    fn execute_ast(&mut self, ast: &Vec<Statement>) -> Result<u64, QueryExecutionError> {

        for statement in ast {

            let executor = &mut parse_statement(statement);
            let result = executor.execute(self);

            if result.is_err() {
                return result;
            }
        }

        return Ok(0);
    }
}

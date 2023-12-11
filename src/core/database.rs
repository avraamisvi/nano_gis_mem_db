use std::collections::HashMap;

use sqlparser::ast::Statement;
use sqlparser::parser::Parser;
use sqlparser::dialect::PostgreSqlDialect;

use super::executors::{QueryExecutionError, QueryExecutor};
use super::parser::parse_statement;
use super::table::Table;

#[cfg(test)]
mod tests;

struct TablesContainer {
    map: HashMap<String, Table>
}

impl TablesContainer {
    fn add(&mut self, table: Table) {
        self.map.insert(table.name.clone(), table);
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

    pub fn addTable(&mut self, table: Table) {
        self.tables.map.insert(table.name.clone(), table);
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

            let result = parse_statement(statement).execute(self);

            if result.is_err() {
                return result;
            }
        }

        return Ok(0);
    }
}

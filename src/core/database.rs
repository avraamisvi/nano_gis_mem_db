use std::collections::HashMap;

use sqlparser::ast::{Statement, Query, ObjectName, ColumnDef};
use sqlparser::parser::Parser;
use sqlparser::dialect::PostgreSqlDialect;

use super::column::{Column, DataType};
use super::table::Table;

#[cfg(test)]
mod tests;

#[derive(Eq)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct QueryExecutionError;

trait QueryExecutor {
    fn execute(&mut self, database: &mut Database, statement: Statement) -> Result<u64, QueryExecutionError>;    
}

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

struct CreateTableExecutor;

impl CreateTableExecutor {
    fn create_table(&mut self, database: &mut Database, 
                    name: &ObjectName, 
                    columns: &Vec<ColumnDef>) -> Table {

        let ObjectName(identifier) = name;
        let proper_name = &identifier[0].value;
        
        Table::new(&proper_name, self.parse_columns(&columns))
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

    fn execute(&mut self, database: &mut Database, statement: Statement) -> Result<u64, QueryExecutionError> {

        if let Statement::CreateTable{name, columns, ..} = statement {
            self.create_table(database, &name, &columns);
        }

        return Ok(0);
    }
    
}

impl Database {
    
    pub fn new() -> Self {
        let tables = TablesContainer{map: HashMap::new()};
        Database{ dialect: PostgreSqlDialect{}, tables: tables}
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
            
            let result: Result<u64, QueryExecutionError> = match statement {
                Statement::CreateTable { name, columns, .. } => {
                    let table = CreateTableExecutor.create_table(self, name, columns);
                    self.tables.add(table);
                    Ok(0_u64)
                },
                _ => unimplemented!()
            };

            if result.is_err() {
                return result;
            }
        }

        return Ok(0);
    }
}

fn parse_data_type(data_type: sqlparser::ast::DataType) -> DataType {
    return match data_type {
        sqlparser::ast::DataType::String(_) => DataType::STRING,
        sqlparser::ast::DataType::Int(_) => DataType::INT,
        _  => DataType::STRING
        // DataType::Character(_) => todo!(),
        // DataType::Text => todo!(),
        // DataType::Char(_) => todo!(),
        // DataType::CharacterVarying(_) => todo!(),
        // DataType::CharVarying(_) => todo!(),
        // DataType::Varchar(_) => todo!(),
        // DataType::Nvarchar(_) => todo!(),
        // DataType::Uuid => todo!(),
        // DataType::CharacterLargeObject(_) => todo!(),
        // DataType::CharLargeObject(_) => todo!(),
        // DataType::Clob(_) => todo!(),
        // DataType::Binary(_) => todo!(),
        // DataType::Varbinary(_) => todo!(),
        // DataType::Blob(_) => todo!(),
        // DataType::Bytes(_) => todo!(),
        // DataType::Numeric(_) => todo!(),
        // DataType::Decimal(_) => todo!(),
        // DataType::BigNumeric(_) => todo!(),
        // DataType::BigDecimal(_) => todo!(),
        // DataType::Dec(_) => todo!(),
        // DataType::Float(_) => todo!(),
        // DataType::TinyInt(_) => todo!(),
        // DataType::UnsignedTinyInt(_) => todo!(),
        // DataType::Int2(_) => todo!(),
        // DataType::UnsignedInt2(_) => todo!(),
        // DataType::SmallInt(_) => todo!(),
        // DataType::UnsignedSmallInt(_) => todo!(),
        // DataType::MediumInt(_) => todo!(),
        // DataType::UnsignedMediumInt(_) => todo!(),
        // DataType::Int4(_) => todo!(),
        // DataType::Int64 => todo!(),
        // DataType::Integer(_) => todo!(),
        // DataType::UnsignedInt(_) => todo!(),
        // DataType::UnsignedInt4(_) => todo!(),
        // DataType::UnsignedInteger(_) => todo!(),
        // DataType::BigInt(_) => todo!(),
        // DataType::UnsignedBigInt(_) => todo!(),
        // DataType::Int8(_) => todo!(),
        // DataType::UnsignedInt8(_) => todo!(),
        // DataType::Float4 => todo!(),
        // DataType::Float64 => todo!(),
        // DataType::Real => todo!(),
        // DataType::Float8 => todo!(),
        // DataType::Double => todo!(),
        // DataType::DoublePrecision => todo!(),
        // DataType::Bool => todo!(),
        // DataType::Boolean => todo!(),
        // DataType::Date => todo!(),
        // DataType::Time(_, _) => todo!(),
        // DataType::Datetime(_) => todo!(),
        // DataType::Timestamp(_, _) => todo!(),
        // DataType::Interval => todo!(),
        // DataType::JSON => todo!(),
        // DataType::Regclass => todo!(),
        // DataType::Bytea => todo!(),
        // DataType::Custom(_, _) => todo!(),
        // DataType::Array(_) => todo!(),
        // DataType::Enum(_) => todo!(),
        // DataType::Set(_) => todo!(),
        // DataType::Struct(_) => todo!(),
    }
}
use super::database::Database;

pub mod insert_executor;
pub mod create_table_executor;

use crate::core::executors::create_table_executor::CreateTableExecutor;
use crate::core::executors::insert_executor::InsertExecutor;

#[derive(Eq)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct QueryExecutionError;

#[enum_dispatch::enum_dispatch]
pub trait QueryExecutor {
    fn execute(&mut self, database: &mut Database) -> Result<u64, QueryExecutionError>;
}

#[enum_dispatch::enum_dispatch(QueryExecutor)]
pub enum Executor {
    CreateTableExecutor,
    InsertExecutor
}

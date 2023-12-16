use sqlparser::ast::{ObjectName, Ident, Query, Statement, Values, Expr};
use crate::core::{database::Database, row::{ValueContainer, Value}, column::{DataType, DataTypeIncompatibleWithColumn}, expression_runners::runner::{parse_expression, RunnerTrait, ExpressionParserError}};
use super::{QueryExecutionError, QueryExecutor};

pub struct InsertExecutor {
    name: ObjectName,
    columns: Vec<Ident>,
    query: Box<Query>
}

impl InsertExecutor {
    pub fn new(name: ObjectName, columns: Vec<Ident>, query: Box<Query>) -> Self {
        InsertExecutor { name, columns, query }
    }

    pub fn get_table_name(&self) -> String {
        let ObjectName(ident) = &self.name;
        ident[0].value.clone()
    }
}

impl QueryExecutor for InsertExecutor {
    fn execute(&mut self, database: &mut Database) -> Result<u64, QueryExecutionError> {
        let table = database.get_table(&self.get_table_name());
        let mut table = table.lock().unwrap();
        let rows = get_values(&self.query);
        
        //NOTA: EXISTE UM JEITO DE EVITAR MUTABILITY AQUI?
        //BOA PRATICA?
        let mut result: Result<(), DataTypeIncompatibleWithColumn> = Ok(());

        for values in rows {
            result = table.insert(values);
            //TODO add errors in log
            if result.is_err() {
                break;
            }
        }

        match result {
            Ok(_) => Ok(1),
            Err(_) => Err(QueryExecutionError)
        }
    }
}

impl From<Statement> for InsertExecutor {
    fn from(value: Statement) -> Self {

        let Statement::Insert { table_name, 
            columns, 
            source, .. } = value else {
                //NOTA: Is this a good practice?
                panic!("Invalid Enum Type");
            };
        
        InsertExecutor::new(table_name.clone(), 
                                    columns.clone(), 
                                    source.clone().unwrap())
    }
}

fn get_values(query: &Box<Query>) -> Vec<Vec<ValueContainer>> {
    let body = &query.body;

    match body.as_ref() {
        sqlparser::ast::SetExpr::Values(values) => parse_values(values),
        sqlparser::ast::SetExpr::Select(_) => todo!(),
        sqlparser::ast::SetExpr::Query(_) => todo!(),
        sqlparser::ast::SetExpr::SetOperation { op, set_quantifier, left, right } => todo!(),
        sqlparser::ast::SetExpr::Insert(_) => todo!(),
        sqlparser::ast::SetExpr::Update(_) => todo!(),
        sqlparser::ast::SetExpr::Table(_) => todo!(),
    }
}

fn parse_values(values: &Values) -> Vec<Vec<ValueContainer>> {
    let rows = &values.rows;
    rows.into_iter().map(|row| {
        row.into_iter().map(|column| {
            parse_column_value(column.clone())
        }).filter_map(|item| {item})
        .collect()
    }).collect()
}

fn parse_column_value(expression: Expr) -> Option<ValueContainer> {

    //println!("############# Expression is: {} ############# ", expression);

    let mut runner = parse_expression(&expression);
    let parsed = runner.execute(&expression);

    match parsed {
     Ok(value) => Some(value),
     Err(error) => {
        //ADD PARSE ERROR TO LOGS
        None
     }   
    }
}
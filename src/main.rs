use core::{table::Table, row::{Row, Value}, column::{DataTypeIncompatibleWithColumn, Column, DataType}};

use crate::core::row::ValueContainer;

mod core;

fn main() {
    let mut tableAbacate = Table::new("Abacate", vec![
        Column::new("id", DataType::INT),
        Column::new("name", DataType::STRING),
        Column::new("age", DataType::INT),
    ]);

    let insert_result = tableAbacate.insert(vec![
        ValueContainer::create_int(1), 
        ValueContainer::create_string("espionage"), 
        ValueContainer::create_int(3)]);

    log_error(insert_result);

    let insert_result = tableAbacate.insert(vec![
        ValueContainer::create_int(3), 
        ValueContainer::create_string("abacate"), 
        ValueContainer::create_int(8)]);

    log_error(insert_result);

    let index_name = "item_name_idx";

    let insert_result = tableAbacate.insert(vec![
        ValueContainer::create_string("1"), 
        ValueContainer::create_string("dinosauro"), 
        ValueContainer::create_int(3)]);

    log_error(insert_result);
    
    tableAbacate.createIndex(1, index_name);
    let result = tableAbacate.find_indexed_row_by_value_equals(index_name, &ValueContainer::create_string("dinosauro"));
    
    match result {
        Some(row) => println!("Found: {}", row.row_id),
        None => println!("Nothing found")
    }

    
}

fn log_error(result: Result<(), DataTypeIncompatibleWithColumn>) {
    match result {
        Err(error) => println!("Error {}", error),
        Ok(_) => ()
    }
}
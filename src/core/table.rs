use std::collections::HashMap;
use super::column::DataTypeIncompatibleWithColumn;
use super::row::Row;
use super::column::Column;
use super::row::ValueContainer;
use std::collections::BTreeMap; 

pub struct Table {
    pub name: String,
    columns: Vec<Column>,
    indexes: HashMap<String, BTreeMap<ValueContainer, Row>>,
    rows: Vec<Row>,
    row_next_id: u64
}

impl Table {

    pub fn new(name: &str, columns: Vec<Column>) -> Self {
        Table { 
            name: name.to_string(), 
            columns: columns, 
            rows: Vec::new(),
            indexes: HashMap::new(),
            row_next_id:  0
        }
    }

    pub fn insert(&mut self, values: Vec<ValueContainer>) -> Result<(), DataTypeIncompatibleWithColumn> {

        let result = self.is_row_compatible_with_columns(&values);

        let row = Row { row_id: self.row_next_id.clone(), values: values };

        match result {
            Err(_) => result,
            Ok(_) => {
                self.rows.push(row);
                self.row_next_id = self.row_next_id + 1;
                Ok(())
            }
        }
    }

    pub fn createIndex(&mut self, column_idx: usize, name: &str) {

        let mut index = BTreeMap::new();

        for row in &self.rows {
            index.insert(row.values[column_idx].clone(), row.clone());
        }

        self.indexes.insert(name.to_string(), index);
    }

    pub fn find_indexed_row_by_value_equals(&mut self, index_name: &str,
         value_container: &ValueContainer) -> Option<&Row> {

        let index = self.indexes.get(&index_name.to_string());

        return match index {
            Some(index) => index.get(&value_container),
            None => Option::None
        }
    }

    fn is_row_compatible_with_columns(&mut self, values: &Vec<ValueContainer>) -> Result<(), DataTypeIncompatibleWithColumn> {

        for idx in 0..self.columns.len() {
            let column = &self.columns[idx];
            let value = &values[idx];

            if value.data_type != column.data_type {
                return Err(DataTypeIncompatibleWithColumn::new(&column, &value));
             }
        }

        return Ok(());
    }

}
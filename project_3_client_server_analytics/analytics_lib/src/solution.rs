use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn evaluate_condition(row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
    match condition {
        Condition::Equal(col_name, value) => {
            let col_index = dataset.column_index(col_name);
            row.get_value(col_index) == value
        },
        Condition::Not(inner) => !evaluate_condition(row, inner, dataset),
        Condition::And(left, right) => evaluate_condition(row, left, dataset) && evaluate_condition(row, right, dataset),
        Condition::Or(left, right) => evaluate_condition(row, left, dataset) || evaluate_condition(row, right, dataset),
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    //todo!("Implement this!");
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if evaluate_condition(row, filter, dataset) {
            result.add_row(row.clone());
        }
    }
    result
}

/*
    let mut result = Dataset::new(dataset.columns().clone());
    let column_to_check = dataset.column_index(&filter.get_group_by());
    for row in dataset.iter() {
        let value_to_check = row.get_value(column_to_check);
        if value_to_check == Condition::Equal(filter) {
            result.add_row(row.clone());
        }
    }
    result
}
*/

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}
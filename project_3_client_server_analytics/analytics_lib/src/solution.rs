use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    dataset.into_iter().map(|(group_value, group_dataset)| {
    let value = match aggregation {
        Aggregation::Count(_) => {
            Value::Integer(group_dataset.len() as i32)
        }

        Aggregation::Sum(column_name) => {
            let col_index = group_dataset.column_index(column_name);
            let sum: i32 = group_dataset.iter()
                .filter_map(|row| {
                    if let Value::Integer(v) = row.get_value(col_index) {
                        Some(*v)
                        } 
                    else {
                        None
                    } })
                .sum();
            Value::Integer(sum)
            }
        Aggregation::Average(column_name) => {
            let col_index = group_dataset.column_index(column_name);
            let values: Vec<i32> = group_dataset.iter()
                .filter_map(|row| {
                if let Value::Integer(v) = row.get_value(col_index) {
                    Some(*v)
                    } 
                    else {
                        None
                    }
                    })
                    .collect();

            let avg = if values.is_empty() { // looked up helper function to confirm that vector is empty 
                0
                } 
                else {
                    values.iter().sum::<i32>() / values.len() as i32
                };
                Value::Integer(avg)
            }
        };

        (group_value, value)

    }).collect()
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
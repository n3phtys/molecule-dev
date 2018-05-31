extern crate serde_json;

use common::AppState;
use std::collections::HashMap;

type JSON = serde_json::Value;

pub trait DataSource {
    fn process(response_values: &HashMap<String, JSON>, &AppState) -> Result<(), ()>;
}

pub trait MutableDataSource {
    fn process(response_values: &HashMap<String, JSON>, &mut AppState) -> Result<(), ()>;
}
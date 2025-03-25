use std::collections::HashMap;

use extism_pdk::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: HashMap<String, Vec<Vec<f64>>>,
}

#[plugin_fn]
pub fn run(fin_data: Json<FinData>) -> FnResult<String> {
    let fin_data = fin_data.into_inner();
    let candles_data: HashMap<String, Vec<Vec<f64>>> = fin_data.data;
    let symbols = candles_data.keys().collect::<Vec<_>>();
    if symbols.len() != 2 {
        return Ok("Please provide exactly 2 symbols for correlation calculation".to_string());
    }
    Ok("TODO".into())
}
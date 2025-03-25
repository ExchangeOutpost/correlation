use std::collections::HashMap;
use std::fmt::Write; // Add this import

use extism_pdk::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: HashMap<String, Vec<Vec<f64>>>,
}

#[plugin_fn]
pub fn run(fin_data: Json<FinData>) -> FnResult<String> {
    let fin_data = fin_data.into_inner();
    let mut result = String::new(); // Pre-allocate memory for better performance
    for (key, value) in fin_data.data.iter() {
        let close_prices = value.iter().map(|x| x[3]).collect::<Vec<f64>>();
        let max = close_prices.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min = close_prices.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let avg = close_prices.iter().sum::<f64>() / close_prices.len() as f64;
        writeln!(
            result,
            "Symbol: {}Max: {}, Min: {}, Avg: {}",
            key, max, min, avg
        ).unwrap(); // Use writeln! for consistent newline handling
    }
    Ok(result)
}
use extism_pdk::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: Vec<Vec<f64>>,
}

#[plugin_fn]
pub fn run(data: Json<FinData>) -> FnResult<String> {
    let data = data.into_inner();
    let close_prices = data.data.iter().map(|x| x[3]).collect::<Vec<f64>>();
    let max = close_prices.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min = close_prices.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let avg = close_prices.iter().sum::<f64>() / close_prices.len() as f64;
    let result = format!("Max: {}, Min: {}, Avg: {}", max, min, avg);
    Ok(result)
}
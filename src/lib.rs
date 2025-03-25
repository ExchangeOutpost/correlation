use std::collections::HashMap;
use ndarray::Array1;
use extism_pdk::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: HashMap<String, Vec<Vec<f64>>>,
}

/// Calculate the Pearson correlation coefficient between two stock price series
fn pearson_correlation(x: &Array1<f64>, y: &Array1<f64>) -> f64 {
    let x_mean = x.mean().unwrap();
    let y_mean = y.mean().unwrap();

    let numerator = x.iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - x_mean) * (yi - y_mean))
        .sum::<f64>();

    let denominator_x = x.iter()
        .map(|&xi| (xi - x_mean).powi(2))
        .sum::<f64>()
        .sqrt();

    let denominator_y = y.iter()
        .map(|&yi| (yi - y_mean).powi(2))
        .sum::<f64>()
        .sqrt();

    numerator / (denominator_x * denominator_y)
}


#[plugin_fn]
pub fn run(fin_data: Json<FinData>) -> FnResult<String> {
    let fin_data = fin_data.into_inner();
    let candles_data: HashMap<String, Vec<Vec<f64>>> = fin_data.data;
    let symbols = candles_data.keys().collect::<Vec<_>>();
    if symbols.len() != 2 {
        return Ok("Please provide exactly 2 symbols for correlation calculation".to_string());
    }
    let correlation=  pearson_correlation(
        &Array1::from(candles_data.get(symbols[0]).unwrap().iter().map(|x| x[2]).collect::<Vec<f64>>()),
        &Array1::from(candles_data.get(symbols[1]).unwrap().iter().map(|x| x[2]).collect::<Vec<f64>>()),
    );
    Ok(correlation.to_string())
}
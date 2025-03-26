use std::collections::HashMap;
use ndarray::Array1;
use extism_pdk::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: HashMap<String, Vec<Vec<f64>>>,
}
#[derive(Serialize, Deserialize)]
pub struct Output {
    correlation: f64,
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
    let symbol_1 = match candles_data.get("symbol_1") {
        Some(data) => data,
        None => return Err(WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, "symbol_1 not found")), -100)),
    };

    let symbol_2 = match candles_data.get("symbol_2") {
        Some(data) => data,
        None => return Err(WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, "symbol_2 not found")), -100)),
    };
    
    let correlation=  pearson_correlation(
        &Array1::from(symbol_1.iter().map(|x| x[2]).collect::<Vec<f64>>()),
        &Array1::from(symbol_2.iter().map(|x| x[2]).collect::<Vec<f64>>()),
    );
    let out = Output {
        correlation: correlation,
    };
    let function_response = serde_json::to_string(&out).unwrap();
    Ok (function_response)
}
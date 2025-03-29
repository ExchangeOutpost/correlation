use extism_pdk::*;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolsData{
    symbol_1: Vec<Vec<f64>>,
    symbol_2: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: SymbolsData,
}
#[derive(Serialize, Deserialize)]
pub struct Output {
    correlation: f64,
}

// Calculate the Pearson correlation coefficient between two stock price series
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
    
    let correlation=  pearson_correlation(
        &Array1::from(fin_data.data.symbol_1.iter().map(|x| x[2]).collect::<Vec<f64>>()),
        &Array1::from(fin_data.data.symbol_2.iter().map(|x| x[2]).collect::<Vec<f64>>()),
    );
    let out = Output {
        correlation: correlation,
    };
    let function_response = serde_json::to_string(&out).unwrap();
    Ok (function_response)
}
mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use ndarray::Array1;
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    symbols: [String;2],
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
pub fn run(fin_data: FinData<f64>) -> FnResult<Output> {
    let ticker_1 = fin_data.get_ticker("symbol_1")?;
    let ticker_2 = fin_data.get_ticker("symbol_2")?;
    let correlation = pearson_correlation(
        &Array1::from_iter(ticker_1.candles.windows(2)
            .map(|w| (w[1].close / w[0].close).ln())),
        &Array1::from_iter(ticker_2.candles.windows(2)
            .map(|w| (w[1].close / w[0].close).ln())),
    );
    Ok(Output {correlation, symbols: [ticker_1.symbol.clone(), ticker_2.symbol.clone()]})
}
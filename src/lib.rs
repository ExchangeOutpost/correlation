use extism_pdk::*;
use ndarray::Array1;
use serde::{Deserialize, Deserializer, Serialize};

/// Represents a single candlestick in financial data, typically used in trading charts.
pub struct Candle {
    /// The timestamp of the candlestick, usually in seconds since the Unix epoch.
    pub timestamp: i64,
    /// The opening price of the asset during the candlestick's time period.
    pub open: f64,
    /// The highest price of the asset during the candlestick's time period.
    pub high: f64,
    /// The lowest price of the asset during the candlestick's time period.
    pub low: f64,
    /// The closing price of the asset at the end of the candlestick's time period.
    pub close: f64,
    /// The trading volume of the asset during the candlestick's time period.
    pub volume: f64,
}
// Custom deserialization from an array
impl<'de> Deserialize<'de> for Candle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: (i64, f64, f64, f64, f64, f64) = Deserialize::deserialize(deserializer)?;
        Ok(Candle {
            timestamp: arr.0,
            open: arr.1,
            high: arr.2,
            low: arr.3,
            close: arr.4,
            volume: arr.5,
        })
    }
}



#[derive(Deserialize)]
pub struct SymbolsData{
    symbol_1: Vec<Candle>,
    symbol_2: Vec<Candle>,
}

#[derive(Deserialize)]
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
        &Array1::from(fin_data.data.symbol_1.iter().map(|x| x.close).collect::<Vec<f64>>()),
        &Array1::from(fin_data.data.symbol_2.iter().map(|x| x.close).collect::<Vec<f64>>()),
    );
    let out = Output {
        correlation: correlation,
    };
    let function_response = serde_json::to_string(&out).unwrap();
    Ok (function_response)
}
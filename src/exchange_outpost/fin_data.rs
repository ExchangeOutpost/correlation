use std::collections::HashMap;
use crate::exchange_outpost::Candle;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use extism_pdk::*;
use extism_pdk::FromBytesOwned;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TickersData<T> {
    pub symbol: String,
    pub exchange: String,
    pub candles: Vec<Candle<T>>,
    pub precision: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FinData<T> {
    tickers_data: HashMap<String, TickersData<T>>,
    piped_data: HashMap<String,String>
}

impl<T> FromBytesOwned for FinData<T>
where
    T: DeserializeOwned,
{
    fn from_bytes_owned(bytes: &[u8]) -> Result<Self, extism_pdk::Error> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[allow(dead_code)]
impl<T> FinData<T> {
    pub fn get_candles(&self, label: &str) -> Result<&Vec<Candle<T>>, WithReturnCode<Error>> {
        self.tickers_data.get(label).and_then(|v| Some(&v.candles)).ok_or(
            WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!(
            "Symbol {} not found", label
        ))), 1))
    }

    pub fn get_pipe_sources (&self) -> Vec<&String> {
        self.piped_data.keys().collect()
    }

    pub fn get_data_from_pipe (&self, source: &str) -> Result<&String, WithReturnCode<Error>> {
        self.piped_data.get(source).ok_or(
            WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!(
            "Source {} not found", source
        ))), 2))
    }

    pub fn get_ticker (&self, label: &str) -> Result<&TickersData<T>, WithReturnCode<Error>> {
        self.tickers_data.get(label).ok_or(
            WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!(
            "Ticker {} not found", label
        ))), 3))
    }
}

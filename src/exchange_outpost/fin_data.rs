use std::collections::HashMap;
use crate::exchange_outpost::Candle;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use extism_pdk::*;
use extism_pdk::FromBytesOwned;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FinData<T> {
    tickers_data: HashMap<String, Vec<Candle<T>>>,
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
    pub fn get_candles(&self, symbol: &str) -> Result<&Vec<Candle<T>>, WithReturnCode<Error>> {
        self.tickers_data.get(symbol).ok_or(
            WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!(
            "Symbol {} not found", symbol
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
}

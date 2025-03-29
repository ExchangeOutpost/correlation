use std::collections::HashMap;
use crate::exchange_outpost::Candle;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use extism_pdk::*;
use extism_pdk::FromBytesOwned;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FinData<T> {
    data: HashMap<String, Vec<Candle<T>>>,
}

impl<T> FromBytesOwned for FinData<T>
where
    T: DeserializeOwned,
{
    fn from_bytes_owned(bytes: &[u8]) -> Result<Self, extism_pdk::Error> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
impl<T> FinData<T> {
    pub fn get_candles(&self, symbol: &str) -> Result<&Vec<Candle<T>>, WithReturnCode<Error>> {
        self.data.get(symbol).ok_or(
            WithReturnCode::new(Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!(
            "Symbol {} not found", symbol
        ))), -1))
    }
}

use extism_pdk::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FinData {
    data: String,
}

#[plugin_fn]
pub fn run(data: FinData) -> FnResult<String> {
    Ok(format!("Hello, {}!", data.data))
}
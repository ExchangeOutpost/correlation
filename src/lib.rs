use extism_pdk::*;

#[plugin_fn]
pub fn main(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}
use extism_pdk::*;

#[plugin_fn]
pub fn run(data: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", data))
}
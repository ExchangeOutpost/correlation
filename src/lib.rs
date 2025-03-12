use extism_pdk::*;

#[plugin_fn]
pub fn run(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}
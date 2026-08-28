use napi::{Error, Result};
use napi_derive::napi;
use oiper_snippets::{apply_snippets, parse_config};
use serde_json::Value;

fn parse_json(config_json: &str) -> Result<Value> {
    serde_json::from_str(config_json).map_err(|error| Error::from_reason(error.to_string()))
}

#[napi(js_name = "validateConfig")]
pub fn validate_config(config_json: String) -> Result<()> {
    let config = parse_json(&config_json)?;

    parse_config(&config)
        .map(|_| ())
        .map_err(|error| Error::from_reason(error.to_string()))
}

#[napi(js_name = "applySnippets")]
pub fn apply(config_json: String, input: String) -> Result<String> {
    let config = parse_json(&config_json)?;
    let config = parse_config(&config).map_err(|error| Error::from_reason(error.to_string()))?;

    Ok(apply_snippets(&input, &config))
}

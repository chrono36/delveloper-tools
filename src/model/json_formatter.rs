use std::error::Error;

use serde_json::{to_string_pretty, Value};

#[derive(Debug)]
pub struct JsonFormatter {}

impl JsonFormatter {
    pub fn pretty_json(text: &str) -> Result<String, &'static str> {
        match serde_json::from_str::<Value>(text) {
            Ok(json) => match to_string_pretty(&json) {
                Ok(formatted) => {
                    return Ok(formatted);
                }
                Err(e) => {
                    return Err("json formatter error");
                }
            },
            Err(_) => return Err("json parse error"),
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadTestConfig {
    pub url: String,
    pub method: Option<HttpMethod>,
    pub request_count: usize,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>, // Static body or template for dynamic body
    pub data_file: Option<String>, // Path to your Excel/CSV file
}

pub fn load_config(file_path: &str) -> Result<LoadTestConfig, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(file_path)?;
    let config: LoadTestConfig = serde_yaml::from_str(&file_contents)?;
    Ok(config)
}

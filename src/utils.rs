use std::collections::HashMap;

use crate::config::HttpMethod;

pub fn replace_placeholders(template: &str, data: &HashMap<String, String>) -> String {
    let mut body = template.to_string();
    for (key, value) in data {
        body = body.replace(&format!("${{{}}}", key), value);
    }
    body
}

pub fn http_method_to_reqwest_method(method: &HttpMethod) -> reqwest::Method {
    match method {
        HttpMethod::Get => reqwest::Method::GET,
        HttpMethod::Post => reqwest::Method::POST,
        HttpMethod::Put => reqwest::Method::PUT,
        HttpMethod::Delete => reqwest::Method::DELETE,
        HttpMethod::Patch => reqwest::Method::PATCH,
    }
}

// Function to display progress
pub fn display_progress(first: usize, second: usize) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Progress: {first}/{second}");
}

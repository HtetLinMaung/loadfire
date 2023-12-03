use std::{collections::HashMap, sync::Arc, time::{Instant, Duration}};

use futures::future::join_all;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::sync::Mutex;

use crate::{
    config::LoadTestConfig,
    data::load_data,
    utils::{http_method_to_reqwest_method, replace_placeholders, display_progress},
};

pub async fn send_request(
    config: &LoadTestConfig,
    data_row: &Option<HashMap<String, String>>,
) -> Result<reqwest::Response, Box<dyn std::error::Error + Send>> {
    let client = reqwest::Client::new();

    let method = match &config.method {
        Some(m) => http_method_to_reqwest_method(m),
        None => reqwest::Method::GET,
    };

    let mut request_builder = client.request(method, &config.url); // Example with GET, adjust as needed

    // Add headers if provided
    if let Some(ref headers) = config.headers {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let header_name = match HeaderName::from_bytes(key.as_bytes()) {
                Ok(h) => h,
                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send>),
            };
            let header_value = match HeaderValue::from_str(value) {
                Ok(h) => h,
                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send>),
            };
            header_map.insert(header_name, header_value);
        }

        request_builder = request_builder.headers(header_map);
    }

    // Add body if provided
    if let Some(body) = &config.body {
        let body = if let Some(row) = data_row {
            replace_placeholders(body, &row)
        } else {
            body.to_string()
        };
        request_builder = request_builder.body(body);
    }

    let response = request_builder
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

    Ok(response)
}

pub async fn perform_load_test(config: &LoadTestConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Load data if file is specified
    let data_rows = if let Some(ref file) = config.data_file {
        load_data(file)?
    } else {
        Vec::new()
    };

    let response_times = Arc::new(Mutex::new(Vec::new()));
    let success_count = Arc::new(Mutex::new(0usize));
    let error_count = Arc::new(Mutex::new(0usize));
    let requests_sent = Arc::new(Mutex::new(0usize));
    let responses_received = Arc::new(Mutex::new(0usize));

    let tasks: Vec<_> = (0..config.request_count)
        .map(|index| {
            let config = config.clone();
            let response_times = Arc::clone(&response_times);
            let success_count = Arc::clone(&success_count);
            let error_count = Arc::clone(&error_count);
            let requests_sent = Arc::clone(&requests_sent);
            let responses_received = Arc::clone(&responses_received);
            let data_row = if data_rows.is_empty() {
                None
            } else {
                data_rows.get(index % data_rows.len()).cloned()
            };

            tokio::spawn(async move {
                {
                    let mut sent = requests_sent.lock().await;
                    *sent += 1;

                    // Display progress at intervals or when all requests are sent
                    if *sent % 1 == 0 || *sent == config.request_count {
                        display_progress(*sent, 0);
                    }
                }

                let start_time = Instant::now();

                match send_request(&config, &data_row).await {
                    Ok(response) => {
                        // println!("{:?}", response);
                        if response.status().is_success() {
                            let mut success = success_count.lock().await;
                            *success += 1;
                        } else {
                            let mut errors = error_count.lock().await;
                            *errors += 1;
                        }
                    }
                    Err(_) => {
                        // println!("{:?}", err);
                        let mut errors = error_count.lock().await;
                        *errors += 1;
                    }
                }

                let elapsed = start_time.elapsed();
                let mut times = response_times.lock().await;
                times.push(elapsed);

                {
                    let mut received = responses_received.lock().await;
                    *received += 1;

                    if *received % 1 == 0 || *received == config.request_count {
                        display_progress(config.request_count, *received);
                    }
                }
            })
        })
        .collect();

    // Wait for all tasks to complete
    join_all(tasks).await;

    let total_duration: Duration = response_times.lock().await.iter().sum();
    let average_duration = total_duration / config.request_count as u32;

    let success = *success_count.lock().await;
    let errors = *error_count.lock().await;

    let success_percentage = (success as f64 / config.request_count as f64) * 100.0;
    let error_percentage = (errors as f64 / config.request_count as f64) * 100.0;

    let response_times_locked = response_times.lock().await;
    let duration = Duration::new(0, 0);
    let min_duration = response_times_locked.iter().min().unwrap_or(&duration);
    let max_duration = response_times_locked.iter().max().unwrap_or(&duration);

    // Final statistics
    println!("Total Requests: {}", config.request_count);
    println!("Successful Requests: {}", success);
    println!("Failed Requests: {}", errors);
    println!("Success Percentage: {:.2}%", success_percentage);
    println!("Failure Percentage: {:.2}%", error_percentage);
    println!("Average Response Time: {:?}", average_duration);
    println!("Minimum Response Time: {:?}", min_duration);
    println!("Maximum Response Time: {:?}", max_duration);

    Ok(())
}

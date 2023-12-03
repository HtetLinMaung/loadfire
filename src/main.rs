use clap::Parser;
use config::load_config;
use http::perform_load_test;

mod config;
mod data;
mod http;
mod utils;

/// Loadfire load testing tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the YAML configuration file
    #[clap(short, long, value_parser)]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match load_config(&args.config) {
        Ok(config) => {
            if let Err(e) = perform_load_test(&config).await {
                eprintln!("Error during load test: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read config: {}", e),
    }
}

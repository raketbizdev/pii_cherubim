mod log_processor;
mod file_utils;
mod pii_sanitizer;

use log_processor::LogProcessor;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_processor = LogProcessor::new("/Users/cmnopal/Desktop/testlog".to_string());

    log_processor.process_logs().await?;

    Ok(())
}
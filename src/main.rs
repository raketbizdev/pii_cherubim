mod log_processor;
mod file_utils;
mod pii_sanitizer;

use log_processor::LogProcessor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let log_processor = LogProcessor::new("/Users/cmnopal/Desktop/testlog".to_string());

    // No need for `await`, as this is a synchronous call
    log_processor.process_logs()?;

    Ok(())
}

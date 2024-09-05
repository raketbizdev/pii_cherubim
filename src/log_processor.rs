use crate::file_utils::find_log_files;
use crate::pii_sanitizer::sanitize_pii;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::path::PathBuf;
use std::fs;
use std::error::Error;

// Make the struct and its methods public so they can be tested
pub struct LogProcessor {
    pub log_directory: String,
}

impl LogProcessor {
    pub fn new(log_directory: String) -> Self {
        LogProcessor { log_directory }
    }

    pub async fn process_logs(&self) -> Result<(), Box<dyn Error>> {
        let log_files = find_log_files(&self.log_directory)?;

        if log_files.is_empty() {
            println!("No log files found.");
            return Ok(());
        }

        for log_file in log_files {
            println!("Monitoring file: {}", log_file.to_str().unwrap());

            if let Err(e) = self.monitor_logs(&log_file).await {
                eprintln!("Error monitoring logs in {}: {:?}", log_file.to_str().unwrap(), e);
            }

            if let Err(e) = self.process_historical_logs(&log_file) {
                eprintln!("Error sanitizing historical logs in {}: {:?}", log_file.to_str().unwrap(), e);
            }
        }

        Ok(())
    }

    pub async fn monitor_logs(&self, log_file: &PathBuf) -> io::Result<()> {
        let file_path = log_file.to_str().unwrap();
        let file = File::open(file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            match sanitize_pii(&line) {
                Ok(masked_line) => {
                    println!("Sanitized Log: {}", masked_line);
                }
                Err(e) => {
                    eprintln!("Error processing log line: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn process_historical_logs(&self, log_file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let log_path = log_file.to_str().unwrap();
        let logs = fs::read_to_string(log_path)?;
        let sanitized_logs = sanitize_pii(&logs)?;
        fs::write(log_path, sanitized_logs)?;
        Ok(())
    }
}
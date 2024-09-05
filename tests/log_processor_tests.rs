use pii_cherubim::log_processor::LogProcessor;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use tokio;

#[tokio::test]
async fn test_log_processing() {
    let tmp_dir = env::temp_dir().join("test_log_processing");
    fs::create_dir_all(&tmp_dir).unwrap();
    let log_file_path = tmp_dir.join("app_log.log");
    let mut log_file = File::create(&log_file_path).unwrap();
    writeln!(log_file, "User john.doe@example.com logged in").unwrap();

    let processor = LogProcessor::new(tmp_dir.to_str().unwrap().to_string());
    processor.process_logs().await.unwrap();

    fs::remove_dir_all(&tmp_dir).unwrap();
}
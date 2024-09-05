use pii_cherubim::file_utils::find_log_files;
use std::env;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_find_log_files() {
    let tmp_dir = env::temp_dir().join("test_logs");
    fs::create_dir_all(&tmp_dir).unwrap();

    let log_file_path = tmp_dir.join("app_log.log");
    let mut log_file = File::create(&log_file_path).unwrap();
    writeln!(log_file, "This is a test log file").unwrap();

    let log_files = find_log_files(tmp_dir.to_str().unwrap()).unwrap();
    assert_eq!(log_files.len(), 1);
    assert_eq!(log_files[0], log_file_path);

    fs::remove_dir_all(&tmp_dir).unwrap();
}
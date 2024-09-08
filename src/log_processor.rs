use crate::file_utils::find_log_files;
use crate::pii_sanitizer::sanitize_pii;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Seek, SeekFrom, Write}; // Import BufRead and Write for reading/writing to the file
use std::error::Error;

pub struct LogProcessor {
    pub log_directory: String,
}

impl LogProcessor {
    pub fn new(log_directory: String) -> Self {
        LogProcessor { log_directory }
    }

    pub fn process_logs(&self) -> Result<(), Box<dyn Error>> {
        let log_files = find_log_files(&self.log_directory)?;

        if log_files.is_empty() {
            println!("No log files found.");
            return Ok(());
        }

        for log_file in log_files {
            println!("Processing historical logs for file: {}", log_file.to_str().unwrap());

            if let Err(e) = self.process_historical_logs(&log_file) {
                eprintln!("Error sanitizing historical logs in {}: {:?}", log_file.to_str().unwrap(), e);
            }

            self.monitor_log_changes(log_file)?;
        }

        Ok(())
    }

    // Monitor real-time changes and track file offset to process new log entries
    pub fn monitor_log_changes(&self, log_file: PathBuf) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
        watcher.watch(&log_file, RecursiveMode::NonRecursive)?;

        println!("Monitoring real-time changes in: {}", log_file.to_str().unwrap());

        // Track the offset to only read newly added lines
        let mut last_offset: u64 = 0;

        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    if let EventKind::Modify(_) = event.kind {
                        if let Err(e) = self.process_new_logs(&log_file, &mut last_offset) {
                            eprintln!("Error processing new logs: {:?}", e);
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {:?}", e);
                }
                Err(e) => {
                    eprintln!("Receiver error: {:?}", e);
                }
            }
        }
    }

    // Process only the new log entries since the last read (based on offset)
    pub fn process_new_logs(&self, log_file: &PathBuf, last_offset: &mut u64) -> Result<(), Box<dyn Error>> {
        let file_path = log_file.to_str().unwrap();
        
        // Open the file for reading and writing
        let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

        // Seek to the last read position (offset)
        file.seek(SeekFrom::Start(*last_offset))?;
        let reader = BufReader::new(file.try_clone()?);  // Clone the file handle for reading

        let mut sanitized_content = String::new();  // Store sanitized logs

        for line in reader.lines() {
            let line = line?;
            match sanitize_pii(&line) {
                Ok(masked_line) => {
                    println!("Sanitized Log: {}", masked_line);
                    sanitized_content.push_str(&masked_line); // Store sanitized line
                    sanitized_content.push('\n'); // Add newline to match original log format
                }
                Err(e) => {
                    eprintln!("Error processing log line: {}", e);
                }
            }
        }

        // Seek to the last read position again for writing sanitized content
        let mut write_file = OpenOptions::new().write(true).open(file_path)?;
        write_file.seek(SeekFrom::Start(*last_offset))?;
        write_file.write_all(sanitized_content.as_bytes())?; // Write sanitized logs back to the file

        // Update the last read offset using the newly written data
        *last_offset = write_file.metadata()?.len();

        Ok(())
    }

    // Process the entire file for historical logs
    pub fn process_historical_logs(&self, log_file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let log_path = log_file.to_str().unwrap();
        let logs = std::fs::read_to_string(log_path)?;
        let sanitized_logs = sanitize_pii(&logs)?;
        std::fs::write(log_path, sanitized_logs)?;
        Ok(())
    }
}
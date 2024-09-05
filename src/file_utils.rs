use std::error::Error;
use std::path::PathBuf;
use walkdir::WalkDir;

// Make this function public so it can be tested
pub fn find_log_files(starting_directory: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut log_files = Vec::new();

    for entry in WalkDir::new(starting_directory)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_lowercase();
            if file_name.contains("log") && file_name.ends_with(".log") {
                log_files.push(path.to_path_buf());
            }
        }
    }

    Ok(log_files)
}
extern crate regex;
use regex::Regex;
use std::error::Error;

pub fn sanitize_pii(log_message: &str) -> Result<String, Box<dyn Error>> {
    // Improved regex pattern for matching common email addresses
    let email_regex = Regex::new(r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})")?;

    // Replace all detected email addresses with the mask "****@*****.com"
    let masked_message = email_regex.replace_all(log_message, "****@*****.com");

    Ok(masked_message.to_string())
}

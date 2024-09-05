extern crate regex;
use regex::Regex;
use std::error::Error;

// Make this function public so it can be tested
pub fn sanitize_pii(log_message: &str) -> Result<String, Box<dyn Error>> {
    let email_regex = Regex::new(r"([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})")?;
    let masked_message = email_regex.replace_all(log_message, "****@*****.com");
    Ok(masked_message.to_string())
}
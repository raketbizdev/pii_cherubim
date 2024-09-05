use pii_cherubim::pii_sanitizer::sanitize_pii;

#[test]
fn test_sanitize_email() {
    let log = "User john.doe@example.com logged in";
    let sanitized_log = sanitize_pii(log).unwrap();
    assert_eq!(sanitized_log, "User ****@*****.com logged in");
}

#[test]
fn test_no_pii_in_log() {
    let log = "System started successfully";
    let sanitized_log = sanitize_pii(log).unwrap();
    assert_eq!(sanitized_log, log);  // No PII, log should remain the same
}

#[test]
fn test_multiple_emails() {
    let log = "Emails: alice@example.com, bob@example.org";
    let sanitized_log = sanitize_pii(log).unwrap();
    assert_eq!(sanitized_log, "Emails: ****@*****.com, ****@*****.com");
}
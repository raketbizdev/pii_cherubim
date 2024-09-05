
# PII Cherubim

**PII Cherubim** is a Rust-based tool that monitors, sanitizes, and processes Personally Identifiable Information (PII) in log files. It operates in real-time to detect sensitive information, such as email addresses, and masks it to ensure compliance with data protection regulations. It also supports historical log sanitization and integration with a dashboard for auditing.

## Features

- **Real-time PII Detection**: Monitor log files in real-time and sanitize sensitive information such as email addresses.
- **Historical Log Sanitization**: Process existing log files and sanitize any detected PII.
- **System Log Search**: Automatically discover log files within a system based on file naming conventions.
- **Integration with External Dashboard**: Generate audit reports and send them to a remote dashboard for compliance tracking.
- **Lightweight and Asynchronous**: Built using asynchronous Rust for efficient log processing without blocking the system.

## Getting Started

### Prerequisites

- **Rust**: Ensure that you have Rust installed on your machine. You can install it via [rustup](https://rustup.rs/).
  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**: This tool is part of the Rust installation and will be used for building and testing the project.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/pii_cherubim.git
   cd pii_cherubim
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the application:

   ```bash
   cargo run
   ```

## How to Use

### Real-time Log Monitoring

To monitor a specific log file for PII, run the tool with the path to the log file:

```bash
cargo run -- log_path
```

Where `log_path` is the path to your log file.

### Historical Log Sanitization

If you need to sanitize logs retrospectively, you can run the tool with the following command:

```bash
cargo run --release -- log_path
```

This will process all logs in the specified path and sanitize any sensitive data.

### Sending Audit Reports

The tool can also send audit reports of the detected PII to a remote dashboard. Ensure that your dashboard URL is properly configured in the source code.

## Tests

To run the tests, use the following command:

```bash
cargo test
```

The tests include:
- Unit tests for PII sanitization
- Integration tests for log file processing and PII detection
- Real-time and historical log file handling

## Example

An example of a sanitized log:
```plaintext
User ****@*****.com logged in
```

The tool detects the email address `john.doe@example.com` and replaces it with asterisks to mask the sensitive information.

## Project Structure

```bash
.
├── src
│   ├── file_utils.rs        # Module for finding log files
│   ├── log_processor.rs     # Core logic for real-time and historical log processing
│   ├── lib.rs               # Exposes modules for testing and integration
│   ├── main.rs              # Application entry point
│   ├── pii_sanitizer.rs     # Module for PII detection and sanitization
└── tests                    # Integration tests
    ├── file_utils_tests.rs
    ├── log_processor_tests.rs
    └── pii_sanitizer_tests.rs
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or file an issue.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m "Add new feature"`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

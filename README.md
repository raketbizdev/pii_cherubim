
# PII Cherubim

**PII Cherubim** is an agent package designed for log security. It provides real-time monitoring, detection, and automatic sanitization of Personally Identifiable Information (PII) in log files, while ensuring compliance with data protection regulations. This lightweight package runs in the background on a server, requiring minimal user intervention. Once installed, it automatically handles Detection and Monitoring, Log Sanitization, and Compliance Auditing.

## Features

- **Real-time PII Detection and Monitoring**: Monitor log files for sensitive data such as emails, credit card numbers, etc., and sanitize them in real-time.
- **Historical Log Sanitization**: Process and sanitize existing log files that may contain PII.
- **Compliance and Auditing**: Generate audit reports and send them to a specified URL, ensuring compliance with regulations.
- **Lightweight and Background Operation**: Once installed, the package runs in the background and requires minimal resources.
- **Secure API Integration**: Data is securely sent to the specified URL using an API key and secret key to ensure authorized access.

## Getting Started

### Prerequisites

- **Rust**: Ensure that you have Rust installed on your server. You can install it via [rustup](https://rustup.rs/).

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**: This tool is part of the Rust installation and will be used for building and running the package.

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

3. Add your **API key** and **Secret key** to the environment variables:

   ```bash
   export PII_API_KEY="your-api-key"
   export PII_SECRET_KEY="your-secret-key"
   ```

4. Run the application in the background:

   ```bash
   nohup cargo run --release &
   ```

The package will now run in the background, monitoring your logs and sending reports securely.

### Configuration

- **API Key and Secret Key**: These are required to authenticate the data being sent to the remote URL for auditing purposes. Ensure you set them as environment variables on the server where the package is running:

   ```bash
   export PII_API_KEY="your-api-key"
   export PII_SECRET_KEY="your-secret-key"
   ```

- **Remote URL for Audit Reports**: Ensure that your dashboard URL is correctly configured in the source code or via an environment variable.

### How to Use

1. **Monitor Logs in Real-Time**: Once installed, the package will automatically monitor your log files and sanitize PII in real-time. Logs will be continuously sanitized in the background.

2. **Audit Reports**: The package generates compliance audit reports and sends them to the URL you provide, ensuring secure and authenticated transmission using the API and secret keys.

### Sending Audit Reports

Audit reports containing PII information that was detected and sanitized can be sent to the configured URL for compliance tracking. Make sure the API and secret keys are set, and the data will be transmitted securely.

### Tests

To run the tests, use the following command:

```bash
cargo test
```

### Example

An example of a sanitized log:
```plaintext
User ****@*****.com logged in
```

The tool detects the email address `john.doe@example.com` and replaces it with asterisks to mask the sensitive information.

### Project Structure

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

### Contributing

Contributions are welcome! Please feel free to submit a pull request or file an issue.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m "Add new feature"`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
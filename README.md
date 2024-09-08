
# PII Cherubim

**PII Cherubim** is a log security agent that monitors and sanitizes Personally Identifiable Information (PII) in real-time. It ensures compliance with data protection regulations by automatically detecting and masking sensitive information, such as email addresses, in log files. The package runs in the background and requires minimal user intervention once set up.

## Features

- **Real-time PII Detection**: Monitors log files for sensitive data like emails and automatically sanitizes it in real-time.
- **Historical Log Sanitization**: Sanitizes existing logs containing PII.
- **Background Operation**: Runs seamlessly in the background, requiring minimal system resources.
- **Secure Data Transmission**: Integrates securely with external auditing endpoints using API keys for authorization.

## Getting Started

### Prerequisites

To run PII Cherubim, you'll need:

- **Rust toolchain**: Install Rust if not already installed by running:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

- **Precompiled binary** (optional): You can use a precompiled binary for your platform, available [here](https://github.com/raketbizdev/pii_cherubim/releases).

Alternatively, you can compile the project from source using Rust.

### Installation (from Source)

1. Clone the repository:

   ```bash
   git clone https://github.com/raketbizdev/pii_cherubim.git
   cd pii_cherubim
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

3. The executable will be in the `target/release` directory. You can run it directly:

   ```bash
   ./target/release/pii_cherubim
   ```

### Running the Application

#### 1. Run in the Background

To start monitoring a specific log file in the background, run the following command:

```bash
nohup ./target/release/pii_cherubim --log /path/to/log/file &
```

This will start the PII Cherubim process, which will continuously monitor and sanitize PII in the specified log file.

#### 2. Real-Time Log Monitoring

When running the application, PII Cherubim will continuously monitor the specified log file for changes. If any sensitive information like email addresses is detected, it will automatically sanitize it and write the changes back to the log file.

Example:
```bash
./pii_cherubim --log /path/to/log/file
```

The application will sanitize any detected email addresses like `john.doe@example.com`, replacing it with `****@*****.com`.

### How to Configure

#### API Key and Secret Key (Optional)

If you're sending audit reports to a remote server, configure the API key and secret key as environment variables:

```bash
export PII_API_KEY="your-api-key"
export PII_SECRET_KEY="your-secret-key"
```

This ensures secure transmission of audit logs to a remote URL.

### Example Logs

Original log entry:

```plaintext
[INFO] 2024-09-08 10:15:32 - User john.doe@example.com successfully logged in.
```

After PII sanitization:

```plaintext
[INFO] 2024-09-08 10:15:32 - User ****@*****.com successfully logged in.
```

### Cleaning Historical Logs

To process historical log files that may contain PII, PII Cherubim will automatically sanitize the existing log files as part of its operation. You do not need to run separate commands to sanitize historical data.
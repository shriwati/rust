# DB Calls - Rust Project

A Rust project for database calls with ClickHouse integration and data access object (DAO) pattern.

## Project Structure

```
db_calls/
├── src/main.rs              # Main application entry point
├── dao/                     # Data Access Object library
│   ├── src/lib.rs          # ClickHouse connection implementation
│   └── Cargo.toml          # DAO dependencies
├── log/                     # Logging directory
├── log_config.yaml         # Logging configuration
└── Cargo.toml              # Main project dependencies
```

## DAO Library Features

The DAO library provides ClickHouse database connectivity with:

- **ClickHouseConfig**: Configuration structure for connection parameters
- **ClickHouseConnection**: Main connection management with async support
- **Connection Functions**:
  - `create_connection()` - Default local connection
  - `create_connection_with_config()` - Custom configuration
  - `test_connection()` - Connectivity verification

## Dependencies

### Main Project
- `log` - Logging functionality
- `dao` - Local DAO library
- `env_logger` - Environment-based logging

### DAO Library
- `clickhouse` - ClickHouse client
- `tokio` - Async runtime
- `serde` - Serialization support
- `anyhow` - Error handling

## Common Commands

### Build Commands
```bash
# Build entire project
cargo build

# Build DAO library only
cargo build -p dao

# Build with release optimization
cargo build --release
```

### Test Commands
```bash
# Run all tests
cargo test

# Run DAO library tests only
cargo test -p dao

# Run tests with output
cargo test -- --nocapture
```

### Development Commands
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Clean build artifacts
cargo clean
```

### ClickHouse Setup

For local development, ensure ClickHouse is running:

```bash
# Default connection settings:
# Host: localhost
# Port: 8123
# Database: default
# Username: default
# Password: (empty)
```

## Usage Example

```rust
use dao::{create_connection, ClickHouseConfig, create_connection_with_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Using default local connection
    let conn = create_connection().await?;

    // Test the connection
    let is_connected = conn.test_connection().await?;
    println!("Connected: {}", is_connected);

    // Using custom configuration
    let config = ClickHouseConfig {
        host: "localhost".to_string(),
        port: 8123,
        database: "my_db".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
    };

    let custom_conn = create_connection_with_config(config).await?;

    Ok(())
}
```

## Git Information

- **Current Branch**: db_calls
- **Main Branch**: main
- **Edition**: 2024
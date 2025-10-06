# DB Calls - Web API Project

A Rust workspace project for database calls with ClickHouse integration and web API functionality.

## Project Structure

```
db_calls/
├── Cargo.toml              # Workspace configuration
├── api_call/               # API library
│   ├── src/lib.rs         # API functions
│   └── Cargo.toml         # API dependencies
└── CLAUDE.md              # Project documentation
```

## Workspace Configuration

The workspace uses:
- **Edition**: 2024
- **Resolver**: Version 3 (required for edition 2024)

## API Call Library

The `api_call` library provides basic API functionality with:

### Functions
- `helloworld(name: &str) -> String` - Returns a welcome message with the provided name

### Dependencies
- `tokio` (v1.42) - Async runtime with full features
- `serde` (v1.0) - Serialization/deserialization with derive support
- `serde_json` (v1.0) - JSON support
- `anyhow` (v1.0) - Error handling

## Common Commands

### Build Commands
```bash
# Build entire workspace
cargo build

# Build api_call library only
cargo build -p api_call

# Build with release optimization
cargo build --release
```

### Test Commands
```bash
# Run all tests
cargo test

# Run api_call tests only
cargo test -p api_call

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

## Git Information

- **Current Branch**: webapi
- **Main Branch**: main
- **Edition**: 2024

## Usage Example

```rust
use api_call::helloworld;

fn main() {
    let message = helloworld("Alice");
    println!("{}", message); // Output: welcome to Rust Alice
}
```

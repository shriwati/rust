# WebAPI - Rust Web API Project

A Rust web API project with Actix Web framework and modular library structure.

## Project Structure

```
webapi/
├── Cargo.toml              # Main package and workspace configuration
├── src/
│   └── main.rs            # Web API server entry point
├── api_call/              # API library
│   ├── src/lib.rs         # API functions
│   └── Cargo.toml         # API dependencies
└── CLAUDE.md              # Project documentation
```

## Workspace Configuration

The workspace uses:
- **Edition**: 2024
- **Resolver**: Version 3 (required for edition 2024)
- **Main Package**: webapi (binary)
- **Library Package**: api_call

## WebAPI Binary

The main binary provides a REST API server using Actix Web:

### Endpoints
- `GET /helloworld?name={name}` - Returns a welcome message for the provided name

### Dependencies
- `actix-web` (v4) - Web framework
- `tokio` (v1.42) - Async runtime with full features
- `serde` (v1.0) - Serialization/deserialization
- `api_call` - Local library for business logic

## API Call Library

The `api_call` library provides business logic functions:

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
# Build entire project
cargo build

# Build with release optimization
cargo build --release

# Build api_call library only
cargo build -p api_call
```

### Run Commands
```bash
# Run the web server
cargo run

# Run with release optimization
cargo run --release
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

## Running the Server

Start the web server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

### Testing the API

```bash
# Using curl
curl "http://127.0.0.1:3000/helloworld?name=Alice"

# Expected response
welcome to Rust Alice
```

## Git Information

- **Current Branch**: webapi
- **Main Branch**: main
- **Edition**: 2024

## Usage Example

### Using the API Library Directly

```rust
use api_call::helloworld;

fn main() {
    let message = helloworld("Alice");
    println!("{}", message); // Output: welcome to Rust Alice
}
```

### Using the Web API

```rust
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloParams {
    name: String,
}

#[get("/helloworld")]
async fn hello_world(query: web::Query<HelloParams>) -> impl Responder {
    api_call::helloworld(&query.name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello_world)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
```

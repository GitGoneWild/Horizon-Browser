# Development Guide

## Getting Started

Welcome to Horizon Browser development! This guide will help you set up your development environment and understand the development workflow.

## Development Setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install Development Tools

```bash
# Rustfmt for code formatting
rustup component add rustfmt

# Clippy for linting
rustup component add clippy

# Rust Analyzer for IDE support
rustup component add rust-analyzer
```

### 3. Clone and Build

```bash
git clone https://github.com/GitGoneWild/Horizon-Browser.git
cd Horizon-Browser
cargo build
```

## Development Workflow

### Code Style

We follow the standard Rust style guidelines:

```bash
# Format all code
cargo fmt --all

# Check formatting without modifying
cargo fmt --all -- --check
```

### Linting

Run Clippy to catch common mistakes:

```bash
# Run clippy on all crates
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p horizon-engine

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name
```

### Running the Browser

```bash
# Debug mode (with logs)
RUST_LOG=horizon=debug cargo run

# Release mode
cargo run --release

# With custom log level
RUST_LOG=horizon=trace,warn cargo run
```

## Project Structure

```
horizon-browser/
├── engine/          # Rendering engine
│   ├── src/
│   │   ├── lib.rs
│   │   ├── renderer.rs
│   │   └── view.rs
│   └── Cargo.toml
├── ui/              # User interface
├── networking/      # Network layer
├── storage/         # Storage system
├── extensions/      # Extension framework
├── sandbox/         # Security sandbox
└── launcher/        # Main application
```

## Coding Conventions

### Naming

- **Modules**: `snake_case`
- **Types**: `PascalCase`
- **Functions**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Traits**: `PascalCase`

### Documentation

All public APIs must be documented:

```rust
/// Brief description of the function.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// let result = my_function(42);
/// ```
pub fn my_function(param: i32) -> String {
    // Implementation
}
```

### Error Handling

Use `anyhow::Result` for functions that can fail:

```rust
use anyhow::Result;

pub fn operation() -> Result<()> {
    // ...
    Ok(())
}
```

### Async Functions

Use `async`/`await` for I/O operations:

```rust
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MyTrait {
    async fn async_operation(&self) -> Result<()>;
}
```

## Debugging

### Logging

Use the `tracing` crate for logging:

```rust
use tracing::{info, debug, warn, error};

info!("Information message");
debug!("Debug message");
warn!("Warning message");
error!("Error message");
```

### Debug Build

Debug builds include:
- Debug symbols
- Better backtraces
- No optimizations

### Backtrace

Enable backtraces for panics:

```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run  # Full backtrace
```

## IDE Setup

### Visual Studio Code

Recommended extensions:
- rust-analyzer
- CodeLLDB (debugging)
- crates (dependency management)

### IntelliJ IDEA / CLion

Install the Rust plugin for full IDE support.

## Adding Dependencies

1. Add to `Cargo.toml`:
```toml
[dependencies]
new-crate = "1.0"
```

2. Use in code:
```rust
use new_crate::SomeType;
```

3. Update workspace dependencies if needed

## Adding New Features

1. Create a new module or crate
2. Write tests for the feature
3. Implement the feature
4. Document the API
5. Run tests and linting
6. Submit a pull request

## Performance Profiling

### Benchmarking

Use criterion for benchmarks:

```rust
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn benchmark_function(c: &mut Criterion) {
        c.bench_function("my_function", |b| {
            b.iter(|| my_function(black_box(42)))
        });
    }

    criterion_group!(benches, benchmark_function);
    criterion_main!(benches);
}
```

### Profiling

Use flamegraph for CPU profiling:

```bash
cargo install flamegraph
cargo flamegraph --bin horizon
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for:
- Code review process
- Pull request guidelines
- Commit message format
- Issue reporting

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

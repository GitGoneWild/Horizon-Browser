# Horizon-Browser

A lightweight, fast, privacy-focused desktop browser built in Rust.

## Overview

Horizon Browser is a modern web browser built from the ground up with privacy, performance, and extensibility in mind. Written in Rust, it provides a secure and efficient browsing experience across Windows, macOS, and Linux.

## Features

- **Privacy First**: Built-in tracking protection, Do Not Track, and third-party cookie blocking
- **Lightweight**: Minimal resource usage with optimized performance
- **Cross-Platform**: Native support for Windows, macOS, and Linux
- **Extensible**: Powerful extension framework for customization
- **Modern UI**: Dark theme by default with clean, intuitive design
- **Secure**: Process sandboxing and strict security policies

## Architecture

Horizon Browser is built as a modular workspace with the following crates:

- **engine**: Core rendering engine and view management
- **ui**: User interface layer with window management
- **networking**: HTTP client and network operations
- **storage**: User data, settings, and profile management
- **extensions**: Extension framework and plugin system
- **sandbox**: Process isolation and security policies
- **launcher**: Main application entry point

For detailed architecture documentation, see [docs/architecture/](docs/architecture/).

## Building

### Prerequisites

- Rust 1.91 or later
- Cargo

### Build Steps

```bash
# Clone the repository
git clone https://github.com/GitGoneWild/Horizon-Browser.git
cd Horizon-Browser

# Build the project
cargo build --release

# Run the browser
cargo run --release
```

## Development

### Running Tests

```bash
cargo test --workspace
```

### Linting

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Project Structure

```
horizon/
├── Cargo.toml              # Workspace configuration
├── engine/                 # Rendering engine
├── ui/                     # User interface
├── networking/             # Network layer
├── storage/                # Data storage
├── extensions/             # Extension system
├── sandbox/                # Security sandbox
├── launcher/               # Main entry point
├── assets/                 # Icons and themes
└── docs/                   # Documentation
```

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Documentation

- [Architecture Overview](docs/architecture/README.md)
- [API Documentation](docs/api/README.md)
- [Build Instructions](docs/BUILD.md)
- [Development Guide](docs/DEVELOPMENT.md)

## Status

🚧 **Under Active Development** - Horizon Browser is in early development. The current version provides a basic working browser window with foundational systems in place.

## Roadmap

- [ ] MVP: Basic browsing functionality
- [ ] Alpha: Full rendering engine integration
- [ ] Beta: Extension support and theming
- [ ] Release: Stable public release


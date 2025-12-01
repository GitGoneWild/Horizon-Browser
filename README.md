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

✅ **Enhanced MVP Complete** - Horizon Browser now includes comprehensive features including full settings management, keyboard shortcuts, enhanced navigation, and a polished dark theme UI. The browser is functional and ready for testing!

## Current Features

### Core Browsing
- ✅ Multiple tab management (open, close, switch tabs)
- ✅ Tab loading indicators
- ✅ Address bar with URL navigation and search
- ✅ Configurable search engines (DuckDuckGo, Google, Bing, Brave)
- ✅ Back/forward/reload/stop/home navigation
- ✅ Navigation history per tab
- ✅ Enhanced home page (about:home) with modern design
- ✅ Modern dark theme UI (GitHub-inspired)
- ✅ Cross-platform desktop support

### Settings & Configuration
- ✅ Full settings page (about:settings) with 5 panels:
  - **General**: Homepage, search engine, startup options
  - **Privacy**: Tracking protection, Do Not Track, cookie blocking, HTTPS-only mode
  - **Appearance**: Theme selection, font size, bookmarks bar
  - **Downloads**: Download directory, save preferences
  - **Advanced**: Developer tools, hardware acceleration, experimental features
- ✅ Settings persistence (ready for storage integration)
- ✅ Privacy-first default settings

### Keyboard Shortcuts
- ✅ Ctrl+T: New tab
- ✅ Ctrl+W: Close tab
- ✅ Ctrl+R / F5: Reload page
- ✅ Alt+Left: Go back
- ✅ Alt+Right: Go forward
- ✅ Alt+Home: Go to home page
- ✅ Ctrl+L: Focus address bar

### UI/UX Enhancements
- ✅ Modern icons with tooltips showing shortcuts
- ✅ Stop/Reload toggle button
- ✅ Home button for quick access
- ✅ Settings button for easy configuration
- ✅ Improved tab styling with rounded corners and better spacing
- ✅ Loading indicators in tabs
- ✅ Enhanced home page with feature showcase
- ✅ Better color scheme and visual hierarchy

See [FEATURES.md](docs/FEATURES.md) for detailed feature documentation.

## Roadmap

- [x] MVP: Basic browsing functionality
- [x] Enhanced MVP: Settings, keyboard shortcuts, polished UI
- [ ] Alpha: Full rendering engine integration (WebView/Servo)
- [ ] Beta: Extension support and advanced theming
- [ ] Release: Stable public release


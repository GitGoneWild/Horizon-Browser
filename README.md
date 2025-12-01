# Horizon Browser

A lightweight, fast, privacy-focused desktop browser built in Rust with advanced networking and security features.

## Overview

Horizon Browser is a modern web browser built from the ground up with privacy, performance, and extensibility in mind. Written in Rust, it provides a secure and efficient browsing experience across Windows, macOS, and Linux. The browser features advanced networking capabilities, built-in VPN support, comprehensive password management, and Firefox-compatible extension support.

## Key Features

### Core Browsing
- **Multi-Tab Support**: Full tab management with keyboard shortcuts
- **Smart Address Bar**: SSL indicators, URL autocomplete, and integrated search
- **Search Engines**: DuckDuckGo (default), Google, Bing, and Brave Search
- **Navigation**: Back/forward history, reload, home, and settings access
- **Modern UI**: GitHub-inspired dark theme with polished interface

### Privacy & Security
- **Privacy First**: Built-in tracking protection, Do Not Track, and third-party cookie blocking
- **HTTPS-Only Mode**: Optional enforcement of secure connections
- **Password Manager**: Secure local credential storage with multi-account support
- **Data Control**: Clear browsing data on exit
- **Process Sandboxing**: Isolated processes for enhanced security

### Advanced Networking
- **DNS Control**: Switch between System, Google DNS, Cloudflare, or custom DNS servers
- **VPN Management**: Per-browser VPN with .ovpn file support and proxy configuration
- **Speed Test**: Built-in network speed testing with real-time monitoring
- **Network Dashboard**: Live statistics for download/upload speeds and latency

### Extensions & Customization
- **Firefox Extension Support**: Install and manage Firefox-compatible extensions
- **Extension Dashboard**: Easy installation, removal, and settings management
- **Theme Support**: Dark and light theme options
- **Font Customization**: Adjustable font sizes

### Cross-Platform
- **Windows**: Native support for Windows 10/11
- **macOS**: Full macOS compatibility
- **Linux**: Support for major Linux distributions
- **Lightweight**: Minimal resource usage with optimized performance

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

## Getting Started

### Prerequisites

Before building Horizon Browser, ensure you have the following installed:

- **Rust**: Version 1.91 or later ([Install Rust](https://rustup.rs/))
- **Cargo**: Comes with Rust installation
- **Build Tools**: Platform-specific build tools
  - **Linux**: `build-essential`, `pkg-config`, `libssl-dev`
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: MSVC Build Tools or MinGW-w64

### Local Setup and Build Instructions

#### 1. Clone the Repository

```bash
git clone https://github.com/GitGoneWild/Horizon-Browser.git
cd Horizon-Browser
```

#### 2. Install Dependencies (Linux only)

On Linux systems, install required development packages:

```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install gcc pkg-config openssl-devel

# Arch Linux
sudo pacman -S base-devel pkg-config openssl
```

#### 3. Build the Project

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized for performance)
cargo build --release
```

The release build is recommended for regular use as it includes optimizations like LTO and is significantly faster.

#### 4. Run the Browser

```bash
# Run debug build
cargo run

# Run release build (recommended)
cargo run --release

# Or run the compiled binary directly
./target/release/horizon  # Linux/macOS
.\target\release\horizon.exe  # Windows
```

### First-Time Setup

On first launch, Horizon Browser will:
1. Create a data directory in your system's application data folder
2. Initialize default settings with privacy-first defaults
3. Open the home page (about:home)

**Data Directory Locations:**
- **Windows**: `%APPDATA%\Horizon\`
- **macOS**: `~/Library/Application Support/Horizon/`
- **Linux**: `~/.local/share/Horizon/`

## Development

### Development Workflow

```bash
# Format code
cargo fmt --all

# Check code without building
cargo check --workspace

# Run clippy linter
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all tests
cargo test --workspace

# Run with logging
RUST_LOG=horizon=debug cargo run
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p horizon-networking

# Run tests with output
cargo test --workspace -- --nocapture

# Run a specific test
cargo test test_name
```

### Project Structure

```
horizon-browser/
├── Cargo.toml              # Workspace configuration
├── Cargo.lock              # Dependency lockfile
│
├── launcher/               # Main entry point and application lifecycle
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   └── app.rs         # Main application state and initialization
│   └── Cargo.toml
│
├── engine/                 # Core rendering engine
│   ├── src/
│   │   ├── lib.rs         # Engine API
│   │   ├── renderer.rs    # Rendering pipeline
│   │   └── view.rs        # View management
│   └── Cargo.toml
│
├── ui/                     # User interface layer
│   ├── src/
│   │   ├── lib.rs         # UI manager
│   │   ├── window.rs      # Main browser window and UI logic
│   │   ├── tabs.rs        # Tab management
│   │   ├── settings.rs    # Settings UI and persistence
│   │   └── theme.rs       # Theme definitions
│   └── Cargo.toml
│
├── networking/             # Networking layer
│   ├── src/
│   │   ├── lib.rs         # Network manager
│   │   ├── client.rs      # HTTP client
│   │   ├── dns.rs         # DNS resolution
│   │   ├── vpn.rs         # VPN management
│   │   ├── speedtest.rs   # Network speed testing
│   │   ├── request.rs     # HTTP request building
│   │   └── response.rs    # HTTP response handling
│   └── Cargo.toml
│
├── storage/                # Data storage and persistence
│   ├── src/
│   │   ├── lib.rs         # Storage manager
│   │   ├── settings.rs    # Settings persistence
│   │   ├── profile.rs     # User profile management
│   │   ├── userdata.rs    # Browser data (cache, history)
│   │   └── secure.rs      # Password and secure storage
│   └── Cargo.toml
│
├── extensions/             # Extension framework
│   ├── src/
│   │   ├── lib.rs         # Extension manager
│   │   ├── manifest.rs    # Extension manifest parsing
│   │   ├── loader.rs      # Extension loading
│   │   └── registry.rs    # Extension registry
│   └── Cargo.toml
│
├── sandbox/                # Security and isolation
│   ├── src/
│   │   ├── lib.rs         # Sandbox manager
│   │   ├── policy.rs      # Security policies
│   │   └── isolation.rs   # Process isolation
│   └── Cargo.toml
│
├── assets/                 # Static assets
│   ├── icons/             # Application icons
│   └── themes/            # Theme definitions
│
└── docs/                   # Documentation
    ├── BUILD.md           # Build instructions
    ├── DEVELOPMENT.md     # Development guide
    ├── SETTINGS.md        # Settings documentation
    ├── SHORTCUTS.md       # Keyboard shortcuts
    ├── api/               # API documentation
    └── architecture/      # Architecture documentation
```

### Workspace Crates

Each crate in the workspace serves a specific purpose:

1. **horizon-launcher**: Application entry point, coordinates all subsystems
2. **horizon-engine**: Rendering engine and view management
3. **horizon-ui**: User interface, window management, and visual components
4. **horizon-networking**: HTTP client, DNS, VPN, and network utilities
5. **horizon-storage**: Settings, profiles, user data, and secure storage
6. **horizon-extensions**: Extension framework and plugin system
7. **horizon-sandbox**: Security policies and process isolation

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


# Building Horizon Browser

## Prerequisites

### Required

- **Rust**: Version 1.91 or later
- **Cargo**: Comes with Rust
- **Git**: For cloning the repository

### Platform-Specific Requirements

#### Windows
- Visual Studio Build Tools or Visual Studio 2019/2022
- Windows SDK

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### Linux
- Build essentials: `sudo apt install build-essential`
- Development libraries:
  ```bash
  # Debian/Ubuntu
  sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev

  # Fedora
  sudo dnf install libxcb-devel libxkbcommon-devel
  ```

## Quick Start

```bash
# Clone the repository
git clone https://github.com/GitGoneWild/Horizon-Browser.git
cd Horizon-Browser

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run the browser
cargo run --release
```

## Build Modes

### Debug Build
```bash
cargo build
```
- Faster compilation
- Includes debug symbols
- No optimizations
- Larger binary size

### Release Build
```bash
cargo build --release
```
- Slower compilation
- Full optimizations
- LTO enabled
- Smaller binary size
- Better performance

## Build Output

Compiled binaries are located in:
- Debug: `target/debug/horizon`
- Release: `target/release/horizon`

## Cross-Compilation

### Building for Windows from Linux
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### Building for macOS from Linux
```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

## Workspace Structure

The project uses a Cargo workspace with multiple crates:

```bash
# Build specific crate
cargo build -p horizon-engine

# Build all crates
cargo build --workspace

# Build with all features
cargo build --workspace --all-features
```

## Build Optimization

### Profile Customization

Edit `Cargo.toml` to customize build profiles:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Incremental Compilation

For faster development builds:

```bash
export CARGO_INCREMENTAL=1
cargo build
```

## Troubleshooting

### Common Issues

**Error: linker 'cc' not found**
- Install build tools for your platform (see prerequisites)

**Error: failed to run custom build command**
- Install required system libraries (see platform-specific requirements)

**Error: Package requires rustc 1.91 or later**
- Update Rust: `rustup update stable`

### Clean Build

If you encounter build issues:

```bash
# Clean build artifacts
cargo clean

# Rebuild
cargo build --release
```

## CI/CD

The project includes GitHub Actions workflows:
- `.github/workflows/ci.yml`: Build and test
- `.github/workflows/lint.yml`: Linting

## Build Time

Approximate build times (release mode):
- First build: 5-10 minutes
- Incremental: 1-2 minutes

Build time varies based on:
- CPU performance
- Available RAM
- SSD vs HDD
- Parallel compilation (`-j` flag)

## Advanced Options

### Parallel Builds
```bash
cargo build --release -j 8
```

### Verbose Output
```bash
cargo build --release --verbose
```

### Check Without Building
```bash
cargo check --workspace
```

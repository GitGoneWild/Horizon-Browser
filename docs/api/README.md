# API Documentation

## Horizon Browser API Reference

This document provides an overview of the public APIs exposed by each crate in the Horizon Browser workspace.

## Engine API (`horizon-engine`)

### Core Types

- `Engine` trait: Main engine interface
- `HorizonEngine`: Default engine implementation
- `Renderer`: Rendering pipeline
- `View`: View container
- `ViewManager`: Multi-view management

### Usage Example

```rust
use horizon_engine::{Engine, HorizonEngine};

#[tokio::main]
async fn main() {
    let mut engine = HorizonEngine::new();
    engine.initialize().await.unwrap();
    engine.render_frame().await.unwrap();
    engine.shutdown().await.unwrap();
}
```

## UI API (`horizon-ui`)

### Core Types

- `UIManager`: Main UI coordinator
- `Theme`: Theme configuration
- `ColorPalette`: Color definitions
- `WindowConfig`: Window settings
- `BrowserWindow`: Main window

### Usage Example

```rust
use horizon_ui::{UIManager, window::WindowConfig};

let mut ui = UIManager::new();
ui.initialize().unwrap();

let config = WindowConfig::default();
let window = BrowserWindow::new(config);
```

## Networking API (`horizon-networking`)

### Core Types

- `NetworkManager`: Network coordinator
- `HttpClient`: HTTP client
- `Request`: HTTP request
- `Response`: HTTP response
- `DnsResolver`: DNS resolution

### Usage Example

```rust
use horizon_networking::{NetworkManager, request::Request};

#[tokio::main]
async fn main() {
    let manager = NetworkManager::new().unwrap();
    let request = Request::get("https://example.com");
    let response = manager.client().send(request).await.unwrap();
}
```

## Storage API (`horizon-storage`)

### Core Types

- `StorageManager`: Storage coordinator
- `Settings`: Browser settings
- `Profile`: User profile
- `ProfileManager`: Profile management
- `SecureStorage`: Secure credential storage
- `UserDataManager`: User data management

### Usage Example

```rust
use horizon_storage::{StorageManager, settings::Settings};
use std::path::PathBuf;

let path = PathBuf::from("./data");
let mut storage = StorageManager::new(path).unwrap();
let settings = storage.settings();
```

## Extensions API (`horizon-extensions`)

### Core Types

- `Extension` trait: Extension interface
- `ExtensionManager`: Extension coordinator
- `Manifest`: Extension manifest
- `ExtensionRegistry`: Extension registry
- `ExtensionLoader`: Extension loader

### Usage Example

```rust
use horizon_extensions::{ExtensionManager, manifest::Manifest};

#[tokio::main]
async fn main() {
    let mut manager = ExtensionManager::new();
    manager.initialize().await.unwrap();
}
```

## Sandbox API (`horizon-sandbox`)

### Core Types

- `SandboxManager`: Sandbox coordinator
- `SecurityPolicy`: Security configuration
- `ProcessIsolator`: Process isolation
- `IsolationLevel`: Isolation levels

### Usage Example

```rust
use horizon_sandbox::{SandboxManager, policy::SecurityPolicy};

let mut sandbox = SandboxManager::new();
sandbox.initialize().unwrap();
let policy = SecurityPolicy::default();
```

## Error Handling

All APIs use `anyhow::Result` for error handling, providing rich error context and easy error propagation.

## Async Support

APIs that perform I/O operations are async and require a tokio runtime:

```rust
#[tokio::main]
async fn main() {
    // Async code here
}
```

## Thread Safety

All major components implement `Send + Sync` and can be safely shared across threads.

## Documentation Generation

Generate full API documentation with:

```bash
cargo doc --workspace --no-deps --open
```

//! # Horizon Browser Launcher
//!
//! Main entry point for the Horizon Browser application.

mod app;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize panic handler
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Horizon Browser panicked: {:?}", panic_info);
        tracing::error!("Panic occurred: {:?}", panic_info);
    }));

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "horizon=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Horizon Browser v{}", env!("CARGO_PKG_VERSION"));

    // Create and run the application
    let app = app::HorizonApp::new()?;
    app.run().await
}

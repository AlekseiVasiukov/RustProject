// ℹ️  This is the entry point of our application.
//     When you run `cargo run`, Rust executes the main() function here.
//
//     The startup sequence:
//     1. Initialize logging
//     2. Load configuration
//     3. Start the queue worker
//     4. Start the HTTP server

// Import our local modules
// The "mod" keyword tells Rust that this file exists and should be compiled
mod config;
mod client;
mod queue;
mod server;
mod types;

// Import external crates we need directly in main
use tracing::info;
use tracing_subscriber::EnvFilter;

// ============================================================================
// Main Function - Application Entry Point
// ============================================================================

/// The main entry point for our application.
/// 
/// This function:
/// 1. Sets up structured logging with tracing
/// 2. Loads the application configuration
/// 3. Starts the background queue worker
/// 4. Creates and starts the Axum HTTP server
/// 
/// # Panics
/// 
/// This function will panic if the server fails to start.
/// For a production app, you'd want to handle this more gracefully.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Step 1: Initialize the tracing subscriber.
    // This enables our info!(), warn!(), error!() macros to print logs.
    // EnvFilter::default().add_directive("info".parse()?) sets log level to "info".
    // You can change this to "debug" for more verbose logging.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    info!("Starting Game Engine application");

    // Step 2: Load configuration.
    // We use from_env() to check for environment variables first,
    // then fall back to default values.
    let config = config::Config::from_env()?;
    info!(?config, "Configuration loaded");

    // Step 3: Start the queue worker.
    // This spawns a background task and gives us a sender for the queue.
    // start_queue_worker is synchronous (non-async) since the spawn happens instantly.
    // The background task itself is async, but starting it returns right away.
    let (queue_sender, _llama_client) = queue::start_queue_worker(config.clone());
    info!("Queue worker started");

    // Step 4: Create the Axum router with all routes.
    let app = server::create_router(queue_sender);
    info!("Axum router created");

    // Step 5: Bind the server to the configured address.
    // axum::Server::bind() prepares the server but doesn't start it yet.
    let listener = tokio::net::TcpListener::bind(&config.axum_bind_address).await?;
    info!(address = %config.axum_bind_address, "Server listening");

    // Step 6: Start serving requests.
    // This blocks until the server is shut down (Ctrl+C or crash).
    // The .with_graceful_shutdown() ensures in-flight requests complete before exit.
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Returns a future that completes when a shutdown signal is received.
/// 
/// This listens for Ctrl+C (SIGINT) or termination signals (SIGTERM).
/// When received, it triggers graceful shutdown of the server.
/// 
/// Graceful shutdown means:
/// - Stop accepting new connections
/// - Wait for active requests to complete
/// - Then exit cleanly
async fn shutdown_signal() {
    // tokio::signal::ctrl_c() returns a future that completes when Ctrl+C is pressed.
    // The .await waits for that signal.
    tokio::signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
    
    info!("Shutdown signal received, stopping server...");
}

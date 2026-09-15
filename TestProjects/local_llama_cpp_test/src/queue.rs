// ℹ️  This file contains our queue worker system.
//     It uses Tokio's "mpsc" (multi-producer single-consumer) channels to
//     process requests in a background task.
//
//     How channels work (simple analogy):
//     Imagine a restaurant kitchen:
//     - Waiters (the server) take orders and put them on a ticket queue
//     - The cook (the worker) takes tickets one at a time and prepares food
//     - When food is ready, it goes back to the waiter for the customer
//
//     In our case:
//     - HTTP requests send queries into the channel
//     - The worker receives them and processes one at a time
//     - Results are sent back through a oneshot channel

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

// Import our custom types
use crate::types::{GameQueryResponse, QueueMessage};
use crate::client::LlamaClient;
use crate::config::Config;

// ============================================================================
// Queue Worker
// ============================================================================

/// Starts the background queue worker.
/// 
/// This function:
/// 1. Creates a channel for sending messages to the worker
/// 2. Spawns a background task that processes messages one at a time
/// 3. Returns the sender half of the channel
/// 
/// # Arguments
/// 
/// * `config` - Application configuration with delay settings
/// 
/// # Returns
/// 
/// A tuple containing:
/// - The `mpsc::Sender` for sending messages to the worker
/// - The `LlamaClient` that was created for making API calls
/// 
/// # Example
/// 
/// ```rust
/// let (tx, client) = start_queue_worker(config).await;
/// // Now you can use `tx` to send messages to the worker
/// ```
pub fn start_queue_worker(config: Config) -> (mpsc::Sender<QueueMessage>, LlamaClient) {
    // Create a new channel with a buffer size of 100.
    // The buffer can hold up to 100 messages before senders block.
    // Channel::bounded() creates a channel with a fixed capacity.
    let (tx, rx) = mpsc::channel::<QueueMessage>(100);

    // Create our llama.cpp client
    let llama_client = LlamaClient::new(config.llama_server_url.clone());

    // Clone the client before moving it into the worker task.
    // One copy stays here (returned to caller), one moves into the worker.
    let llama_client_for_worker = llama_client.clone();

    // Spawn a background task using tokio::spawn.
    // This runs asynchronously - it does NOT wait for the worker to finish.
    // The worker will run until it receives a Shutdown message or the app exits.
    tokio::spawn(async move {
        // The async move keyword tells Rust to move all captured variables
        // (config, llama_client_for_worker, rx) into this closure.
        run_queue_worker(rx, llama_client_for_worker, config).await;
    });

    // Return the sender and the original client.
    // The sender can now be used from anywhere to send messages to the worker.
    (tx, llama_client)
}

/// The main loop for the queue worker.
/// 
/// This runs in a background task and processes messages one at a time.
/// 
/// # Arguments
/// 
/// * `rx` - The receiver half of the mpsc channel
/// * `llama_client` - The client for making llama.cpp requests
/// * `config` - Configuration with simulation delay settings
async fn run_queue_worker(
    mut rx: mpsc::Receiver<QueueMessage>,
    llama_client: LlamaClient,
    config: Config,
) {
    info!("Queue worker started");

    // Counter for tracking how many requests we've processed.
    // This is used for the "every Nth request fails" simulation.
    let mut request_count: u32 = 0;

    // Loop over incoming messages.
    // The while let loop is like a "for each" but for async streams.
    // It will keep going until the channel is closed and empty.
    // Note: rx is declared as `mut` so we can call .recv().await on it.
    while let Some(message) = rx.recv().await {
        match message {
            QueueMessage::Query { request, response_sender } => {
                // Increment our request counter.
                // The += 1 operator adds 1 to the current value.
                request_count += 1;

                info!(
                    request_count = request_count,
                    "Processing query request"
                );

                // Simulate queue delay (2 seconds by default).
                // This represents the time a request waits before being processed.
                let queue_delay = Duration::from_millis(config.queue_delay_ms);
                info!(delay_ms = config.queue_delay_ms, "Simulating queue delay");
                sleep(queue_delay).await;

                // Check if this request should fail (every Nth request).
                // The % operator gives us the remainder. If remainder is 0,
                // it means request_count is a multiple of failure_interval.
                if request_count % config.failure_interval == 0 {
                    warn!(
                        request_count = request_count,
                        "Simulating request failure (every {}th request)",
                        config.failure_interval
                    );

                    // Create an error response.
                    let error_response = GameQueryResponse {
                        content: String::new(),
                        success: false,
                        timestamp: chrono::Utc::now(),
                        error: Some(format!(
                            "Simulated failure: request #{}/{} failed",
                            request_count, config.failure_interval
                        )),
                    };

                    // Send the error response back to the caller.
                    // The .ok() converts the Result to Option, silently ignoring errors.
                    // This is fine because if the receiver dropped, there's nothing to do.
                    let _ = response_sender.send(error_response);
                    continue;
                }

                // Simulate request delay (500ms by default).
                // This represents network overhead before the actual API call.
                let request_delay = Duration::from_millis(config.request_delay_ms);
                info!(delay_ms = config.request_delay_ms, "Simulating request delay");
                sleep(request_delay).await;

                // Now make the actual request to llama.cpp.
                let result = llama_client.complete(&request).await;

                // Handle the result (success or failure).
                match result {
                    Ok(content) => {
                        // Success: create a positive response.
                        info!("Request completed successfully");
                        let response = GameQueryResponse {
                            content,
                            success: true,
                            timestamp: chrono::Utc::now(),
                            error: None,
                        };
                        let _ = response_sender.send(response);
                    }
                    Err(e) => {
                        // Failure: create an error response.
                        error!(error = %e, "Request to llama.cpp failed");
                        let error_response = GameQueryResponse {
                            content: String::new(),
                            success: false,
                            timestamp: chrono::Utc::now(),
                            error: Some(format!("llama.cpp error: {}", e)),
                        };
                        let _ = response_sender.send(error_response);
                    }
                }
            }
            QueueMessage::Shutdown => {
                info!("Queue worker shutting down");
                break;
            }
        }
    }

    info!("Queue worker stopped");
}
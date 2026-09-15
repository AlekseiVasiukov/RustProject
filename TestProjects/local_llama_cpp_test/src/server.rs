// ℹ️  This file contains our Axum HTTP server.
//     Axum is a popular web framework for Rust that makes it easy to
//     build REST APIs with async handlers.
//
//     Key concepts:
//     - Routes: URL paths like /health or /game/query
//     - Handlers: Async functions that process HTTP requests
//     - State: Shared data (like our queue sender) passed to handlers
//     - Swagger UI: Auto-generated API documentation

use axum::{
    routing::get,
    routing::post,
    Router,
    Json,
    extract::State,
    http::StatusCode,
};
use tokio::sync::mpsc;
use tracing::info;

// Import utoipa for OpenAPI/Swagger generation
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Import our custom types and modules
use crate::types::{GameQueryRequest, GameQueryResponse, HealthStatus, QueueMessage};

// ============================================================================
// ApiState - Shared state passed to all handlers
// ============================================================================

/// Holds shared state that all request handlers can access.
/// 
/// In Axum, "state" is a way to share resources (like database connections
/// or channel senders) across all handler functions.
/// 
/// Think of it as a backpack that every handler can reach into.
#[derive(Clone)]
pub struct ApiState {
    /// Sender for the queue channel.
    /// 
    /// This is how handlers send work requests to the background queue worker.
    pub queue_sender: mpsc::Sender<QueueMessage>,
}

// ============================================================================
// OpenAPI Schema Definition
// ============================================================================

/// Defines the OpenAPI schema for auto-generated Swagger documentation.
/// 
/// This macro tells utoipa which types to include in the API documentation.
/// When you visit /swagger-ui, you'll see these types documented.
#[derive(OpenApi)]
#[openapi(
    info(title = "Game Engine API",
        description = "API for the Game Engine with llama.cpp integration. This server exposes endpoints for game queries and health monitoring.",
        version = "0.1.0",
        contact(name = "Developer")),
    paths(
        health_check,
        game_query,
    ),
    components(schemas(
        GameQueryRequest,
        GameQueryResponse,
        HealthStatus,
    ))
)]
pub struct ApiDoc;

// ============================================================================
// Handler Functions
// ============================================================================

/// GET /health - Health check endpoint.
/// 
/// This endpoint returns the current server health status.
/// It's commonly used by load balancers and monitoring tools.
/// 
/// The #[utoipa::path] macro auto-generates OpenAPI documentation.
/// Without it, Swagger wouldn't know this endpoint exists.
/// 
/// # Returns
/// 
/// - 200 OK with HealthStatus JSON when server is healthy
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Server is healthy", body = HealthStatus),
    )
)]
async fn health_check() -> (StatusCode, Json<HealthStatus>) {
    // Create our health status response
    let status = HealthStatus {
        status: "ok".to_string(),
        message: "Game Engine server is running".to_string(),
        timestamp: chrono::Utc::now(),
    };

    // Return 200 OK status and the JSON response
    (StatusCode::OK, Json(status))
}

/// POST /game/query - Main game query endpoint.
/// 
/// Receives a prompt and forwards it to the llama.cpp server through the queue.
/// 
/// The handler:
/// 1. Receives the JSON request body
/// 2. Creates a oneshot channel for getting the response back
/// 3. Sends the query to the queue worker
/// 4. Awaits the response and returns it to the client
/// 
/// # Arguments
/// 
/// * `state` - The shared ApiState (injected by Axum automatically)
/// * `request` - The JSON body containing the query request
/// 
/// # Returns
/// 
/// - 200 OK with GameQueryResponse on success
/// - 503 Service Unavailable if queue is full or worker is down
/// 
#[utoipa::path(
    post,
    path = "/game/query",
    request_body = GameQueryRequest,
    tag = "Game",
    responses(
        (status = 200, description = "Query processed successfully", body = GameQueryResponse),
        (status = 503, description = "Queue worker unavailable", body = GameQueryResponse),
    )
)]
async fn game_query(
    // State extractor gives us access to our shared state
    State(state): State<ApiState>,
    // Json extractor automatically deserializes the request body
    Json(request): Json<GameQueryRequest>,
) -> (StatusCode, Json<GameQueryResponse>) {
    // Create a oneshot channel for this specific request.
    // A oneshot channel can only send ONE message, then it's done.
    // This is perfect for request-response patterns.
    let (response_sender, response_receiver) = tokio::sync::oneshot::channel();

    // Build our queue message with the request and response channel
    let message = QueueMessage::Query {
        request,
        response_sender,
    };

    // Try to send the message to the queue worker
    let send_result = state.queue_sender.send(message).await;

    match send_result {
        Ok(()) => {
            // Message sent successfully, now wait for the response
            info!("Query sent to queue, awaiting response");
            
            // Receive the response from the worker
            match response_receiver.await {
                Ok(response) => {
                    // Worker sent back a response
                    if response.success {
                        (StatusCode::OK, Json(response))
                    } else {
                        // Request failed in the worker (simulated failure)
                        (StatusCode::OK, Json(response))
                    }
                }
                Err(_) => {
                    // The oneshot channel was dropped without sending a response
                    // This shouldn't happen, but we handle it gracefully
                    error_response("Queue worker disappeared without sending response")
                }
            }
        }
        Err(_) => {
            // The channel is full or the worker has stopped
            error_response("Queue worker is unavailable")
        }
    }
}

/// Helper function to create an error response.
/// 
/// This avoids code duplication when returning errors.
/// 
/// # Arguments
/// 
/// * `error_message` - The error message to include in the response
/// 
/// # Returns
/// 
/// A tuple of (StatusCode, Json<GameQueryResponse>)
fn error_response(error_message: &str) -> (StatusCode, Json<GameQueryResponse>) {
    let response = GameQueryResponse {
        content: String::new(),
        success: false,
        timestamp: chrono::Utc::now(),
        error: Some(error_message.to_string()),
    };
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

// ============================================================================
// Server Setup
// ============================================================================

/// Creates and configures the Axum router with all routes.
/// 
/// This function sets up:
/// 1. All HTTP routes (GET /health, POST /game/query)
/// 2. Swagger UI at /swagger-ui
/// 3. Shared state for all handlers
/// 
/// # Arguments
/// 
/// * `queue_sender` - The sender for the queue channel
/// 
/// # Returns
/// 
/// A configured Router ready to serve requests.
pub fn create_router(queue_sender: mpsc::Sender<QueueMessage>) -> Router {
    // Create our shared state
    let api_state = ApiState { queue_sender };

    // Build the router with route definitions
    Router::new()
        // Register our API routes
        .route("/health", get(health_check))
        .route("/game/query", post(game_query))
        
        // Add Swagger UI for interactive API documentation
        // The ApiDoc struct we defined earlier provides the OpenAPI spec
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        // Add CORS middleware to allow browser-based clients
        .layer(tower_http::cors::CorsLayer::permissive())
        
        // Add tracing middleware for logging all requests
        .layer(tower_http::trace::TraceLayer::new_for_http())
        
        // Add shared state layer - this makes api_state available to all handlers
        .with_state(api_state)
}
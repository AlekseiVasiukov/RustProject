// ℹ️  This file contains our HTTP client for talking to the llama.cpp server.
//     It uses the `reqwest` crate to send HTTP requests and receive JSON responses.
//
//     Think of this as a "translator" between our app and the llama.cpp server.
//     We give it a prompt, and it returns the LLM's response.

use anyhow::Result;
use reqwest::Client;
use tracing::{info, warn, error};

// Import our custom types from the types module
use crate::types::GameQueryRequest;

// ============================================================================
// LlamaClient - Our HTTP client wrapper for llama.cpp
// ============================================================================

/// A client for communicating with the llama.cpp server.
/// 
/// This struct wraps the `reqwest::Client` and the server URL.
/// Reusing the same client connection is more efficient than creating
/// a new one for every request.
#[derive(Clone)]
pub struct LlamaClient {
    /// The underlying reqwest HTTP client.
    /// 
    /// This handles connection pooling and keeps TCP connections alive.
    client: Client,
    
    /// The base URL of the llama.cpp server.
    /// 
    /// Example: "http://localhost:8080"
    base_url: String,
}

impl LlamaClient {
    /// Creates a new LlamaClient instance.
    /// 
    /// # Arguments
    /// 
    /// * `base_url` - The URL of the llama.cpp server (e.g., "http://localhost:8080")
    /// 
    /// # Returns
    /// 
    /// A new `LlamaClient` ready to make requests.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let client = LlamaClient::new("http://localhost:8080".to_string());
    /// ```
    pub fn new(base_url: String) -> Self {
        LlamaClient {
            // reqwest::Client::new() creates a client with sensible defaults.
            // This client can be cloned and shared across async tasks.
            client: Client::new(),
            base_url,
        }
    }

    /// Sends a completion request to the llama.cpp server.
    /// 
    /// This method:
    /// 1. Builds the JSON request body from our GameQueryRequest
    /// 2. Sends it as a POST request to the /completion endpoint
    /// 3. Parses the response and extracts the LLM's generated text
    /// 
    /// # Arguments
    /// 
    /// * `request` - The GameQueryRequest containing the prompt
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either the response string or an error.
    /// 
    /// # llama.cpp API Format
    /// 
    /// The llama.cpp server expects a JSON body like:
    /// ```json
    /// {
    ///   "prompt": "Your prompt here",
    ///   "n_predict": 256,
    ///   "temperature": 0.8
    /// }
    /// ```
    pub async fn complete(&self, request: &GameQueryRequest) -> Result<String> {
        // Build the full URL for the completion endpoint.
        // llama.cpp uses /completion for generating text.
        let url = format!("{}/completion", self.base_url);
        
        // Log what we're doing for debugging purposes.
        // tracing::info!() is like println!() but with log levels.
        info!(url = %url, prompt = %request.prompt, "Sending completion request");

        // Prepare the request body as a Rust struct.
        // llama.cpp expects specific JSON fields.
        // The &request.prompt borrows the prompt string without taking ownership.
        let body = serde_json::json!({
            "prompt": &request.prompt,
            // n_predict limits how many tokens the model generates.
            // 256 is a reasonable default for short responses.
            "n_predict": 256,
            // temperature controls randomness: 0.0 = deterministic, 1.0 = very random
            // 0.8 gives a good balance of creativity and coherence
            "temperature": 0.8,
        });

        // Send the POST request.
        // The .await keyword pauses this function until the HTTP request completes.
        // This does NOT block the entire program - other tasks can run while waiting.
        let response = self.client
            .post(&url)
            .json(&body)  // Serialize our body struct to JSON automatically
            .send()      // Actually send the HTTP request
            .await?;     // The ? operator propagates errors. If this fails,
                         // the function returns Early with the error.

        // Check if the HTTP response was successful (status code 2xx).
        // if !response.status().is_success() would work too, but this is shorter.
        if !response.status().is_success() {
            // Read the error text from the response body
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            
            // Log the error for debugging
            error!(status = %status, error = %error_text, "llama.cpp request failed");
            
            // Return an error with a descriptive message
            return Err(anyhow::anyhow!(
                "llama.cpp server returned error {}: {}",
                status,
                error_text
            ));
        }

        // Parse the JSON response from llama.cpp.
        // .json::<Value>() parses the response body into a flexible JSON value.
        // This is like a generic "bag of JSON data" that we can explore.
        let json: serde_json::Value = response.json().await?;

        // Extract the generated text from the response.
        // llama.cpp returns the text in either "content" or "result" field.
        // We use .get() to safely access nested JSON fields.
        let content = json.get("content")
            .and_then(|v| v.as_str())
            .or_else(|| json.get("result").and_then(|v| v.as_str()))
            .unwrap_or("");

        // If the content is empty, warn the user.
        // This helps with debugging when the model returns nothing.
        if content.is_empty() {
            warn!("llama.cpp returned empty response");
            Ok("No response from model.".to_string())
        } else {
            // Log success and return the response text.
            info!(length = content.len(), "Received completion response");
            Ok(content.to_string())
        }
    }

    /// Tests if the llama.cpp server is reachable.
    /// 
    /// Sends a GET request to the server root and checks if it responds.
    /// 
    /// # Returns
    /// 
    /// `true` if the server responds with any status code, `false` if unreachable.
    pub async fn is_reachable(&self) -> bool {
        // Try to connect to the server root URL.
        // We use a short timeout so this doesn't hang forever.
        let result = self.client
            .get(&self.base_url)
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await;

        // If the request completes (success or failure), the server is reachable.
        // Only a true connection failure means it's unreachable.
        match result {
            Ok(_response) => {
                info!(url = %self.base_url, "llama.cpp server is reachable");
                true
            }
            Err(e) => {
                warn!(url = %self.base_url, error = %e, "llama.cpp server is not reachable");
                false
            }
        }
    }
}
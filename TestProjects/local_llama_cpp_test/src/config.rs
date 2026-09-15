// ℹ️  This file contains our application configuration.
//     Instead of "hardcoding" values (writing them directly in code),
//     we put them in one place so they're easy to change.
//
//     For example, if you want to change the llama.cpp server URL,
//     you only change it here instead of searching through all files.

// Import our shared types from the types module
use anyhow::Result;

// ============================================================================
// Config - Main configuration struct
// ============================================================================

/// Holds all configuration values for the application.
/// 
/// Each field stores a specific setting. The `#[derive(Clone, Debug)]` line
/// gives us automatic implementations for:
/// - Clone: Copy this struct to another variable
/// - Debug: Print it with println!("{:?}", config) for logging
#[derive(Debug, Clone)]
pub struct Config {
    /// The URL of the llama.cpp server.
    /// 
    /// Default: "http://localhost:8080"
    /// This is where our client will send HTTP requests.
    pub llama_server_url: String,
    
    /// The address our Axum server binds to.
    /// 
    /// Default: "0.0.0.0:3000"
    /// 0.0.0.0 means "listen on all network interfaces"
    pub axum_bind_address: String,
    
    /// Simulated queue delay in milliseconds.
    /// 
    /// Default: 2000 (2 seconds)
    /// This simulates the time a request waits in a queue.
    pub queue_delay_ms: u64,
    
    /// Simulated request delay in milliseconds.
    /// 
    /// Default: 500 (half a second)
    /// This simulates network/processing overhead.
    pub request_delay_ms: u64,
    
    /// Every Nth request will fail.
    /// 
    /// Default: 3
    /// So requests 3, 6, 9, 12, ... will return errors.
    pub failure_interval: u32,
}

impl Config {
    /// Creates a new Config with default values.
    /// 
    /// This is a "constructor" method. The `-> Self` means it returns
    /// a Config struct (Self is just a shorthand for the current type).
    /// 
    /// # Returns
    /// 
    /// A `Config` instance with sensible defaults for local development.
    pub fn new() -> Self {
        Config {
            // The llama.cpp server runs on port 8080 by default
            llama_server_url: "http://localhost:8080".to_string(),
            
            // Our game engine server runs on port 3000
            axum_bind_address: "0.0.0.0:3000".to_string(),
            
            // 2 seconds = 2000 milliseconds for queue delay
            queue_delay_ms: 2000,
            
            // 500 milliseconds for request delay
            request_delay_ms: 500,
            
            // Every 3rd request fails in simulation
            failure_interval: 3,
        }
    }

    /// Creates a Config from environment variables if set, otherwise uses defaults.
    /// 
    /// This checks for environment variables like LLAMA_SERVER_URL.
    /// If they exist and are not empty, it uses those values.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either the Config or an error.
    /// In this case, errors are unlikely since we have fallback defaults.
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            // std::env::var() gets an environment variable.
            // It returns Err if the variable is not set.
            // We use `.ok()` to convert that to None, then `.unwrap_or()` for a fallback.
            llama_server_url: std::env::var("LLAMA_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            
            axum_bind_address: std::env::var("AXUM_BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            
            queue_delay_ms: std::env::var("QUEUE_DELAY_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2000),
            
            request_delay_ms: std::env::var("REQUEST_DELAY_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(500),
            
            failure_interval: std::env::var("FAILURE_INTERVAL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
        })
    }
}
mod routes;
mod types;

use anyhow::Result;

// use crate::types;
#[tokio::main]
pub async fn main() -> Result<()> {
    //build command for linux
    //cargo build --release --target x86_64-unknown-linux-gnu

    //Example of using debug/release build commands
    // if cfg!(debug_assertions) {
    //     println!("Debugging enabled");
    // } else {
    //     println!("Debugging disabled");
    // }

    // #[cfg(debug_assertions)]
    // println!("Debugging enabled");

    // #[cfg(not(debug_assertions))]
    // println!("Debugging disabled");

    // example();

    //Logging initialization
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let routes = routes::create_routes()?;

    if cfg!(debug_assertions) {
        tracing::info!("Starting debug server on 0.0.0.0:3000");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        axum::serve(listener, routes).await?;
    } else {
        tracing::info!("Starting release server on 127.0.0.1:3000");
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        axum::serve(listener, routes).await?;
    }

    Ok(())
}

//Example of using debug/release build commands

//   #[cfg(not(debug_assertions))]
//     fn example() {
//         println!("Debugging disabled");
//     }

//         #[cfg(debug_assertions)]
//     fn example() {
//         println!("Debugging enabled");
//     }

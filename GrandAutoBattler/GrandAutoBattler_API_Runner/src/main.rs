mod routes;
mod types;

// use crate::types;
#[tokio::main]
pub async fn main() {
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

    let routes = routes::create_routes();

    if cfg!(debug_assertions) {
        println!("Endpoint for Debug 0.0.0.0:3000 ");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, routes).await.unwrap();
    } else {
        println!("Endpoint for Release 127.0.0.1:3000 ");
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        axum::serve(listener, routes).await.unwrap();
    }
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

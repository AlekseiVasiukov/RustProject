mod routes;
mod types;

// use crate::types;
#[tokio::main]
pub async fn main() {
    let routes = routes::create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

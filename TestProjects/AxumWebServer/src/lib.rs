
use axum::{Router, routing::get};

pub(crate) async fn run() {
    let app = Router::new().route("/", get(hello_world()));
    //  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn hello_world() -> String {
    "Hello World".to_owned()
}

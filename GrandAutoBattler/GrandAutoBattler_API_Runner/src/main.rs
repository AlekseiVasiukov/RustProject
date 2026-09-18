mod types;
mod routes;


use GrandAutoBattler_GameEngine::game_instance_manager;

use routes::create_routes;
// use crate::types;
#[tokio::main]
pub async fn main() {
    let mut game_manger = game_instance_manager::GameInstancesManager::new();
    let game = game_manger.GetNewGameInstanceId();
    println!("{:?}", game);
    println!("YO");

    // let routes = create_routes();
    let routes = routes::create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

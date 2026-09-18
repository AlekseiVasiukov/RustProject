use GrandAutoBattler_GameEngine::{game_instance_manager};
// #[tokio::main]
pub  fn main() {
    let mut game_manger = game_instance_manager::GameInstancesManager::new();
    let game = game_manger.GetNewGameInstanceId();
    println!("{:?}", game);
    println!("YO");
    loop{
        
    }
}

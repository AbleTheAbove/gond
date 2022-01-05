mod components;
mod entities;

use components::*;
use entities::*;

fn main() {
    let mut game_state: GameState = GameState::new();
    let data = serde_json::to_string(&game_state).unwrap();
    println!("{}", data);
}

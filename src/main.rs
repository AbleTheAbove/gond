mod components;
mod entities;

use components::*;
use entities::*;

fn main() {
    let mut game_state: GameState = GameState::new();

    game_state.add_world(Some(Name("Mercury")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Venus")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Earth")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Mars")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Jupiter")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Saturn")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Uranus")), Some(Name("USSR")), vec![0, 3, 5]);
    game_state.add_world(Some(Name("Neptune")), Some(Name("USSR")), vec![0, 3, 5]);

    let data = serde_json::to_string(&game_state).unwrap();
    println!("{}", data);
}

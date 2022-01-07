mod components;
mod entities;

use components::*;
use entities::*;

fn main() {
    let mut game_state: GameState = GameState::new();

    game_state.add_world(Some(Name("Earth")), None, Vec::new());

    let data = serde_json::to_string(&game_state).unwrap();
    println!("{:#?}", game_state);
}

//by what metric are you rating trades and how to weight them
// data strucutes, every nation and how they connect, distance/weight/value between them
// make nations produce then consume
// fill a map with planets

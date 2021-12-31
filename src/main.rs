use serde::Serialize;
use std::collections::HashMap;

fn main() {
    let mut game_state: GameState = GameState::new();
    game_state.new_entity(Some(Name("Seattle")), Some(Money(0)), None);
    game_state.new_entity(None, Some(Money(0)), Some(Quantity(1)));
    game_state.new_entity(None, Some(Money(0)), Some(Quantity(1)));
    game_state.new_entity(None, Some(Money(0)), Some(Quantity(1)));
    game_state.new_entity(None, Some(Money(0)), Some(Quantity(1)));
    game_state.new_entity(None, Some(Money(0)), Some(Quantity(1)));
    game_state.new_entity(None, Some(Money(0)), Some(Quantity(1)));
    let data = serde_json::to_string(&game_state).unwrap();
    println!("{}", data);
}

type Id = u16;

#[derive(Debug, Serialize)]
struct Name(&'static str);

#[derive(Debug, Serialize)]
struct Money(u32);

#[derive(Debug, Serialize)]
struct Quantity(u16);

#[derive(Debug, Serialize)]
struct GameState {
    entity_count: Id,
    names: HashMap<Id, Option<Name>>,
    money: HashMap<Id, Option<Money>>,
    quantity: HashMap<Id, Option<Quantity>>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            entity_count: 0,
            names: HashMap::new(),
            money: HashMap::new(),
            quantity: HashMap::new(),
        }
    }

    fn new_entity(
        &mut self,
        name: Option<Name>,
        currency: Option<Money>,
        quantity: Option<Quantity>,
    ) {
        let id = self.entity_count;
        self.names.insert(id, name);
        self.money.insert(id, currency);
        self.quantity.insert(id, quantity);
        self.entity_count += 1;
    }
}

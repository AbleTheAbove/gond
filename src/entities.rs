use crate::components::*;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct GameState {
    pub entity_count: Id,
    pub names: HashMap<Id, Option<Name>>,
    pub money: HashMap<Id, Option<Money>>,
    pub quantity: HashMap<Id, Option<Quantity>>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entity_count: 0,
            names: HashMap::new(),
            money: HashMap::new(),
            quantity: HashMap::new(),
        }
    }

    pub fn new_entity(
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

#[derive(Debug, Serialize)]
struct Worlds {
    name: String,
    // has regions, nations, name, owner
    // can change names
}

impl Worlds {}

#[derive(Debug, Serialize)]
struct Nations {
    name: String,
    regions: Vec<Regions>,
    provinces: Vec<Provinces>,
    // has name, worlds, regions, provinces, money, popgroups, needs/wants, government type, religion, resources, vassals, overlords
    // can aquire new worlds, regions, provinces, can trade, can create or destroy popgroups, can change government, can change religion, can aquire or lose resources, can change names
}

impl Nations {}

#[derive(Debug, Serialize)]
struct Regions {
    name: String,
    // has provinces, name, an owner
    // can change names, can change what provinces are in it, can summarize provinces
}

impl Regions {}

#[derive(Debug, Serialize)]
struct Provinces {
    name: String,
    // has popgroups, resources, name, an owner
    // can change names, can change resources, can create and remove popgroups, can change owners
}

impl Provinces {}

#[derive(Debug, Serialize)]
struct PopGroups {
    name: String,
    population_count: u64,
    // has number of people, has money, has needs/wants, has growth, has worldviews, job type, name, religion, politics
    // can change name, can gain or lose money, can attempt to meet needs/wants, can trade, can change jobs, can change religion, can change politics
}

impl PopGroups {}

#[derive(Debug, Serialize)]
struct Resources {
    name: String,
    value: u64,
    quantity: u64,
    // has name, value, quantity, extraction rate
    // can run out, can be used, can be created
}

impl Resources {}

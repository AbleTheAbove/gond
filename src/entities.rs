use crate::components::*;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct GameState {
    // has worlds, nations, regions, provinces, popgroups, resources
    pub entity_count: Id,
    pub worlds: Worlds,
    //pub nations: Nations,
    //pub regions: Regions,
    //pub provinces: Provinces,
    //pub pop_groups: PopGroups,
    //pub resources: Resources,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entity_count: 0,
            worlds: Worlds::new(),
            //nations: Nations::new(),
            //regions: Regions::new(),
            //provinces: Provinces::new(),
            //pop_groups: PopGroups::new(),
            //resources: Resources::new(),
        }
    }

    pub fn add_world(&mut self, name: Option<Name>, owner: Option<Name>, regions: Vec<RegionId>) {
        self.worlds.names.insert(self.entity_count, name);
        self.worlds.owners.insert(self.entity_count, owner);
        self.worlds.regions.insert(self.entity_count, regions);
        self.entity_count += 1;
    }
}

#[derive(Debug, Serialize)]
pub struct Worlds {
    names: HashMap<Id, Option<Name>>,
    owners: HashMap<Id, Option<Name>>,
    regions: HashMap<Id, Vec<RegionId>>,
    // can change names
}

impl Worlds {
    pub fn new() -> Worlds {
        Worlds {
            names: HashMap::new(),
            owners: HashMap::new(),
            regions: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Nations {
    // has name, worlds, regions, provinces, money, popgroups, needs/wants, government type, religion, resources, vassals, overlords
// can aquire new worlds, regions, provinces, can trade, can create or destroy popgroups, can change government, can change religion, can aquire or lose resources, can change names
}

impl Nations {}

#[derive(Debug, Serialize)]
struct Regions {
    // has provinces, name, an owner
// can change names, can change what provinces are in it, can summarize provinces
}

impl Regions {}

#[derive(Debug, Serialize)]
struct Provinces {
    // has popgroups, resources, name, an owner
// can change names, can change resources, can create and remove popgroups, can change owners
}

impl Provinces {}

#[derive(Debug, Serialize)]
struct PopGroups {
    // has number of people, has money, has needs/wants, has growth, has worldviews, job type, name, religion, politics
// can change name, can gain or lose money, can attempt to meet needs/wants, can trade, can change jobs, can change religion, can change politics
}

impl PopGroups {}

#[derive(Debug, Serialize)]
struct Resources {
    // has name, value, quantity, extraction rate
// can run out, can be used, can be created
}

impl Resources {}

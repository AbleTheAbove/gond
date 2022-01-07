use crate::components::*;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct GameState {
    // has worlds, nations, regions, provinces, popgroups, resources
    pub entity_count: Id,
    pub worlds: Worlds,
    pub nations: Nations,
    pub regions: Regions,
    pub provinces: Provinces,
    pub pop_groups: PopGroups,
    pub resources: Resources,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entity_count: 0,
            worlds: Worlds::new(),
            nations: Nations::new(),
            regions: Regions::new(),
            provinces: Provinces::new(),
            pop_groups: PopGroups::new(),
            resources: Resources::new(),
        }
    }

    pub fn add_world(&mut self, name: Option<Name>, owner: Option<Name>, regions: Vec<RegionId>) {
        self.worlds.name.insert(self.entity_count, name);
        self.worlds.owner.insert(self.entity_count, owner);
        self.worlds.regions.insert(self.entity_count, regions);
        self.entity_count += 1;
    }

    pub fn add_nation(
        &mut self,
        name: Option<Name>,
        worlds: Vec<Option<WorldId>>,
        regions: Vec<Option<RegionId>>,
        provinces: Vec<Option<ProvinceId>>,
        pop_groups: Vec<Option<PopGroupId>>,
        resources: Vec<Option<ResourceId>>,
        treasury: u64,
    ) {
        self.nations.name.insert(self.entity_count, name);
        self.nations.worlds.insert(self.entity_count, worlds);
        self.nations.regions.insert(self.entity_count, regions);
        self.nations.provinces.insert(self.entity_count, provinces);
        self.nations
            .pop_groups
            .insert(self.entity_count, pop_groups);
        self.nations.resources.insert(self.entity_count, resources);
        self.nations.treasury = treasury;
    }
}

#[derive(Debug, Serialize)]
pub struct Worlds {
    name: HashMap<Id, Option<Name>>,
    owner: HashMap<Id, Option<Name>>,
    regions: HashMap<Id, Vec<RegionId>>,
    // can change names
}

impl Worlds {
    pub fn new() -> Worlds {
        Worlds {
            name: HashMap::new(),
            owner: HashMap::new(),
            regions: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Nations {
    name: HashMap<Id, Option<Name>>,
    worlds: HashMap<Id, Vec<Option<WorldId>>>,
    regions: HashMap<Id, Vec<Option<RegionId>>>,
    provinces: HashMap<Id, Vec<Option<ProvinceId>>>,
    // perhaps currency should be its own struct/type for different currency and exchange rates
    pop_groups: HashMap<Id, Vec<Option<PopGroupId>>>,
    resources: HashMap<Id, Vec<Option<ResourceId>>>,
    treasury: u64,
    // needs/wants, government type, religion, vassals, overlords
    // can aquire new worlds, regions, provinces, can trade, can create or destroy popgroups, can change government, can change religion, can aquire or lose resources, can change names
}

impl Nations {
    pub fn new() -> Nations {
        Nations {
            name: HashMap::new(),
            worlds: HashMap::new(),
            regions: HashMap::new(),
            provinces: HashMap::new(),
            treasury: 0,
            pop_groups: HashMap::new(),
            resources: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Regions {
    // has provinces, name, an owner
// can change names, can change what provinces are in it, can summarize provinces
}

impl Regions {
    pub fn new() -> Regions {
        Regions {}
    }
}

#[derive(Debug, Serialize)]
pub struct Provinces {
    // has popgroups, resources, name, an owner
// can change names, can change resources, can create and remove popgroups, can change owners
}

impl Provinces {
    pub fn new() -> Provinces {
        Provinces {}
    }
}

#[derive(Debug, Serialize)]
pub struct PopGroups {
    // has number of people, has money, has needs/wants, has growth, has worldviews, job type, name, religion, politics
// can change name, can gain or lose money, can attempt to meet needs/wants, can trade, can change jobs, can change religion, can change politics
}

impl PopGroups {
    pub fn new() -> PopGroups {
        PopGroups {}
    }
}

#[derive(Debug, Serialize)]
pub struct Resources {
    // has name, value, quantity, extraction rate
// can run out, can be used, can be created
}

impl Resources {
    pub fn new() -> Resources {
        Resources {}
    }
}

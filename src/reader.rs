use time::Date;
use std::collections::VecDeque;
use crate::{
    galactic_object::GalacticObject,
    country::Country,
    federation::Federation,
    fleet::Fleet,
    shared::{Coordinate,Flag,Ethos}
};

// Using `VecDeqeque` for sequentially numbered items (0..n)

struct Game {
    version: String,
    version_control_revision: u64,
    name: String,
    date: Date,
    required_dlcs: Vec<String>,
    player: Vec<Player>,
    tick: u64,
    random_log_day: u64,
    species: Vec<Species>,
    last_created_species:u64,
    nebula: Vec<Nebula>,
    construction: VecDeque<Construction>, // This is a super weird one, so I'm guessing here,
    pop: VecDeque<Pop>,
    last_created_pop: u64,
    galactic_objects: VecDeque<GalacticObject>,
    starbases: VecDeque<Starbase>,
    planets: VecDeque<Planet>,
    countries: VecDeque<Country>,
    federations: VecDeque<Federation>,
    truces: Vec<u64>, // `Vec<u64>` is placeholder, don't know what this is
    trade_deals:  Vec<u64>, // `Vec<u64>` is placeholder, don't know what this is
    last_created_country: u64,
    last_refugee_country: u64,
    last_created_system: u64,
    leaders: VecDeque<Leader>,
    ships: VecDeque<Ship>,
    fleets: VecDeque<Fleet>,
    ...
}

// -------------------------------------------------

struct Player {
    name: String,
    country: u32
}

// -------------------------------------------------

struct Species {
    name_list: Vec<String>,
    name: String,
    plural: String,
    adjective: Option<String>,
    class: String,
    portrait: String,
    traits: Vec<String>,
    home_planet: u64,
    name_data: Option<String>
}

// -------------------------------------------------

struct Nebula {
    coordinate: Coordinate,
    name: String,
    radius: u64,
    galactic_objects: Vec<u64>
}
// -------------------------------------------------

struct Construction {
    owner: u32,
    location: ConstructionLocation,
    simultaneous: bool,
    r#type: ConsturctionType,
    disabled: bool
}
struct ConstructionLocation {
    r#type: u64,
    id :u64
}
enum ConsturctionType { Planet, Army }

// -------------------------------------------------

struct Pop {
    species_index:u64,
    ethos: Ethos,
    job: String,
    catagory: String,
    planet: u64,
    crime: f32,
    power: u64,
    diplomatic_weight: f32,
    happiness: f32,
    amenities_usage: u32,
    housing_usage: u32
}

// -------------------------------------------------

struct Starbase {
    level: String,
    modules: VecDeque<String>,
    buildings: VecDeque<String>,
    build_queue: u64,
    shipyard_build_queue: u64,
    ship_design: u64,
    station: u64,
    owner: u64,
    orbitals: VecDeque<u64>
}

// -------------------------------------------------

struct Planet {
    name: String,
    custom_name: Option<bool>,
    planet_name: String,
    coordinate: Coordinate,
    orbit: u64,
    planet_size: u64,
    bombardment_damage: f32,
    last_bombardment: Date,
    automated_development: bool,
    controller: u64,
    kill_pop: Date,
    build_queue: u64,
    army_build_queue: u64,
    branch_office_build_queue: u64,
    planet_orbitals: Vec<u64>, // u64 is placeholder, I have no idea what type is here
    shipclass_orbital_station: u64,
    flags: Vec<Flag>,
    entity: u64,
    surveyed_by: Option<u64>,
    prevent_anomaly: bool,
    deposits: Vec<u64>,
    favourite_jobs: Vec<u64>, // u64 is placeholder, I have no idea what type is here
    stability: f32,
    migration: f32,
    crime: f32,
    amenitiesL: f32,
    amenities_usage: f32,
    free_amenities: f32,
    free_housing: f32,
    total_housing: f32,
    housing_usage: f32,
    employable_pops: u64,
    num_sapient_pops: u64,
    auto_slots_taken: Vec<bool>
}

// -------------------------------------------------

struct Leader{
    name: LeaderName,
    species_index: u64,
    portrait: String,
    gender: Gender,
    country: u64,
    creator: u64,
    class: String,
    experience: Option<f32>,
    level: u64,
    location: Location,
    pre_ruler_location: Option<Location>,
    date: Date,
    age: u64,
    agenda: String,
    design: Option<LeaderDesign>,
    mandate: Option<String>,
    roles: LeaderRoles,
}

struct LeaderName {
    first_name: String,
    second_name: Option<String>
}

enum Gender { Male, Female }

struct Location {
    r#type: LocationType,
    area: Option<LocationArea>,
    assignment: Option<u64>, // u64 is placeholder
    id: u64
}
enum LocationType { Sector, Planet, Tech, Ship }
enum LocationArea { Physics, Engineering, Society }

struct LeaderDesign {
    gender: Gender,
    name: String,
    protrait: String,
    texture: u64,
    hair: u64,
    clothes: u64,
    leader_class: String
}

struct LeaderRoles {
    admiral: Vec<String>,
    general: Vec<String>,
    scientist: Vec<String>,
    governor: Vec<String>,
    ruler: Vec<String>
}

// -------------------------------------------------

struct Ship {
    fleet: u64,
    name: String,
    key: Option<String>,
    reserve: u64,
    ship_design: u64,
    design_upgrade: Option<u64>,
    graphical_culture: String,
    sections: Vec<ShipSection>,
    speed: Option<f32>,
    coordinate: Coordinate,
    target_coordinate: Coordinate,
    post_move_angle: f32,
    hitpoints: f32,
    shield_hitpoints: f32,
    armor_hitpoints: f32,
    max_hitpoints: f32,
    max_shield_hitpoints: f32,
    max_armour_hitpoints: f32,
    rotation: f32,
    forward_x: f32,
    forward_y: f32,
    upgrade_progress: f32,
    next_weapon_index: Option<u64>,
    created_this_update: bool,
    disable_at_health: Option<f32>,
    enable_at_health: Option<f32>,
    leader: Option<u64>
}

struct ShipSection {
    design: String,
    slot: String,
    weapons: Option<Vec<ShipSectionWeapon>>
}
struct ShipSectionWeapon {
    index: u64,
    template: String,
    component_slot: String
}

// -------------------------------------------------
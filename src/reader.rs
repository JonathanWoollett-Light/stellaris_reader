use serde::{Deserialize, Serialize};
use time::Date;

use std::collections::{VecDeque,HashSet};

use crate::{
    galactic_object::GalacticObject,
    country::Country,
    federation::Federation,
    fleet::Fleet,
    shared::{Coordinate,Flag,Ethos,Orbital,TypeAndIDDescriptor},
    galaxy::Galaxy,
    leader::Leader
};

// Using `VecDeqeque` for sequentially numbered items (0..n)

// The fact like some of this shit is plural and some aint is real dumb
#[derive(Serialize, Deserialize)]
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
    construction: VecDeque<Construction>,
    pop: VecDeque<Pop>,
    last_created_pop: u64,
    galactic_object: VecDeque<GalacticObject>,
    starbase_mgr: VecDeque<Starbase>,
    planets: VecDeque<Planet>,
    country: VecDeque<Country>,
    federation: VecDeque<Federation>,
    truce: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    trade_deal:  Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    last_created_country: u64,
    last_refugee_country: u64,
    last_created_system: u64,
    leaders: VecDeque<Leader>,
    ships: VecDeque<Ship>,
    fleet: VecDeque<Fleet>,
    fleet_template: VecDeque<FleetTemplate>,
    last_created_fleet: u64,
    last_created_ship: u64,
    last_created_leader: u64,
    last_created_army: u64,
    last_created_design: u64,
    army: VecDeque<Army>,
    deposit: VecDeque<Option<Deposit>>,
    ground_combat: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    war: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    debris: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    missile: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    strike_craft: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    ambient_object: VecDeque<AmbientObject>,
    last_created_ambient_object: u64,
    last_notification_id: u64,
    camera_focus: u64,
    random_name_database: RandomNameDatabase,
    name_list: Vec<u64>, // TODO `Vec<u64>` is placeholder, don't know what this is
    galaxy: Galaxy,
    galaxy_radius: f32,
    flags: Vec<Flag>,
    saved_event_target: Vec<SavedEventTarget>,
    ship_design: VecDeque<ShipDesign>,
    megastructures: VecDeque<Megastructure>,
    bypasses: VecDeque<Bypass>, // I have to write a fucking seperate rule bc some idiot decided putting some fucking random integer and null value in the middle of this list was reasonable
    natural_wormholes: VecDeque<Wormhole>,
    sectors: VecDeque<Sector>,
    buildings: VecDeque<Building>,
    archaeological_sites: ArchaeologicalSites,
    global_ship_designs: Vec<GlobalShipDesign>,
    clusters: Vec<Cluster>,
    rim_galactic_objects: Vec<u64>,
    used_color: HashSet<String>, // HashSet is used for when there are multiple fields of the same thing declared, it should be a list, but it isn't
    used_symbols: Vec<u64>,
    used_species_names: HashSet<ClassWithOptionalValues>,
    used_species_portraits: HashSet<ClassWithOptionalValues>,
    random_seed: i64,
    random_count: u64,
    market: Market,
    trade_routes_manager: Vec<u64>, // placeholder
    slave_market_manager: Vec<u64>, // placeholder
    galactic_community: GalacticCommunity
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Player {
    name: String,
    country: u32
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
struct Nebula {
    coordinate: Coordinate,
    name: String,
    radius: u64,
    galactic_objects: Vec<u64>
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Construction {
    owner: u32,
    location: ConstructionLocation,
    simultaneous: bool,
    r#type: ConsturctionType,
    disabled: bool
}
#[derive(Serialize, Deserialize)]
struct ConstructionLocation {
    r#type: u64,
    id :u64
}
#[derive(Serialize, Deserialize)]
enum ConsturctionType { Planet, Army }

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
    amenities: f32,
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
struct ShipSection {
    design: String,
    slot: String,
    weapons: Option<Vec<ShipSectionWeapon>>
}
#[derive(Serialize, Deserialize)]
struct ShipSectionWeapon {
    index: u64,
    template: String,
    component_slot: String
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct FleetTemplate {
    fleet: u64,
    home_base: Orbital,
    fleet_template_design: Vec<FleetTemplateDesign>,
    all_queued: Vec<u64>, // TODO This is a placeholder value, find actual value
    count: u64,
    fleet_size: u64
}
#[derive(Serialize, Deserialize)]
struct FleetTemplateDesign {
    design: u64,
    count: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Army {
    name: String,
    r#type: String,
    health: f32,
    max_health: f32,
    home_planet: u64,
    owner: u64,
    species_index: u64,
    planet: u64,
    moral: f32,
    pop: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Deposit {
    r#type: String,
    swap_type: Option<String>,
    planet: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct AmbientObject {
    coordinate: Coordinate,
    data: String,
    properties: AmbientObjectProperties
}
#[derive(Serialize, Deserialize)]
struct AmbientObjectProperties {
    coordinate: Coordinate,
    attach: TypeAndIDDescriptor,
    offset: [f32;3],
    scale: f32,
    entity_face_object: TypeAndIDDescriptor
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct RandomNameDatabase {
    species_modification_prefix: Vec<String>,
    species_modification_postfix: Vec<String>,
    star_names: Vec<String>,
    black_hole_names: Vec<String>,
    nebula_names: Vec<String>,
    asteroid_prefix: Vec<String>,
    asteroid_postfix: Vec<String>, // TODO This is repeated a huge number of times and with identical data, possible bug?
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct SavedEventTarget {
    r#type: SavedEventTargetType,
    id: u64,
    name: String
}
#[derive(Serialize, Deserialize)]
enum SavedEventTargetType { Country, Fleet, Planet, GalacticObject }

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct ShipDesign {
    name: String,
    ship_size: String,
    section: Option<Vec<ShipDesignSection>>,
    auto_gen_design: bool,
    initial_design: Option<bool>,
    required_component: Option<Vec<String>>
}
#[derive(Serialize, Deserialize)]
struct ShipDesignSection {
    template: String,
    slot: String,
    component: Vec<ShipDesignSectionComponent>
}
#[derive(Serialize, Deserialize)]
struct ShipDesignSectionComponent {
    slot: String,
    template: String
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Megastructure {
    r#type: String,
    coordinate: Coordinate,
    planet: u64,
    bypass: u64,
    orbitals: Vec<u64>, // TODO This is placeholder value
    build_queue: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Bypass {
    r#type: String,
    active: bool,
    linked_to: u64,
    connections: Vec<u64>,
    active_connections: Vec<u64>,
    owner: TypeAndIDDescriptor
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Wormhole {
    coordinate: Coordinate,
    bypass: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Sector {
    name: String,
    systems: Vec<u64>,
    local_capital: u64,
    governor: u64,
    owner: u64,
    resources: u64,
    r#stype: String
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Building {
    r#type: String
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct ArchaeologicalSites {
    sites: VecDeque<Site>
}
#[derive(Serialize, Deserialize)]
struct Site {
    location: TypeAndIDDescriptor,
    last_excavator_country: u64,
    excavator_fleet: u64,
    r#type: String,
    index: u64,
    clues: u64,
    last_roll: u64,
    days_left: u64,
    difficulty: u64,
    locked: bool,
    visible_to: Vec<u64>
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct GlobalShipDesign {
    name: String,
    ship_design: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Cluster {
    id: String,
    position: Coordinate,
    radius: u64,
    objects: Vec<u64>
}

// -------------------------------------------------
#[derive(Serialize, Deserialize, Hash,Eq,PartialEq)]
struct ClassWithOptionalValues {
    class: String,
    values: Option<Vec<u64>>
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Market {
    id: Vec<u64>,
    next_monthly_trade_item_id: u64,
    country: u64
}


// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct GalacticCommunity {
    election: u64
}
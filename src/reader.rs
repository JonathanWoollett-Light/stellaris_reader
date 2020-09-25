use time::Date;
use std::collections::VecDeque;
use crate::{
    galactic_object::GalacticObject,
    country::Country,
    federation::Federation,
    fleet::Fleet,
    shared::{Coordinate,Flag,Ethos,Orbital,TypeAndIDDescriptor,EmpireFlag}
};

// Using `VecDeqeque` for sequentially numbered items (0..n)

// The fact like some of this shit is plural and some aint is real dumb

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

struct FleetTemplate {
    fleet: u64,
    home_base: Orbital,
    fleet_template_design: Vec<FleetTemplateDesign>,
    all_queued: Vec<u64>, // TODO This is a placeholder value, find actual value
    count: u64,
    fleet_size: u64
}
struct FleetTemplateDesign {
    design: u64,
    count: u64
}

// -------------------------------------------------

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

struct Deposit {
    r#type: String,
    swap_type: Option<String>,
    planet: u64
}

// -------------------------------------------------

struct AmbientObject {
    coordinate: Coordinate,
    data: String,
    properties: AmbientObjectProperties
}

struct AmbientObjectProperties {
    coordinate: Coordinate,
    attach: TypeAndIDDescriptor,
    offset: [f32;3],
    scale: f32,
    entity_face_object: TypeAndIDDescriptor
}

// -------------------------------------------------

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

struct Galaxy {
    template: String,
    shape: GalaxyShape,
    num_empires: u64,
    num_advanced_empires: u64,
    num_fallen_empires: u64,
    num_marauder_empires: u64,
    habitability: u64,
    primitive: u64,
    advanced_start_near_player: bool,
    caravaneers_enabled: bool,
    xeno_compatibility_enabled: bool,
    scaling: bool,
    crises: f32,
    technology: f32,
    clustered: bool,
    random_empires: bool,
    random_fallen_empires: bool,
    random_advanced_empires: bool,
    core_radius: u64,
    player_locations: GalaxyPlayerLocations,
    difficulty: GalaxyDifficulty,
    aggressiveness: GalaxyAggressiveness,
    name: String,
    ironman: bool,
    num_gateways: f32,
    num_wormhole_pairs: f32,
    num_hyperlanes: f32,
    mid_game_start: u64,
    end_game_start: u64,
    victory_year: u64,
    num_guaranteed_colonies: u64,
    design: Vec<GalaxyDesign>
}

enum GalaxyShape { Elliptical }
enum GalaxyPlayerLocations { Normal }
enum GalaxyDifficulty { Cadet, Ensign, Captain, Commodore, Admiral, GrandAdmiral }
enum GalaxyAggressiveness { Normal }

// Kinda a dumb fucking name when this struct is basically just a copy of the `Country` struct
struct GalaxyDesign {
    key: String,
    ship_pefix: String,
    species: GalaxyDesignSpecies,
    secondary_species: Option<GalaxyDesignSpecies>,
    name: String,
    adjective: String,
    authority: String,
    flags: Option<Vec<String>>,
    government: String,
    planet_name: String,
    planet_class: String,
    system_name: String,
    initilizer: String,
    graphical_culture: String,
    city_graphical_culture: String,
    empire_flag: EmpireFlag,
    ruler: GalaxyDesignRuler,
    spawn_as_fallen: bool,
    ignore_portrait_duplication: bool,
    room: String,
    spawn_enabled: bool,
    ethic: Vec<String>,
    civics: Vec<String>,
    origin: String
}
struct GalaxyDesignSpecies {
    class: String,
    protrait: String,
    name: String,
    plural: String,
    adjective: String,
    name_list: String,
    r#trait: Option<Vec<String>>
}
struct GalaxyDesignRuler {
    gender: Gender,
    name: String,
    portrait: String,
    texture: u64,
    hair: u64,
    clothes: u64,
    ruler_title: Option<String>,
    ruler_title_female: Option<String>,
    leader_class: String
}

// -------------------------------------------------

struct SavedEventTarget {
    r#type: SavedEventTargetType,
    id: u64,
    name: String
}

enum SavedEventTargetType { Country, Fleet, Planet, GalacticObject }

// -------------------------------------------------

struct ShipDesign {
    name: String,
    ship_size: String,
    section: Option<Vec<ShipDesignSection>>,
    auto_gen_design: bool,
    initial_design: Option<bool>,
    required_component: Option<Vec<String>>
}
struct ShipDesignSection {
    template: String,
    slot: String,
    component: Vec<ShipDesignSectionComponent>
}
struct ShipDesignSectionComponent {
    slot: String,
    template: String
}

// -------------------------------------------------

struct Megastructure {
    r#type: String,
    coordinate: Coordinate,
    planet: u64,
    bypass: u64,
    orbitals: Vec<u64>, // TODO This is placeholder value
    build_queue: u64
}

// -------------------------------------------------

struct Bypass
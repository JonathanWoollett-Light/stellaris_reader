use time::Date;
use crate::{
    shared::{Ethos,Flag,Resource},
    modules::Modules
};
use std::collections::VecDeque;

pub struct Country {
    flag: CountryFlag,
    color_index: i64,
    name: String,
    adjective: String,
    custom_name: Option<bool>, // If it's false, field is not present.
    tech_status: TechStatus,
    last_date_was_human: Date,
    last_date_war_lost: Date,
    last_date_at_war: Date,
    bugdet: Budget,
    events: Option<EventChain>, // TODO Should this be an option?
    terra_incognita: TerraIncognita,
    military_power: f32,
    economy_power: f32,
    victory_rank: u16,
    victory_score: f32,
    tech_power: f32,
    immigration: f32,
    emigration: f32,
    fleet_size: u64,
    empire_size: Option<u64>,
    new_colonies: u64,
    sapient: Option<u64>,
    graphical_culture: Option<String>,
    city_graphical_culture: Option<String>,
    room: Option<String>,
    ai: AI,
    capital: Option<u64>,
    species_index: u64,
    ethos: Option<Vec<Ethos>>,
    fleet_template_manager: Vec<Vec<u64>>, // TODO This is wonky I don't know how to best do it
    government: Option<Government>,
    personality: Option<String>,
    next_election: Date,
    government_date: Date,
    surveyed: Option<Vec<u64>>,
    visited_objects: Vec<u64>,
    intel_level: Vec<u64>,
    highest_intel_level: Vec<u64>,
    flags: Option<Vec<Flag>>,
    sensor_range_fleets: Vec<u64>,
    faction: Faction,
    name_list: Option<String>,
    ship_names: Option<Vec<(String,u64)>>,
    ruler: Option<u64>,
    control_groups: Option<VecDeque<ControlGroup>>,
    ship_prefix: Option<String>,
    active_policies: Vec<Policy>,
    policy_flags: Vec<PolicyFlag>,
    federation: Option<u64>,
    starting_system: Option<u64>,
    has_advisor: Option<bool>,
    shown_message_types: Option<Vec<String>>,
    owned_leaders: Vec<u64>,
    owned_fleets: Vec<u64>,
    traditions: Option<Vec<String>>,
    owned_armies: Option<Vec<u64>>,
    owned_planets: Option<Vec<u64>>,
    controlled_planets: Option<Vec<u64>>,
    ship_design: Vec<u64>,
    r#type: String,
    modules: Modules,
    initialized: bool,
    random_name_variables: Option<Counter>,
    relations_manager: Option<Vec<Relation>>,
    customization: String,
    last_changed_country_type: Date,
    usable_bypasses: Option<Vec<u64>>,
    hyperlane_systems: Vec<u64>,
    sectors: Vec<Sector>,
    given_value: u64,
    num_upgrade_starbase: u64,
    starbase_capacity: u64,
    edict_capacity: u64,
    employable_pops: Option<u64>,
    owned_species: Option<Vec<u64>>
}

// --------------------------------------------

struct CountryFlag {
    icon: Image,
    background: Image,
    colors: [Option<String>;4] // Always has 4 strings, but they can be "null"
}
struct Image {
    catagory: String,
    file: String,
}

// --------------------------------------------

struct TechStatus {
    techs: Vec<Tech>,
    physics_queue: Option<TechQueued>,
    society_queue: Option<TechQueued>,
    engineering_queue: Option<TechQueued>,
    stored_techpoints: [f32;3],
    alternatives: TechAlternatives,
    potential: Option<Vec<(String,u64)>>,
    leaders: Option<[u64;3]>,
    always_available_tech: Option<String>,
    last_increased_tech: Option<String>,
}
struct Tech {
    technology: String,
    level: u64
}
struct TechAlternatives {
    physics: Vec<String>,
    society: Vec<String>,
    engineering: Vec<String>
}
struct TechQueued {
    progress: f32,
    technology: String,
    date: Date
}

// --------------------------------------------

struct Budget {
    current_month: CurrentMonth,
    last_month: LastMonth,
}
struct CurrentMonth {
    income: Vec<IncomeExpenseAffect>,
    expenses: Vec<IncomeExpenseAffect>,
    balance: Vec<IncomeExpenseAffect>
}
struct LastMonth {
    income: Vec<IncomeExpenseAffect>,
    expenses: Vec<IncomeExpenseAffect>,
    balance: Vec<IncomeExpenseAffect>,
    extra_income: Vec<IncomeExpenseAffect>,
    extra_expenses: Vec<IncomeExpenseAffect>,
    extra_balance: Vec<IncomeExpenseAffect>
}
struct IncomeExpenseAffect {
    name: String,
    quantities: Vec<Resource>
}

// --------------------------------------------

struct EventChain {
    event_chain: String,
    scope: EventScope
}
struct EventScope {
    r#type: EventType,
    id: u64,
    random: [f32;2],
    root: EventScopeRoot,
    from: EventScopeRoot,
    prev: EventScopeRoot
}
struct EventScopeRoot {
    r#type: EventType,
    id: u64,
    random: [f32;2]
}
enum EventType { Country,Leader }

// --------------------------------------------

struct Research {
    physics: f32,
    society: f32,
    engineer: f32
}

// --------------------------------------------

struct TerraIncognita {
    size: u64,
    data: Vec<u64>,
    systems: Vec<u64>
}

// --------------------------------------------

struct AI {
    initialized: bool,
    budget: [f32;4],
    prepare_war_date: Date,
    robot_colonies: u64,
    robot_colonies_with_free_buildings: u64,
    attitude: Option<Vec<Attitude>>,
    random_seed: u64,
    random_count: u64,
    synced_random_seed: u64,
    synced_random_count: u64,
}

struct Attitude {
    country: u64,
    attitude: String,
    weight: u64,
    priority: u64,
    date: Date
}

// --------------------------------------------

struct Government {
    r#type: String,
    authrity: String,
    civics: Vec<String>,
    origin: String
}

// --------------------------------------------

struct ControlGroup {
    r#type: u64,
    id: u64
}

// --------------------------------------------

struct Policy {
    policy: String,
    selected: String
}

// --------------------------------------------

enum PolicyFlag {
    DiploStancePrimitive, DiploStanceExpansionist,
    UnrestrictedWars,
    OrbitialBombardmentIndiscriminate,
    RseettlementAllowed,
    FirstContactAttackAllowed,
    BorderPolicyOpen, BorderPolicyClosed,
    RobotsAllowed, RobotsOutlawed,
    PopulationControlsAllowed, PopulationControlsNotAllowed,
    SlaveryNotAllowed,
    PurgeDisplacementOnly, PurgeNotAllowed,
    AppropriationNotAllowed,
    RefugeesAllowed
}

// --------------------------------------------

struct Sector {
    resources: u64,
    monthly_transfer: Vec<f32>, // placeholder, I don't know what goes here
    owned: Vec<f32>
}

// --------------------------------------------

struct Faction {
    hostile: Option<bool>,
    needs_border_access: Option<bool>,
    generate_borders: Option<bool>,
    needs_colony: Option<bool>,
    aggro_range: i64,
    aggro_range_measure_from: FactionRangeMeasure,
    primitive: Option<bool>,
    primitive_age: Option<String>
}
enum FactionRangeMeasure { This } // "This" is a replacement for "Self" since Rust doesn't allow "Self"

// --------------------------------------------

struct Counter {
    pre_communications_name: preCommunicationsName 
}
struct preCommunicationsName {
    second: u64
}

// --------------------------------------------

struct Relation {
    owner: u64,
    country: u64,
    contact: bool,
    alliance: bool,
    borders: bool,
    border_range: u64,
    communications: bool,
    diplo_action_dates: Vec<Date>
}
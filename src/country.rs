use time::Date;
use crate::shared::Ethos;
use std::collections::VecDeque;

pub struct Country {
    flag: CountryFlag,
    color_index: i64,
    name: String,
    adjective: String,
    custom_name: bool,
    tech_status: TechStatus,
    last_date_was_human: Date,
    last_date_war_lost: Date,
    last_date_at_war: Date,
    bugdet: Budget,
    events: Vec<u64>, // u64 is placeholder, I have no idea what type is here
    terra_incognita: TerraIncognita,
    military_power: f32,
    economy_power: f32,
    victory_rank: u16,
    victory_score: f32,
    tech_power: f32,
    immigration: f32,
    emigration: f32,
    fleet_size: u64,
    empire_size: u64,
    new_colonies: u64,
    sapient: u64,
    graphical_culture: String,
    city_graphical_culture: String,
    room: String,
    ai: AI,
    capital: u64,
    species_index: u64,
    ethos: Vec<Ethos>,
    fleet_template_manager: (String,u64), // Need to look into this one more
    government: Government,
    personality: String,
    next_election: Date,
    government_date: Date,
    surveyed: Vec<u64>,
    visited_objects: Vec<u64>,
    intel_level: Vec<u64>,
    highest_intel_level: Vec<u64>,
    flags: Vec<u64>,
    control_groups: VecDeque<ControlGroup>,
    ship_prefix: String,
    active_policies: Vec<Policy>,
    policy_flags: Vec<String>,
    starting_system: u64,
    has_advisor: bool,
    shown_message_types: Vec<String>,
    owned_leaders: Vec<u64>,
    owned_fleets: Vec<u64>,
    owned_armies: Vec<u64>,
    owned_planets: Vec<u64>,
    controlled_planets: Vec<u64>,
    ship_design: Vec<u64>,
    r#type: String,
    modules: Modules
}
struct CountryFlag {
    icon: Image,
    background: Image,
    colors: [Option<String>;4]
}
struct Image {
    catagory: String,
    file: String,
}
struct TechStatus {
    techs: Vec<Tech>,
    stored_techpoints: [f32;3],
    alternatives: TechAlternatives,
    potential: Vec<(String,u64)>,
    leaders: [u64;3],
    always_available_tech: String,
    last_increased_tech: String,
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
struct Resource {
    name: String,
    amount: f32
}
struct Research {
    physics: f32,
    society: f32,
    engineer: f32
}

struct TerraIncognita {
    size: u64,
    data: Vec<u64>,
    systems: Vec<u64>
}

struct AI {
    initialized: bool,
    budgetL: [f32;4],
    prepare_war_date: Date,
    robot_colonies: u64,
    robot_colonies_with_free_buildings: u64,
    random_seed: u64,
    random_count: u64,
    synced_random_seed: u64,
    synced_random_count: u64,
}

struct Government {
    r#type: String,
    authrity: String,
    civics: Vec<String>,
    origin: String
}

struct ControlGroup {
    r#type: u64,
    id: u64
}

struct Policy {
    policy: String,
    selected: String
}

struct Modules {
    standard_event_module: StandardEventModule
}
struct StandardEventModule {
    delayed_events: Vec<DelayedEvent>
}
struct DelayedEvent {
    
}
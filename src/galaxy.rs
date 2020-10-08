use serde::{Deserialize, Serialize};

use crate::shared::{EmpireFlag, Gender};

#[derive(Serialize, Deserialize)]
pub struct Galaxy {
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
#[derive(Serialize, Deserialize)]
enum GalaxyShape { Elliptical }
#[derive(Serialize, Deserialize)]
enum GalaxyPlayerLocations { Normal }
#[derive(Serialize, Deserialize)]
enum GalaxyDifficulty { Cadet, Ensign, Captain, Commodore, Admiral, GrandAdmiral }
#[derive(Serialize, Deserialize)]
enum GalaxyAggressiveness { Normal }

// Kinda a dumb fucking name when this struct is basically just a copy of the `Country` struct
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
struct GalaxyDesignSpecies {
    class: String,
    protrait: String,
    name: String,
    plural: String,
    adjective: String,
    name_list: String,
    r#trait: Option<Vec<String>>
}
#[derive(Serialize, Deserialize)]
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
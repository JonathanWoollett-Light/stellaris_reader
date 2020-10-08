use serde::{Deserialize, Serialize};
use time::Date;

use crate::shared::Gender;

#[derive(Serialize, Deserialize)]
pub struct Leader{
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

#[derive(Serialize, Deserialize)]
struct LeaderName {
    first_name: String,
    second_name: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Location {
    r#type: LocationType,
    area: Option<LocationArea>,
    assignment: Option<u64>, // u64 is placeholder
    id: u64
}
#[derive(Serialize, Deserialize)]
enum LocationType { Sector, Planet, Tech, Ship }
#[derive(Serialize, Deserialize)]
enum LocationArea { Physics, Engineering, Society }

#[derive(Serialize, Deserialize)]
struct LeaderDesign {
    gender: Gender,
    name: String,
    protrait: String,
    texture: u64,
    hair: u64,
    clothes: u64,
    leader_class: String
}

#[derive(Serialize, Deserialize)]
struct LeaderRoles {
    admiral: Vec<String>,
    general: Vec<String>,
    scientist: Vec<String>,
    governor: Vec<String>,
    ruler: Vec<String>
}
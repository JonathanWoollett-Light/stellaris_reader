use serde::{Deserialize, Serialize};

use crate::shared::{Coordinate,Flag};

#[derive(Serialize, Deserialize)]
pub struct GalacticObject {
    coordinate: Coordinate,
    r#type: GalacticObjectType,
    name: String,
    planets: Vec<u64>,
    ambient_object: Option<Vec<u64>>,
    megastructures: Option<Vec<u64>>,
    star_class: String,
    hyperlanes: Vec<HyperLane>,
    asteroid_belts: Option<Vec<AsteroidBelt>>,
    natural_wormholes: u64,
    bypasses: u64,
    flags: Vec<Flag>,
    initializer: String,
    init_parent: Option<u64>,
    fleet_presence: Option<Vec<u64>>,
    inner_radius: u64,
    out_radius: u64,
    starbase: u64,
    trade_hub: TradeHub,
    trade_collection: Option<Target>,
    trade_piracy: TradePiracy,
    sector: u64
}
#[derive(Serialize, Deserialize)]
enum GalacticObjectType { Star }
#[derive(Serialize, Deserialize)]
struct HyperLane {
    to: u64,
    length: u64,
    brigde: Option<bool>
}
#[derive(Serialize, Deserialize)]
struct TradeHub {
    collected: u64,
    collected_from: Option<u64>
}
#[derive(Serialize, Deserialize)]
struct TradePiracy {
    throughput: f32,
    max: f32,
    active: f32,
    used: f32,
    targets: Option<Target>
}
#[derive(Serialize, Deserialize)]
struct Target {
    target: u64,
    distance: u64,
}
#[derive(Serialize, Deserialize)]
struct AsteroidBelt {
    r#type: String,
    inner_radius: u64
}

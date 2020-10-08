use serde::{Deserialize, Serialize};
use time::Date;

use crate::shared::{Coordinate,Orbital};

#[derive(Serialize, Deserialize)]
pub struct Fleet {
    name: String,
    fleet_template: Option<u64>,
    ships: Vec<u64>,
    combat: FleetCombat,
    fleet_stats: FleetStats,
    current_order: Option<FleetOrder>,
    owner: u64,
    station: Option<bool>,
    civilian: Option<bool>,
    ground_support_stance: String,
    fleet_stance: FleetStance,
    mia_from: Coordinate,
    movement_manager: FleetMovementManager,
    order_id:  Option<u64>,
    friends_should_follow: Option<bool>,
    mobile: Option<bool>,
    hit_points: f32,
    military_power: f32,
    cached_killed_ships: u64,
    cahced_dead_ships: u64,
    cached_disabled_ships: u64,
    cached_disengaged_ships: u64,
    cached_combined_removed_ships: u64
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct FleetCombat {
    coordinate: Coordinate,
    formation_pos: FleetCombatFormationPosition,
    formation: FleetCombatFormation,
    start_coordinate: Coordinate,
    start_date: Date
}
#[derive(Serialize, Deserialize)]
struct FleetCombatFormationPosition {
    x: f32,
    y: f32,
    speed: f32,
    rotation: f32,
    forward_x: f32,
    forward_y: f32
}
#[derive(Serialize, Deserialize)]
struct FleetCombatFormation { root: i64 /* TODO Should this be an f32? */ }

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct FleetStats { combat_stats: FleetStatsCombatStats }
#[derive(Serialize, Deserialize)]
struct FleetStatsCombatStats {
    fleet: Vec<u64>, // This is placeholder type
    date: Date
}

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct FleetOrder {
    order: FleetOrderType,
    sub_order: Option<Box<FleetOrder>>
}
#[derive(Serialize, Deserialize)]
enum FleetOrderType {
    AssistResearchOrder(AssistResearchOrder),
    OrbitPlanetOrder(OrbitPlanetOrder),
    MoveToSystemPointOrder(MoveToSystemPointOrder)
}
#[derive(Serialize, Deserialize)]
struct AssistResearchOrder { planet: u64, order_id: u64 } // TODO Should `order_id` be an option in parent struct?
#[derive(Serialize, Deserialize)]
struct OrbitPlanetOrder { orbitable: Orbital }
#[derive(Serialize, Deserialize)]
struct MoveToSystemPointOrder { coordinate: Coordinate }

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
enum FleetStance { Evasive }

// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct FleetMovementManager {
    formation: FleetMovementManagerFormation,
    coordinate: Coordinate,
    target: FleetMovementManagerTarget,
    target_coordinate: Coordinate,
    time_since_last_path_update: Option<u64>,
    state: FleetMovementManagerState,
    path: FleetMovementManagerPath,
    orbit: FleetMovementManagerOrbit,
    last_ftl_jump: FleetMovementManagerLastFTLJump
}
#[derive(Serialize, Deserialize)]
struct FleetMovementManagerFormation {
    scale: f32,
    angle: f32,
    r#type: FleetMovementManagerFormationType
}
#[derive(Serialize, Deserialize)]
enum FleetMovementManagerFormationType { Circle, Wedge }

#[derive(Serialize, Deserialize)]
struct FleetMovementManagerTarget { coordinate: Coordinate }
#[derive(Serialize, Deserialize)]
enum FleetMovementManagerState { MoveIdle, MoveSystem }
#[derive(Serialize, Deserialize)]
struct FleetMovementManagerPath {
    node: Option<FleetMovementManagerPathNode>,
    date: Date
}
#[derive(Serialize, Deserialize)]
struct FleetMovementManagerPathNode {
    coordinate: Coordinate,
    ftl: FleetMovementManagerPathNodeFTL
}
#[derive(Serialize, Deserialize)]
enum FleetMovementManagerPathNodeFTL { JumpHyperlane }

#[derive(Serialize, Deserialize)]
struct FleetMovementManagerOrbit {
    orbitable: Option<Orbital>,
    index: Option<i64>
}

#[derive(Serialize, Deserialize)]
struct FleetMovementManagerLastFTLJump {
    from: Coordinate,
    to: u64,
    fleet: u64,
    jump_method: FleetMovementManagerLastFTLJumpJumpMethod,
    bypass_from: u64,
    bypass_to: u64
}
#[derive(Serialize, Deserialize)]
enum FleetMovementManagerLastFTLJumpJumpMethod { JumpCount }

// -------------------------------------------------
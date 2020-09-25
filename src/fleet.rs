use crate::shared::{Coordinate,Orbital};
use time::Date;

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

struct FleetCombat {
    coordinate: Coordinate,
    formation_pos: FleetCombatFormationPosition,
    formation: FleetCombatFormation,
    start_coordinate: Coordinate,
    start_date: Date
}
struct FleetCombatFormationPosition {
    x: f32,
    y: f32,
    speed: f32,
    rotation: f32,
    forward_x: f32,
    forward_y: f32
}
struct FleetCombatFormation { root: i64 /* TODO Should this be an f32? */ }

// -------------------------------------------------

struct FleetStats { combat_stats: FleetStatsCombatStats }
struct FleetStatsCombatStats {
    fleet: Vec<u64>, // This is placeholder type
    date: Date
}

// -------------------------------------------------



// -------------------------------------------------

struct FleetOrder {
    order: FleetOrderType,
    sub_order: Option<Box<FleetOrder>>
}
enum FleetOrderType {
    AssistResearchOrder(AssistResearchOrder),
    OrbitPlanetOrder(OrbitPlanetOrder),
    MoveToSystemPointOrder(MoveToSystemPointOrder)
}
struct AssistResearchOrder { planet: u64, order_id: u64 } // TODO Should `order_id` be an option in parent struct?
struct OrbitPlanetOrder { orbitable: Orbital }
struct MoveToSystemPointOrder { coordinate: Coordinate }

// -------------------------------------------------

enum FleetStance { Evasive }

// -------------------------------------------------

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

struct FleetMovementManagerFormation {
    scale: f32,
    angle: f32,
    r#type: FleetMovementManagerFormationType
}
enum FleetMovementManagerFormationType { Circle, Wedge }

struct FleetMovementManagerTarget { coordinate: Coordinate }

enum FleetMovementManagerState { MoveIdle, MoveSystem }

struct FleetMovementManagerPath {
    node: Option<FleetMovementManagerPathNode>,
    date: Date
}
struct FleetMovementManagerPathNode {
    coordinate: Coordinate,
    ftl: FleetMovementManagerPathNodeFTL
}
enum FleetMovementManagerPathNodeFTL { JumpHyperlane }

struct FleetMovementManagerOrbit {
    orbitable: Option<Orbital>,
    index: Option<i64>
}

struct FleetMovementManagerLastFTLJump {
    from: Coordinate,
    to: u64,
    fleet: u64,
    jump_method: FleetMovementManagerLastFTLJumpJumpMethod,
    bypass_from: u64,
    bypass_to: u64
}
enum FleetMovementManagerLastFTLJumpJumpMethod { JumpCount }

// -------------------------------------------------
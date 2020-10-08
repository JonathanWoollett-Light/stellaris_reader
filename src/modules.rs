use serde::{Deserialize, Serialize};
use time::Date;

use crate::shared::Resource;

#[derive(Serialize, Deserialize)]
pub struct Modules {
    standard_event_module: StandardEventModule,
    standard_economy_module: Option<StandardEconomyModule>,
    standard_leader_module: Option<StandardLeaderModule>,
    standard_diplomacy_module: Option<StandardDiplomacyModule>,
    standard_technology_module: Option<StandardTechnologyModule>,
    standard_pop_factions_module: Option<StandardPopFactionsModule>,
    standard_expansion_module: Option<StandardExpansionModule>,
    standard_species_rights_module: Option<StandardSpeciesRightsModule>,
    standard_trade_routes_module: Option<StandardTradeRoutesModule>,

    exclusive_diplomacy_module: Option<ExclusiveDiplomacyModule>,
    basic_technology_module: Option<BasicTechnologyModule>
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardEventModule {
    delayed_events: Option<Vec<DelayedEvent>>
}
#[derive(Serialize, Deserialize)]
struct DelayedEvent {
    event: String,
    days: u64,
    scope: EventScope
}
#[derive(Serialize, Deserialize)]
struct EventScope {
    r#type: EventScopeType,
    id: u64,
    random: [u64;2],
    from: Option<Box<EventScope>>,
    saved_event_targets: Option<Vec<EventTarget>>
}
#[derive(Serialize, Deserialize)]
enum EventScopeType { Country, Planet, Species, ArchaeologicalSite }

#[derive(Serialize, Deserialize)]
struct EventTarget {
    r#type: EventScopeType,
    id: u64,
    name: String
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardEconomyModule {
    resources: Vec<Resource>
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardLeaderModule {
    enabled: bool,
    leaders: Vec<u64>
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardDiplomacyModule {
    borders: u64
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardTechnologyModule {
    // This can just be empty?
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardPopFactionsModule {
    last_created: Date,
    potential_count: Vec<u64>,
    total_faction_members: u64
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardExpansionModule {
    // This can just be empty?
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardSpeciesRightsModule {
    enabled: bool,
    default: SpeciesRights,
    primary: SpeciesRights,
    built_species: SpeciesRights
}
#[derive(Serialize, Deserialize)]
struct SpeciesRights {
    species: u64,
    living_standard: LivingStandard,
    citizenship: Citizenship,
    military_service: MilitaryService,
    slvery: Slavery,
    purge: Purge,
    population_control: PopulationControl,
    colonization_control: ColonizationControl,
    migration_control: MigrationControl,

    former_living_standard: LivingStandard,
    former_citizenship: Citizenship,
    former_military_service: MilitaryService,
    former_slvery: Slavery,
    former_purge: Purge,
    former_population_control: PopulationControl,
    former_colonization_control: ColonizationControl,
    former_migration_control: MigrationControl,

    last_changed_living_standard: Date,
    last_changed_citizenship: Date,
    last_changed_military_service: Date,
    last_changed_slvery: Date,
    last_changed_purge: Date,
    last_changed_population_control: Date,
    last_changed_colonization_control: Date,
    last_changed_migration_control: Date
}
#[derive(Serialize, Deserialize)]
enum LivingStandard { LivingStandardStratified, LivingStandardNormal }
#[derive(Serialize, Deserialize)]
enum Citizenship { CitizenshipFull }
#[derive(Serialize, Deserialize)]
enum MilitaryService { MilitaryServiceFull }
#[derive(Serialize, Deserialize)]
enum Slavery { SlaveryNormal }
#[derive(Serialize, Deserialize)]
enum Purge { PurgeDisplacement }
#[derive(Serialize, Deserialize)]
enum PopulationControl { PopulationControlNo }
#[derive(Serialize, Deserialize)]
enum ColonizationControl { ColonizationControlNo }
#[derive(Serialize, Deserialize)]
enum MigrationControl { MigrationControlNo }
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct StandardTradeRoutesModule {
    collected: f32,
    days: u64
}
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct ExclusiveDiplomacyModule {
    contact_rule: Option<ExclusiveDiplomacyModuleContactRule>
}
#[derive(Serialize, Deserialize)]
enum ExclusiveDiplomacyModuleContactRule { ScriptOnly }
// -------------------------------------------------
#[derive(Serialize, Deserialize)]
struct BasicTechnologyModule {
    // This can just be empty?
}
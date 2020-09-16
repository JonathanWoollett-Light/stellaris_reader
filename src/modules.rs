pub struct Modules {
    standard_event_module: StandardEventModule,
    standard_economy_module: StandardEconomyModule,
    standard_leader_module: StandardLeaderModule,
    standard_diplomacy_module: StandardDiplomacyModule,
    standard_technology_module: StandardTechnologyModule,
    standard_pop_factions_module: StandardPopFactionsModule,
    standard_expansion_module: StandardExpansionModule,
    standard_species_rights_module: StandardSpeciesRightsModule,
    standard_trade_routes_module: StandardTradeRoutesModule
}
// -------------------------------------------------
struct StandardEventModule {
    delayed_events: Vec<DelayedEvent>
}
struct DelayedEvent {
    event: String,
    days: u64,
    scope: EventScope
}
struct EventScope {
    r#type: EventScopeType,
    id: u64,
    random: [u64;2]
    from: EventFrom
}
enum EventScopeType { Country, Planet, Species }
struct EventFrom {
    r#type: Option<EventScopeType>,
    id: u64,
    random: [u64;2],
    saved_event_targets: Vec<EventTarget>
}
struct EventTarget {
    r#type: EventScopeType,
    id: u64,
    name: String
}
// -------------------------------------------------
struct StandardEconomyModule {
    resources: Vec<Resource>
}
// -------------------------------------------------
struct StandardLeaderModule {
    enabled: bool,
    leaders: Vec<u64>
}
// -------------------------------------------------
struct StandardDiplomacyModule {
    borders: u64
}
// -------------------------------------------------
struct StandardTechnologyModule {
    // This can just be empty?
}
// -------------------------------------------------
struct StandardPopFactionsModule {
    last_created: Date,
    potential_count: Vec<u64>,
    total_faction_members: u64
}
// -------------------------------------------------
struct StandardExpansionModule {
    // This can just be empty?
}
// -------------------------------------------------
struct StandardSpeciesRightsModule {
    enabled: bool,
    default: SpeciesRights,
    primary: SpeciesRights,
    built_species: SpeciesRights
}
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
enum LivingStandard { LivingStandardStratified, LivingStandardNormal }
enum Citizenship { CitizenshipFull }
enum MilitaryService { MilitaryServiceFull }
enum Slavery { SlaveryNormal }
enum Purge { PurgeDisplacement }
enum PopulationControl { PopulationControlNo }
enum ColonizationControl { ColonizationControlNo }
enum MigrationControl { MigrationControlNo }
// -------------------------------------------------
struct StandardTradeRoutesModule {
    collected: f32,
    days: u64
}
// -------------------------------------------------
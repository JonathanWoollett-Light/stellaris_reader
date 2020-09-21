use time::Date;
use crate::shared::Flag;

pub struct Federation {
    name: String,
    federation_progression: FederationProgression,
    members: Vec<u64>,
    start_date: Date,
    ship_designs: Vec<u64>,
    leader: u64
}
struct FederationProgression {
    federation_type: String,
    experience: f32,
    base_cohesion: i64, // This might be an f32
    cohesion: i64, // This might be an f32
    laws: Laws,
    action_settings: Vec<ActionSetting>,
    envoy: u64, // This may be a `Vec<u64>` need to check that
    levels: u64,
    perks: Vec<Perk>,
    timed_modifier: TimedModifier,
    succession_type: SuccessionType,
    succession_term: SuccessionTerm,
    last_succession_date: Date,
    expired: Date,
    flags: Vec<Flag>,
    only_leader_builds_fleets: bool,
    allow_subjects_to_join: bool,
    equal_voting_power: bool,
    free_migration: bool,
    research_agreement: bool,
    failed_vote_half_price: bool,
    research_sharing: bool
}

// --------------------------------------------

struct Laws {
    law_category_voting_weight: LawCategoryVotingWeight,
    law_category_separate_treaties: LawCategorySeparateTreaties,
    law_category_allow_subjects_to_join: LawCategoryAllowSubjectsToJoin,
    law_category_succession_power: LawCategorySuccessionPower,
    law_category_free_migration: LawCategoryFreeMigration,
    law_category_war_declaration_vote: LawCategoryWarDeclarationVote,
    law_category_succession_term: LawCategorySuccessionTerm,
    law_category_fleet_contribution: LawCategoryFleetContribution,
    law_category_kick_members_vote: LawCategoryKickMembersVote,
    law_category_invite_members_vote: LawCategoryInviteMembersVote,
    law_category_build_fleets: LawCategoryBuildFleets,
    law_category_succession_type: LawCategorySuccessionType
}
enum LawCategoryVotingWeight { VoteWeightEqual }
enum LawCategorySeparateTreaties { TreatiesSeparateYes }
enum LawCategoryAllowSubjectsToJoin { AllowSubjectsToJoinYes }
enum LawCategorySuccessionPower { SuccessionPowerDiplomaticWeight }
enum LawCategoryFreeMigration { FreeMigrationNo }
enum LawCategoryWarDeclarationVote { DeclareWarMajorityVoteHegemony }
enum LawCategorySuccessionTerm { SuccessionTermYears20 }
enum LawCategoryFleetContribution { FleetContributionLowHegemony }
enum LawCategoryKickMembersVote { KickMembersPresidentVoteHegemony }
enum LawCategoryInviteMembersVote { InviteMembersPresidentVoteHegemony }
enum LawCategoryCentralization { CentralizationMinimal }
enum LawCategoryBuildFleets { FederationBuildFleetsEveryone }
enum LawCategorySuccessionType { SuccessionTypeStrongestHegemony }

// --------------------------------------------

struct ActionSetting {
    action: String,
    vote_type: VoteType,
    acceptance_type: VoteType // TODO Is this the same enum? Double check this
}
enum VoteType { MajorityVote, Leader, Default }

// --------------------------------------------

struct Perk {
    level: u64,
    r#type: String
}

// --------------------------------------------

struct TimedModifier {
    modifier: String,
    days: u64
}

// --------------------------------------------

enum SuccessionType { DiplomaticWeight }

// --------------------------------------------

enum SuccessionTerm { Years20 }
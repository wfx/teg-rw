use serde::Deserialize;
use std::collections::HashMap;

/// Top-level structure representing a complete rule definition file
#[derive(Debug, Deserialize)]
pub struct RuleDefinition {
    /// Unique identifier for the rule set (e.g. "teg")
    pub id: String,
    /// Human-readable name (e.g. "TEG")
    pub name: String,
    /// Author of the ruleset
    pub author: String,
    /// Version string (semantic versioning preferred)
    pub version: String,
    /// Short description of the rule set
    pub description: String,
    /// Optional link to rules documentation or reference
    pub rules: String,
    /// Configurable parameters and setup behavior
    pub parameters: RuleParameters,
    /// Phase definition with allowed actions and transitions
    pub phases: HashMap<String, PhaseDefinition>,
    /// List of possible goals that can be assigned to players
    pub goals: Vec<GoalDefinition>,
}

/// Holds tunable parameters affecting game setup and rules
#[derive(Debug, Deserialize)]
pub struct RuleParameters {
    /// Name of the initial phase
    pub default_phase: String,
    /// Minimum number of players allowed
    pub min_players: u8,
    /// Maximum number of players allowed
    pub max_players: u8,
    /// Bonus figures per zone (set_id, bonus_amount)
    pub sets_bonus: Vec<(u8, u8)>,
    /// Placement-related configuration
    pub placement: PlacementConfig,
    /// Sequence of bonus figures for card trades (increasing)
    pub card_bonus_sequence: Vec<u8>,
}

/// Figure placement settings
#[derive(Debug, Deserialize)]
pub struct PlacementConfig {
    /// Number of figures per player in the setup round
    pub setup_round_figures: u8,
    /// Number of figures per player in the regular round
    pub regular_round_figures: u8,
    /// Whether bonuses for full zones apply during placement
    pub fieldsets_bonus: bool,
    /// When control bonuses are applied (e.g. "turn")
    pub control_bonus: String,
}

/// Describes a game phase, which may contain multiple actions
#[derive(Debug, Deserialize)]
pub struct PhaseDefinition {
    /// Mapping of action names to definitions (e.g. "place_figure": …)
    #[serde(flatten)]
    pub actions: HashMap<String, ActionDefinition>,
}

/// Defines what happens when an action is executed in a phase
#[derive(Debug, Deserialize)]
pub struct ActionDefinition {
    /// Mapping from result strings to next phase names
    pub result: HashMap<String, String>,
    /// Optional constraints (number, boolean, string values)
    #[serde(default)]
    pub constraints: Option<HashMap<String, ConstraintValue>>,
}

/// Flexible enum to represent different constraint types
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ConstraintValue {
    /// Boolean constraint (e.g. true/false)
    Bool(bool),
    /// Numeric constraint (e.g. min_players: 2)
    Number(u8),
    /// String-based constraint (e.g. "turn")
    String(String),
}

/// Defines a player goal – either simple or with fallback logic
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GoalDefinition {
    /// Simple goal without fallback
    Simple {
        name: String,
        #[serde(rename = "type")]
        goal_type: String,
        #[serde(default)]
        sets: Option<Vec<u8>>,
        #[serde(default)]
        min_total_figures: Option<u8>,
        #[serde(default)]
        field_count: Option<u8>,
        #[serde(default)]
        target_player: Option<u8>,
    },
    /// Goal with a fallback (e.g. if remove fails, control total)
    WithFallback {
        name: String,
        #[serde(rename = "type")]
        goal_type: String,
        target_player: u8,
        fallback_goal: FallbackGoal,
    },
}

/// Fallback goal definition (used inside `WithFallback`)
#[derive(Debug, Deserialize)]
pub struct FallbackGoal {
    #[serde(rename = "type")]
    pub goal_type: String,
    pub field_count: u8,
}

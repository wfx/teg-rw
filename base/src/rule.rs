use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::Path};
use thiserror::Error;

/// Errors that can occur loading or validating rule data.
#[derive(Debug, Error)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON parse error: {0}")]
    Ron(#[from] ron::error::SpannedError),
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Top-level rule set definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    /// Unique identifier for the rule set (e.g., variant name).
    pub id: String,
    /// List of phases in this rule set.
    #[serde(rename = "phases")]
    pub phases: Vec<PhaseDefinition>,
}

/// Definition of a single phase in the game flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseDefinition {
    /// Unique phase identifier.
    pub id: String,
    /// Actions available in this phase.
    #[serde(default)]
    pub actions: Vec<ActionDefinition>,
    /// Next phase to transition to after this one (optional).
    pub next_phase: Option<String>,
}

/// Definition of an action within a phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDefinition {
    /// Kind of action (e.g., "Place", "Interact").
    pub kind: String,
    /// Constraints that must be satisfied to perform the action.
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    // Resulting state changes when the action is executed.
    #[serde(default)]
    pub result: ActionResult,
}

/// A constraint on an action (field condition, minimum value, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Constraint {
    pub field: String,
    pub value: ron::Value,
}

/// The result of an action, consisting of state changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionResult {
    #[serde(rename = "changes")]
    pub state_changes: Vec<StateChange>,
}

/// Possible state changes triggered by an action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum StateChange {
    MoveFigures {
        from: String,
        to: String,
        count: u32,
    },
    ChangeOwner {
        field_id: String,
        new_owner: String,
    },
    // TODO: weitere StateChanges ergänzen
}

impl RuleSet {
    /// Load rule set from a .ron file, parse and validate.
    pub fn load(path: &Path) -> Result<Self, DataError> {
        let raw = fs::read_to_string(path)?;
        let ruleset: RuleSet = ron::from_str(&raw)?;
        ruleset.validate()?;
        Ok(ruleset)
    }

    /// Validate uniqueness and consistency of the rule set.
    pub fn validate(&self) -> Result<(), DataError> {
        // Ensure unique rule set id
        if self.id.trim().is_empty() {
            return Err(DataError::Validation("RuleSet id must not be empty".into()));
        }
        // Unique phase ids
        let mut seen = HashSet::new();
        for phase in &self.phases {
            if !seen.insert(&phase.id) {
                return Err(DataError::Validation(format!(
                    "Duplicate phase id: {}",
                    phase.id
                )));
            }
        }
        // Validate next_phase references
        for phase in &self.phases {
            if let Some(ref np) = phase.next_phase {
                if !seen.contains(np) {
                    return Err(DataError::Validation(format!(
                        "Phase '{}' references unknown next_phase: {}",
                        phase.id, np
                    )));
                }
            }
        }
        Ok(())
    }
}

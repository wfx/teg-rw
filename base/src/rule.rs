use crate::DataError;
use crate::Validatable;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
//use thiserror::Error;

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

impl Validatable for RuleSet {
    /// Validate uniqueness and consistency of the rule set.
    fn validate(&self) -> Result<(), DataError> {
        if self.id.trim().is_empty() {
            return Err(DataError::Validation("RuleSet id must not be empty".into()));
        }

        let mut seen = HashSet::new();
        for phase in &self.phases {
            if !seen.insert(&phase.id) {
                return Err(DataError::Validation(format!(
                    "Duplicate phase id: {}",
                    phase.id
                )));
            }
        }

        for phase in &self.phases {
            if let Some(ref np) = phase.next_phase {
                if !seen.contains(np) {
                    return Err(DataError::Validation(format!(
                        "Phase '{}' references unknown next_phase: {}",
                        phase.id, np
                    )));
                }
            }

            for action in &phase.actions {
                // Prüfe, ob "kind" zulässig ist (Whitelist)
                let allowed = [
                    "assign_fields",
                    "assign_goals",
                    "place_figure",
                    "calculate_gain",
                    "gain_figures",
                    "encounter",
                    "change_ownership",
                    "redistribute_figures",
                    "check_card_reward",
                    "draw_field_card",
                    "end_phase",
                ];
                if !allowed.contains(&action.kind.as_str()) {
                    return Err(DataError::Validation(format!(
                        "Invalid action kind in phase '{}': '{}'",
                        phase.id, action.kind
                    )));
                }

                // Optionale Prüfung der Constraints
                for constraint in &action.constraints {
                    if constraint.field.trim().is_empty() {
                        return Err(DataError::Validation(format!(
                            "Empty constraint field in action '{}'",
                            action.kind
                        )));
                    }
                    // Optional: type check for constraint.value?
                }

                // Prüfe Result-Änderungen
                for change in &action.result.state_changes {
                    match change {
                        StateChange::MoveFigures { from, to, count } => {
                            if from == to {
                                return Err(DataError::Validation(
                                    "MoveFigures: from and to must differ".into(),
                                ));
                            }
                            if *count == 0 {
                                return Err(DataError::Validation(
                                    "MoveFigures: count must be > 0".into(),
                                ));
                            }
                        }
                        StateChange::ChangeOwner {
                            field_id,
                            new_owner,
                        } => {
                            if field_id.trim().is_empty() || new_owner.trim().is_empty() {
                                return Err(DataError::Validation(
                                    "ChangeOwner: field_id and new_owner must be set".into(),
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_rule {
    use super::*;
    use crate::loader::load_ron;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn validate_all_rule_files() {
        //let dir = "wad/game";
        let manifest = env!("CARGO_MANIFEST_DIR");
        let dir: PathBuf = Path::new(manifest).join("..").join("wad").join("game");

        assert!(
            dir.exists(),
            "wad/game directory does not exist: {}",
            dir.display()
        );

        let mut failed = vec![];

        for entry in fs::read_dir(dir).expect("Failed to read wad/game directory") {
            let entry = entry.expect("Invalid dir entry");
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "ron")
                && path
                    .file_name()
                    .map_or(false, |name| name.to_string_lossy().ends_with(".rule.ron"))
            {
                match load_ron::<RuleSet, _>(&path) {
                    Ok(_) => { /* success */ }
                    Err(e) => failed.push((path.display().to_string(), e.to_string())),
                }
            }
        }

        if !failed.is_empty() {
            for (file, err) in &failed {
                eprintln!("❌ Validation failed for {}:\n{}", file, err);
            }
            panic!("{} rule file(s) failed validation", failed.len());
        }
    }
}

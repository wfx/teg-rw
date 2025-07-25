use serde::Deserialize;
use std::collections::HashMap;

// Event types that observers can listen to
#[derive(Debug, Clone)]
pub enum PhaseFlowEvent {
    PhaseChanged {
        from: String,
        to: String,
    },
    ActionExecuted {
        phase: String,
        action: String,
        result: String,
    },
    ConstraintChecked {
        phase: String,
        action: String,
        success: bool,
    },
}

// Observer trait that all listeners must implement
pub trait PhaseFlowObserver {
    fn on_event(&mut self, event: &PhaseFlowEvent);
}

// Main configuration from RON file
#[derive(Debug, Deserialize)]
pub struct PhaseFlowConfig {
    default_phase: String,
    phases: HashMap<String, PhaseActions>,
    goals: Vec<Goal>,
}

// Actions possible in a phase
#[derive(Debug, Deserialize)]
pub struct PhaseActions {
    #[serde(flatten)]
    actions: HashMap<String, ActionConfig>,
}

// Configuration for a single action
#[derive(Debug, Deserialize)]
pub struct ActionConfig {
    result: HashMap<String, String>, // from result -> next phase
    #[serde(default)]
    constraints: HashMap<String, Constraint>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Constraint {
    Number(i32),
    Boolean(bool),
}

// The actual Phase Flow Control
pub struct PhaseFlowControl {
    config: PhaseFlowConfig,
    current_phase: String,
    action_context: ActionContext,
    observers: Vec<Box<dyn PhaseFlowObserver>>,
}

pub struct ActionContext {
    // Context data here
}

impl PhaseFlowControl {
    pub fn new(config: PhaseFlowConfig) -> Self {
        Self {
            current_phase: config.default_phase.clone(),
            config,
            action_context: ActionContext::default(),
            observers: Vec::new(),
        }
    }

    // Add a new observer
    pub fn add_observer(&mut self, observer: Box<dyn PhaseFlowObserver>) {
        self.observers.push(observer);
    }

    // Notify all observers of an event
    fn notify_observers(&mut self, event: PhaseFlowEvent) {
        for observer in &mut self.observers {
            observer.on_event(&event);
        }
    }

    pub fn is_action_allowed(&self, action: &str) -> bool {
        self.config
            .phases
            .get(&self.current_phase)
            .and_then(|phase| phase.actions.get(action))
            .is_some()
    }

    pub fn check_constraints(&mut self, action: &str) -> Result<bool, String> {
        let constraints = self
            .config
            .phases
            .get(&self.current_phase)
            .and_then(|phase| phase.actions.get(action))
            .map(|action| &action.constraints)
            .ok_or("Action not found")?;

        // Implement constraint checking here
        let success = true; // Placeholder

        self.notify_observers(PhaseFlowEvent::ConstraintChecked {
            phase: self.current_phase.clone(),
            action: action.to_string(),
            success,
        });

        Ok(success)
    }

    pub fn execute_action(&mut self, action: &str, result: &str) -> Result<(), String> {
        let next_phase = self
            .config
            .phases
            .get(&self.current_phase)
            .and_then(|phase| phase.actions.get(action))
            .and_then(|action| action.result.get(result))
            .ok_or("Invalid action or result")?;

        // Notify about the action execution
        self.notify_observers(PhaseFlowEvent::ActionExecuted {
            phase: self.current_phase.clone(),
            action: action.to_string(),
            result: result.to_string(),
        });

        let old_phase = self.current_phase.clone();
        self.current_phase = next_phase.clone();

        // Notify about the phase change
        self.notify_observers(PhaseFlowEvent::PhaseChanged {
            from: old_phase,
            to: self.current_phase.clone(),
        });

        Ok(())
    }

    pub fn available_actions(&self) -> Vec<String> {
        self.config
            .phases
            .get(&self.current_phase)
            .map(|phase| phase.actions.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn current_phase(&self) -> &str {
        &self.current_phase
    }
}

use crate::field::FieldId;

/// A turn-based action initiated by a participant.
/// This enum describes all possible interactions with the game board,
/// regardless of the specific rule set used.
#[derive(Debug, Clone)]
pub enum TurnAction {
    /// Place a number of units onto a specific field.
    Place {
        /// Target field ID.
        to: FieldId,
        /// Number of units to place.
        count: u32,
    },

    /// Interact with a neighboring field according to the active rules.
    Interact {
        /// Originating field ID.
        from: FieldId,
        /// Target field ID.
        to: FieldId,
    },

    /// Move units from one field to another (usually under own control).
    Move {
        /// Source field ID.
        from: FieldId,
        /// Destination field ID.
        to: FieldId,
        /// Number of units to move.
        count: u32,
    },

    /// End the current participant's turn.
    EndTurn,
}

/// Trait for validating loaded data structures.
/// Each module should implement its own validation logic.
pub trait Validatable {
    /// Checks if the data is valid and consistent.
    /// Returns Ok(()) on success, or Err(String) with error message.
    fn validate(&self) -> Result<(), String>;
}

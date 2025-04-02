/// Trait for entities that can move on the hex grid
pub trait Moveable {
    /// Check if the entity can move to the given coordinates
    fn can_move_to(&self, q: i32, r: i32) -> bool;
    /// Move the entity to the given coordinates if possible
    /// Returns true if the move was successful
    fn move_to(&mut self, q: i32, r: i32) -> bool;
}

/// Trait for entities that can be interacted with
pub trait Interactable {
    /// The type of interaction result
    type Result;
    /// Perform the interaction and return the result
    fn interact(&mut self) -> Self::Result;
}
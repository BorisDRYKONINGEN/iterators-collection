//! A module containing the `ChildIterator` trait

/// A trait that means `Self` is an iterator on another iterator we could access to in some ways
pub trait ChildIterator {
    type Parent: Iterator;

    /// Destroys `self` and returns its parent
    fn release_parent(self) -> Self::Parent;

    /// Returns a mutable reference to the parent of `self`
    fn get_parent_mut(&mut self) -> &mut Self::Parent;

    /// Returns a reference to the parent of `self`
    fn get_parent(&self) -> &Self::Parent;
}

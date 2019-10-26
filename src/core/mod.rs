//! Some useful features shared by more than one other module or impossible to class in one of these

/// A resettable iterator. It means that calling a `reset` method will set the iterator to the first position
pub trait ResettableIterator: Iterator {
    /// Resets the iterator to its initial state when called
    fn reset(&mut self);

    /// Creates a new iterator from the current one and reset it
    fn reset_clone(&self) -> Self
    where
        Self: Clone,
    {
        let mut new = self.clone();
        new.reset();
        new
    }
}

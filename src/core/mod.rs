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

    /// Creates a ResettableMap from the current iterator
    fn resettable_map<F, R>(self, callback: F) -> self::ResettableMap<Self, F>
    where
        F: FnMut(Self::Item) -> R,
        Self: Sized,
    {
        ResettableMap {
            iterator: self,
            callback,
        }
    }
}

/// A resettable version of `std::iter::Map`. A simple trait implementation is not suitable because it requires to get access to private elements of `std::iter::Map` like the iterator stored
/// 
/// You can use it like you would use `std::iter::Map` but it implements the `ResettableIterator` trait too
pub struct ResettableMap<I, F> {
    iterator: I,
    callback: F,
}

impl<I, F, R> Iterator for ResettableMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<R> {
        let item = self.iterator.next()?;
        let result = (self.callback)(item);

        Some(result)
    }
}

impl<I, F, R> self::ResettableIterator for ResettableMap<I, F>
where
    I: self::ResettableIterator,
    F: FnMut(I::Item) -> R,
{
    fn reset(&mut self) {
        self.iterator.reset();
    }
}

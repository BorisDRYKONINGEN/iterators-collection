//! A module about advanced memory sharing during iteration

/// Iterates twice over the same collection
/// 
/// # Example
/// The following code
/// 
/// ```
/// use iterators_collection::share::DoubleIterator;
/// 
/// let mut array = [1, 2, 3, 4, 5];
/// let iter = DoubleIterator::new(&mut array);
/// 
/// for (i, j) in iter {
///     // Some code here
/// }
/// ```
/// 
/// Means the same as
/// 
/// ```
/// let array = [1, 2, 3, 4, 5];
/// for i in array.iter() {
///     for j in array.iter() {
///         // Some code here
///     }
/// }
/// ```
/// 
/// with some differences:
/// - i and j will never be the same with `DoubleIterator`
/// 
/// - you can safely iterate on a mutable slice with `DoubleIterator`
/// 
/// - i and j CANNOT be shared across threads because it is unsafe to increment the iterator in one thread while accessing one of these references from the other one. It may lead to a data race
/// 
/// - i and j are raw pointers and not references because the correct lifetime for the borrowed values is not known at compile time since a simple call to the `next` method may lead to a data race because two mutable references to the same object may exist
pub struct DoubleIterator<'a, T> {
    slice: &'a mut [T],
    first: usize,
    second: usize,
}

impl<'a, T> DoubleIterator<'a, T> {
    /// Creates a `DoubleIterator` from a slice
    pub fn new(slice: &'a mut [T]) -> Self {
        Self {
            slice,

            // It is safe to put two zeros because when next will be called, it will be first incremented and then, first = 0 and second = 1 which is the expected behaviour
            first: 0,
            second: 0,
        }
    }

    /// Returns a mutable pointer to the `index`th element of the borrowed slice
    /// 
    /// # Unsafety
    /// Indexes are not checked if the `debug_assert!`s are disabled
    /// 
    /// This pointer is unsafe to use
    unsafe fn nth_ptr(&mut self, index: usize) -> *mut T {
        debug_assert!(index < self.slice.len());
        self.slice.get_unchecked_mut(index) as *mut T
    }

    /// Increments the indexes `first` and `second` or returns Err
    fn increment(&mut self) -> Result<(), ()> {
        loop {
            // Increment
            self.second += 1;

            // Check for overflow
            if self.second == self.slice.len() {
                self.second = 0;
                self.first += 1;

                if self.first == self.slice.len() {
                    // Restore initial state to prevent the iterator from looping once again
                    self.first = self.slice.len() - 1;
                    self.second = self.first;

                    return Err(());
                }
            }

            if self.first != self.second {
                return Ok(());
            }
        }
    }
}

impl<T> Iterator for DoubleIterator<'_, T> {
    type Item = (*mut T, *mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(()) = self.increment() {
            return None;
        }

        Some(unsafe { (self.nth_ptr(self.first), self.nth_ptr(self.second)) })
    }
}

#[cfg(test)]
mod tests;

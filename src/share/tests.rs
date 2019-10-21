//! The unit-tests module for the share module

use crate::share::*;

#[test]
fn double_iterator_never_get_the_same_value() { // What a long name!
    let mut array = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut array);

    for (i, j) in iter {
        assert_ne!(*i, *j);
    }
}

#[test]
fn double_iterator_get_all_the_possible_values() { // Another long name!
    let mut array = [0, 1, 2, 3, 4];
    let mut count = [0; 5];
    let mut iter = DoubleIterator::new(&mut array);

    for (i, _j) in iter {
        count[*i] += 1;
    }

    assert_eq!(count, [4; 5]);
}

#[test]
fn double_iterator_does_not_loop() {
    let mut array = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut array);

    // Consuming the iterator
    while iter.next().is_some() {}

    assert!(iter.next().is_none());
}

//! The unit-tests module for the share module

use crate::share::*;
use crate::ResettableIterator;

#[test]
fn double_iterator_never_get_the_same_value() { // What a long name!
    let mut array = [1, 2, 3, 4, 5];
    let iter = DoubleIterator::new(&mut array);

    for (i, j) in iter {
        unsafe {
            assert_ne!(*i, *j);
        }
    }
}

#[test]
fn double_iterator_get_all_the_possible_values() { // Another long name!
    let mut array = [0, 1, 2, 3, 4];
    let mut count = [0; 5];
    let iter = DoubleIterator::new(&mut array);

    for (i, _j) in iter {
        unsafe {
            count[*i] += 1;
        }
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

#[test]
fn double_iterator_safe_for_each() {
    let mut array = [1, 2, 3, 4, 5];
    let iter = DoubleIterator::new(&mut array);

    // Here, we just test that it compiles without error
    iter.safe_for_each(|i: &mut i32, j: &mut i32| {
        println!("Got i = {} and j = {}", i, j);
    });
}

#[test]
fn double_iterator_reset() {
    let mut array = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut array);

    while iter.next().is_some() {}

    iter.reset();
    iter.next().unwrap();
}

#[test]
fn double_iterator_set() {
    let mut array = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut array);

    while iter.next().is_some() {}

    iter.set(0, 1);
    iter.next().unwrap();
}

#[test]
#[should_panic]
fn double_iterator_set_panics_with_same_values() {
    let mut array = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut array);

    // We could write other values
    iter.set(3, 3);
}

#[test]
#[should_panic]
fn double_iterator_set_panics_for_overflow() {
    let mut array = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut array);

    iter.set(5, 4);
}

#[test]
fn single_line_iterator_iterates_well() {
    let mut slice = [1, 2, 3, 4, 5];

    for n in 0..slice.len() {
        let iter = SingleLineIterator::new(&mut slice, n);

        for (i, j) in iter {
            assert_eq!(n + 1, *i);
            assert_ne!(i, j);
        }
    }
}

#[test]
#[should_panic]
fn single_line_iterator_new_panics_when_out_of_range() {
    let mut slice = [1, 2, 3, 4, 5];
    let _iter = SingleLineIterator::new(&mut slice, 5);
}

#[test]
fn single_line_iterator_from_double_iterator_works() {
    let mut slice = [1, 2, 3, 4, 5];
    let mut iter = DoubleIterator::new(&mut slice);
    iter.set(0, 3);

    let iter = SingleLineIterator::from(iter);
    let expected = vec![(1, 4), (1, 5)];

    let mut last_n = 0;
    for (n, i) in iter.enumerate() {
        last_n = n;

        println!("Received i = {:?}", i);
        assert_eq!(*i.0, expected[n].0);
        assert_eq!(*i.1, expected[n].1);
    }

    assert_eq!(last_n, expected.len() - 1);
}


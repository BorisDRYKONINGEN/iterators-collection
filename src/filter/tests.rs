//! Unit tests for the filter module

use crate::filter;

#[test]
fn exclude() {
    let array = [1, 2, 3, 4, 5];
    let array_iter = array.iter().cloned();
    let iter = filter::Exclude::with_blacklist(array_iter, vec![3, 5]);

    assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 4]);
}

#[test]
fn exclude_construction() {
    let array = [1, 2, 3, 4, 5];
    let array_iter = array.iter().cloned();
    let iter1 = filter::Exclude::with_blacklist(array_iter.clone(), vec![3, 5]);

    let mut iter2 = filter::Exclude::new(array_iter);
    iter2.exclude(3);
    iter2.exclude(5);
    iter2.exclude(3); // 3 shouldn't be added now because it has already been added

    assert_eq!(iter1.excluded, iter2.excluded);

    iter2.force_exclude(3); // should be added now because of the call to `force_exclude`
    assert_eq!(iter2.excluded, vec![3, 5, 3]);
}

use crate::core::*;

struct Count {
    pub begin: u32,
    pub cur: u32,
    pub end: u32,
}

impl Iterator for Count {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.cur == self.end {
            None
        } else {
            self.cur += 1;
            Some(self.cur - 1)
        }
    }
}

impl ResettableIterator for Count {
    fn reset(&mut self) {
        self.cur = self.begin;
    }
}

#[test]
fn count_iterates_well() {
    let expected = [0, 1, 2, 3, 4, 5];
    let iter = Count { begin: 0, cur: 0, end: 6 };

    for (n, i) in iter.enumerate() {
        assert_eq!(expected[n], i);
    }
}

#[test]
fn count_resets_well() {
    let mut iter = Count { begin: 0, cur: 0, end: 6 };
    iter.next();
    iter.next();

    iter.reset();
    assert_eq!(iter.next(), Some(0));
}

#[test]
fn resettable_map_iterates_well() {
    let mut counter = 0;
    let iter = Count { begin: 0, cur: 0, end: 6 };

    for i in iter.resettable_map(|x| x + 1) {
        counter += 1;
        assert_eq!(counter, i);
    }
}

#[test]
fn resettable_map_resets_well() {
    let iter = Count { begin: 0, cur: 0, end: 6 };
    let mut iter = iter.resettable_map(|x| x + 1);

    iter.next();
    iter.next();
    iter.next();

    iter.reset();

    assert_eq!(iter.next(), Some(1));
}

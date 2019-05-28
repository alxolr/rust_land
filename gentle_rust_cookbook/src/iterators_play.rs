use std::collections::HashSet;
use std::iter::FromIterator;

pub fn play() {
    let mut iter = 0..3;

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

pub fn set() {
    let mut v1 = vec![1, 10, 5, 1, 3, 11, 40, 1];
    v1.sort();
    println!("{:?}", v1);
    let set: HashSet<u32> = HashSet::from_iter(v1);

    println!("Deduped set {:?}", set.contains(&10));
}

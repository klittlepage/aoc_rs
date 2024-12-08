use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

use rug::{ops::AddFrom, Integer};

pub fn count_distinct<T: Clone + Hash + PartialEq + Eq>(
    values: impl Iterator<Item = T>,
) -> HashMap<T, Integer> {
    let mut counts: HashMap<T, Integer> = HashMap::new();

    for entry in values {
        match counts.entry(entry.clone()) {
            Entry::Occupied(mut x) => {
                x.get_mut().add_from(1);
            }
            Entry::Vacant(x) => {
                x.insert(Integer::from(1));
            }
        }
    }

    counts
}

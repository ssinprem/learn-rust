use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Eq + Hash> {
    set: HashSet<T>,
}

impl<T: Clone + Eq + Hash + Copy> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut hs = HashSet::new();
        for i in input {
            hs.insert(*i);
        }
        Self { set: hs }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.set.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.iter().all(|s| other.set.contains(s))
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.set.iter().any(|s| other.set.contains(s))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            set: self
                .set
                .iter()
                .filter(|s| other.contains(s))
                .copied()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            set: self
                .set
                .iter()
                .filter(|s| !other.contains(s))
                .copied()
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut set = HashSet::new();
        self.set.iter().for_each(|s| {
            set.insert(*s);
        });
        other.set.iter().for_each(|s| {
            set.insert(*s);
        });

        Self { set }
    }
}

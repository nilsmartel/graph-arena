use std::{cmp::Eq, collections::HashMap, hash::Hash};

pub struct GraphArena<T>
where
    T: Sized + Hash + Eq,
{
    data: Vec<T>,
    mapping: HashMap<T, usize>,
}

impl<T> GraphArena<T>
where
    T: Sized + Hash + Eq,
{
    pub fn new() -> Self {
        GraphArena {
            data: Vec::new(),
            mapping: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) -> usize {
        if let Some(key) = self.mapping.get(&item) {
            return *key;
        }

        let index = self.data.len();
        self.data.push(item);
        self.mapping.insert(item, index);
        index
    }
}

use std::{
    cmp::Eq,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub struct GraphArena<T>
where
    T: Sized + Hash + Eq,
{
    data: Vec<T>,
    mapping: HashMap<u64, usize>,
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

    pub fn get(&self, key: usize) -> &T {
        &self.data[key]
    }

    pub fn insert(&mut self, item: T) -> usize {
        let hashsum = hash(&item);

        if let Some(key) = self.mapping.get(&hashsum) {
            // perfrom EQ test im data[key] and use new hasher, if it fails
            // if self.data[*key] != item {
            //     self.rehash();
            //     return self.insert(item);
            // }

            return *key;
        }

        let index = self.data.len();
        self.data.push(item);
        self.mapping.insert(hashsum, index);
        index
    }
}

fn hash<T>(item: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        let obj = (123, 456);
        let h1 = hash(&obj);
        let h2 = hash(&obj);
        assert_eq!(h1, h2);
    }
}

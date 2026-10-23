#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    tick: u64,
    map: HashMap<K, (V, u64)>,
    order: BTreeMap<u64, K>,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        LRUCache {
            capacity,
            tick: 0,
            map: HashMap::new(),
            order: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((val, old_tick)) = self.map.get_mut(key) {
            self.order.remove(old_tick);
            self.tick += 1;
            *old_tick = self.tick;
            self.order.insert(self.tick, key.clone());
            return Some(&*val);
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.tick += 1;
        let old = self.map.insert(key.clone(), (value, self.tick));
        self.order.insert(self.tick, key);
        if let Some((old_val, old_tick)) = old {
            self.order.remove(&old_tick);
            return Some(old_val);
        }
        if self.map.len() > self.capacity {
            let (_, oldest) = self.order.pop_first().unwrap();
            self.map.remove(&oldest);
        }
        None
    }
}

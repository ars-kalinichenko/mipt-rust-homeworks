#![forbid(unsafe_code)]
use crate::node::Node;

pub struct AVLTreeMap<K, V> {
    // TODO: your code goes here.
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
        unimplemented!()
    }

    // fn get(&self, key: ...) -> Option<&V>
    // fn get_key_value(&self, key: ...) -> Option<&V>
    // fn contains_key(&self, key: ...) -> bool
    // fn insert(&mut self, key: K, value: V) -> Option<V>
    // fn nth_key_value(&self, k: usize) -> Option<(&K, &V)>
    // fn remove(&mut self, key: ...) -> Option<V>
    // fn remove_entry(&mut self, key: ...) -> Option<(K, V)>
}

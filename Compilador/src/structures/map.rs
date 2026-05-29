use std::collections::HashMap;
use crate::common::printable::Printable;

pub struct MyMap<K, V> {
    items: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> MyMap<K, V> {
    pub fn new() -> Self {
        MyMap {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.items.insert(key, value);
    }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> Printable for MyMap<K, V> {
    fn print(&self) {
        println!("{:?}", self.items);
    }
}
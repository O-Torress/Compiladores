use std::collections::VecDeque;
use crate::common::printable::Printable;

pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
}

impl<T: std::fmt::Debug> Printable for Queue<T> {
    fn print(&self) {
        println!("{:?}", self.items);
    }
}
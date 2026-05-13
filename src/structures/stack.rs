use crate::common::printable::Printable;

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

impl<T: std::fmt::Debug> Printable for Stack<T> {
    fn print(&self) {
        println!("{:?}", self.items);
    }
}
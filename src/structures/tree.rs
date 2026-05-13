use crate::common::printable::Printable;

#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    pub fn insert(self, value: T) -> Self {
        match self {
            BinaryTree::Empty => {
                BinaryTree::Node {
                    value,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }
            }

            BinaryTree::Node {
                value: current,
                left,
                right,
            } => {
                if value < current {
                    BinaryTree::Node {
                        value: current,
                        left: Box::new(left.insert(value)),
                        right,
                    }
                } else {
                    BinaryTree::Node {
                        value: current,
                        left,
                        right: Box::new(right.insert(value)),
                    }
                }
            }
        }
    }
}

impl<T: std::fmt::Debug> Printable for BinaryTree<T> {
    fn print(&self) {
        println!("{:#?}", self);
    }
}
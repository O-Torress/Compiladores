use crate::common::printable::Printable;

#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    Node {
        value: T,
        children: Vec<Box<Tree<T>>>,
    },
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn with_value(value: T) -> Self {
        Tree::Node {
            value,
            children: Vec::new(),
        }
    }

    /// Inserta `value` en el árbol de forma no balanceada.
    /// Si el árbol está vacío, crea un nodo raíz con el valor.
    /// Si ya existe un nodo raíz, añade el nuevo valor como hijo del nodo actual
    /// (sin intentar redistribuir o equilibrar).
    pub fn insert(self, value: T) -> Self {
        match self {
            Tree::Empty => Tree::Node {
                value,
                children: Vec::new(),
            },

            Tree::Node { value: current, mut children } => {
                children.push(Box::new(Tree::Node { value, children: Vec::new() }));
                Tree::Node { value: current, children }
            }
        }
    }

    /// Añade un hijo al nodo actual sin reconstruir el árbol.
    pub fn add_child(&mut self, child: Tree<T>) {
        if let Tree::Node { children, .. } = self {
            children.push(Box::new(child));
        }
    }
}

impl<T: std::fmt::Debug> Printable for Tree<T> {
    fn print(&self) {
        println!("{:#?}", self);
    }
}
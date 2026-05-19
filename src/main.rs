use support_library::structures::stack::Stack;
use support_library::structures::queue::Queue;
use support_library::structures::tree::Tree;
use support_library::structures::map::MyMap;

use support_library::helpers::logger::{log, LogLevel};

use support_library::common::printable::Printable;

fn main() {

    log(LogLevel::Info, "Iniciando programa");

    // STACK
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);

    stack.print();
    
    // QUEUE
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);

    queue.print();

    // TREE
    let tree = Tree::new()
        .insert(10)
        .insert(5)
        .insert(15);

    tree.print();

    // MAP
    let mut map = MyMap::new();

    map.insert("nombre", "Santiago");

    map.print();

}
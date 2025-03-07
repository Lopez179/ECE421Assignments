use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::Display;
// The node type stores the data and two pointers. It uses Option to represent nullability in safe Rust.
// It uses an Rc (Reference Counted) pointer to give ownership of the next node
// to the current node. And a Weak (weak Reference Counted) pointer to reference.
// the previous node without owning it.

// It uses RefCell for interior mutability. It allows mutation through shared references.
struct Node<T> {
    data: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
// Constructs a node with some `data` initializing prev and next to null.
    fn new(data: T) -> Self {
        Self { data, prev: None, next: None }
    }
    // Appends `data` to the chain of nodes. The implementation is recursive.
    fn append(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        let is_last = node.borrow().next.is_none();
        if is_last {
            // If the current node is the last one, create a new node,
            // set its prev pointer to the current node and store it as the node after the current one.
            let mut new_node = Node::new(data);
            new_node.prev = Some(Rc::downgrade(&node));
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().next = Some(rc.clone());
            Some(rc)
        }
        else {
            // Not the last node, just continue traversing the list:
            if let Some(ref mut next) = node.borrow_mut().next {
            Self::append(next, data)
            } else { None }
        }
    }
}
    // The doubly-linked list with pointers to the first and last nodes in the list.
struct List<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> List<T> {
// Constructs an empty list.
    fn new() -> Self {
        Self { first: None, last: None }
    }
    // Appends a new node to the list, handling the case where the list is empty.
    fn append(&mut self, data: T) {
        match self.last.take() {
            Some(l_n) => {
                // Let node::append handle this
                let new_last = Node::append(&mut l_n.clone(), data);
                // Change the list.last reference
                self.last = new_last;
            },
            None => {
                // create new node
                let new = Node::new(data);
                let new_ref = Rc::new(RefCell::new(new));

                // change the list reference fields
                self.first = Some(new_ref.clone());
                self.last = Some(new_ref.clone());
            },
        }
    }
}

// Pretty-printing
impl<T: Display> Display for List<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.first.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}
fn main() {
    let mut list = List::new();
    println!("{}", list);
    for i in 0..5 {
        list.append(i);
    }
    println!("{}", list);
}
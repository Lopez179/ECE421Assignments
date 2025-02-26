pub enum LinkedList<T>{
    Tail,
    Head(T,Box<LinkedList<T>>),
}

impl<T> LinkedList<T> {
    // Part a
    pub fn empty() -> Self {
        Self::Tail
    }


    // Part b
    pub fn new(t:T)->Self {
        Self::Head(t, Box::new(Self::Tail))
    }

    // Part c
    pub fn push(self, t:T)->Self{
        let new_head = Self::Head(t, Box::new(self));
        new_head
    }

    // Part d
    pub fn push_back(&mut self, t:T) {
        let mut current_node = self;
        while let Self::Head(_, next_node) = current_node {
            current_node = next_node;
        }
        *current_node = LinkedList::new(t);
    }
}

fn main() {
    let mut node_a: LinkedList<i32> = LinkedList::new(2);
    node_a = node_a.push(1);
    node_a.push_back(3);
    if let LinkedList::Head(value_a, node_b) = node_a {
        println!("{}", value_a);

        if let LinkedList::Head(value_b, node_c) = *node_b {
            println!("{}", value_b);
            if let LinkedList::Head(value_c, _) = *node_c {
                println!("{}", value_c);
            }
        }
    }
}
use im::list::*;
#[derive(Debug, PartialEq)]
pub enum LinkedList<T>{
    Tail,
    Head(List<T>),
}

impl<T> LinkedList<T> {
    // Part a
    pub fn empty() -> Self {
        Self::Tail
    }

    // Part b
    pub fn new(t:T)->Self {
        let typedlist: List<T> = List::new();
        Self::Head(typedlist)
    }

    // Part c
    pub fn push(self, t:T)->Self{
        match self {
            LinkedList::Tail => {
                Self::new(t)
            },
            LinkedList::Head(list) => {
                LinkedList::Head(cons(t, list))
            },
        }
        
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l,Head(cons(4, cons(3, List::new()))));
        l.push_back(2);
        assert_eq!(l,Head(cons(4, cons(3, cons(2,List::new())))));
    }
}

fn main() {
    
}
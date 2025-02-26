use im::list::*;
use std::vec;
#[derive(Debug, PartialEq)]
pub enum LinkedList<T>{
    Tail,
    Head(List<T>),
}

impl<T: Clone> LinkedList<T> {
    // Part a
    pub fn empty() -> Self {
        Self::Tail
    }

    // Part b
    pub fn new(t:T)->Self {
        let mut typedlist: List<T> = List::new();
        typedlist = cons(t, typedlist);
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
        let mut new: LinkedList<T> = LinkedList::Tail;
        let mut v: Vec<T> = Vec::new();
        match self {
            LinkedList::Tail => {
            },
            LinkedList::Head(list) => {
                for i in list.iter() {
                    let value = i.as_ref().clone();
                    v.push(value);
                }
            },
        }
        // LinkedList
        new = new.push(t);
        for i in v.into_iter().rev() {
            new = new.push(i);
        }
        *self = new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l,LinkedList::Head(cons(4, cons(3, List::new()))));
        l.push_back(2);
        assert_eq!(l,LinkedList::Head(cons(4, cons(3, cons(2,List::new())))));
    }
}

//cons(value_a, moved_list)
//Returns a list where all positions have the same values of moved_list, except there's a new position that holds value_a.

fn main() {
    
}
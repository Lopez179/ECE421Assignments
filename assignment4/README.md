# Assignment 4 ECE 421
## Question 1
Run the test with:

```cargo run --package Q1```

Code is inside Q1/src/main.rs
## Question 2
### Part A
cons(value_a, moved_list)

Returns a list where all positions have the same values of moved_list, except there's a new position that holds value_a.

### Part B
```cargo run --package Q2```

## Question 3
```cargo run --package Q3```

## Question 4
### Part A
The code is creating DoubleNode instances and then wrapping them in reference counted, mutable reference smart pointers. Rc makes the pointer reference counted, while RefCell adds interior mutability. The code then prints the number of strong references, which is the amount of references that need to be dropped before the actual value is dropped.

### Part B
Double node implements a node of a doubly linked list. It acts like a linked list but with supports traversal in both directions

### Part C
Weak does not take ownership the way Rc does. This means that creating a weak reference does not effect the strong reference count, and does not need to be dropped in order for the value to be dropped. It's usage is for preventing reference cycles that would otherwise be necessitated by two structs containing references to eachother. For example, in the provided code each of the doubly linked list nodes would need to contain Weak references to each other. Reference cycles cause two references to depend on eachother being dropped in order for their values to be dropped, which would essentially mean that it would be impossible to drop their values.


### Part D
```if let Some(ref mut x) = *a.borrow_mut()```

Is necessary to access the value of an option, and get a mutable reference to a.

```(*x).prev = Rc::clone(&b);```

This assigns a weak pointer to b to the prev field of a.

## Question 5



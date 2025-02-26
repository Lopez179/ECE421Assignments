use std::cell::Cell;
enum Level {
    Low,
    Medium,
    High
}

struct Task {
    id: Cell<u8>,
    level: Level
}

fn main() {
    let task = Task {
        id: Cell::new(10),
        level: Level::High
    };

    task.id.set(100);
    println!("Task with ID: {}", task.id.get());
}

// The code was fixed by wrapping the id field of the Task in a cell. This provided interior mutability on the id field, while
// still making the other field (level) immutable. This allows multiple scopes to mutate a shared state, that otherwise would
// not be mutable.
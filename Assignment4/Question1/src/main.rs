use std::mem;
struct Bag<T> {
    items: [T; 3],
}

fn main() {
    let b1 = Bag {items: [1u8, 2u8, 3u8], };
    let b2 = Bag {items: [1u32, 2u32, 3u32], };

    println!("Size of First Bag = {} bytes", BagSize(b1));
    println!("Size of Second Bag = {} bytes", BagSize(b2));
}

fn BagSize<T: std::fmt::Debug>(b: Bag<T>) -> usize {
    let mut count: usize = 0;
    for i in b.items.iter() {
        count+=mem::size_of_val(i);
    }
    count
} 
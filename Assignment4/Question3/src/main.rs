use std::mem;
struct Bagu8 {
    items: [u8; 3],
}

struct Bagu32 {
    items: [u32; 3],
}

fn main() {
    let b1 = Bagu8 {items: [1u8, 2u8, 3u8], };
    let b2 = Bagu32 {items: [1u32, 2u32, 3u32], };
}

fn BagSizeu8(b: Bagu8) -> usize {
    let mut count: usize = 0;
    for i in b.items.iter() {
        count+=mem::size_of_val(i);
    }
    count
} 

fn BagSizeu32(b: Bagu32) -> usize {
    let mut count: usize = 0;
    for i in b.items.iter() {
        count+=mem::size_of_val(i);
    }
    count
} 
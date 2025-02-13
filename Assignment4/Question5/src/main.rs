use std::mem;
fn main() {
    let vec1 = vec![12, 32, 13];
    let vec2 = vec![44, 55, 16];
    { 
        let vec1_iter = vec1.iter();
        let boxed = Box::new(vec1_iter);
        println!("vec1_iter: {} bytes", size(boxed));
    }

    {
        let vec_chained = vec1.iter().chain(vec2.iter());
        let boxed = Box::new(vec_chained);
        println!("vec_chained: {} bytes", size(boxed));
    }

    {
        let vec1_2=vec![vec1, vec2];
        let vec_flattened = vec1_2.iter().flatten();
        let boxed = Box::new(vec_flattened);
        println!("vec_flattened: {} bytes", size(boxed));
    }

}

fn size<T: std::fmt::Debug>(b: T) -> usize {
    mem::size_of_val(&b)
}
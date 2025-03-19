use rayon::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use crossbeam::scope;

fn concurrent_quick_sort<T: PartialOrd + std::fmt::Debug + Send>(v: &mut [T]) {
    let pivot = partition(v);
    
    let (left_slice, right_slice) = v.split_at_mut(pivot);
    let right_slice = &mut right_slice[1..];

    

    scope(|s| {
        s.spawn(move |_| {
            if left_slice.len() > 1 {
                concurrent_quick_sort(left_slice);
            }
        });
    
        s.spawn(move |_| {
            if right_slice.len() > 1 {
                concurrent_quick_sort(right_slice);
            }
        });
    })
    .unwrap();
}

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut left_pointer: i32 = -1;
    let mut right_pointer: usize = 0;

    while right_pointer < (v.len() - 1) {
        if v[right_pointer] < v[pivot] {
            left_pointer+=1;

            v.swap(left_pointer as usize, right_pointer);
        }
        right_pointer+=1;
    }
    left_pointer+=1;
    v.swap((left_pointer) as usize, right_pointer);

    left_pointer as usize
}

fn main() {
    let mut test = Vec::from_iter([6, 2, 7, 11, 0, 1, 8]);
    concurrent_quick_sort(&mut test);
    println!("{:?}", test);
}

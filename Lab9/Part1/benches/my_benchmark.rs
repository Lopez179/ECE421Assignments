use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;



pub trait VecExt<T: PartialOrd> {
    fn selection_sort(&mut self);
}

impl<T: PartialOrd> VecExt<T> for Vec<T> {
    fn selection_sort(&mut self) {
        let n = self.len();

        for i in 0..n - 1 {
            let mut current_min_index = i;

            for j in (i + 1)..n {
                if self[j] < self[current_min_index] {
                    current_min_index = j;
                }
            }

            self.swap(i, current_min_index);
        }
    }
}

fn my_function(v: Vec<i64>) {
    let mut testvec = v;
    testvec.selection_sort();
    //println!("{:?}",testvec);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| {rng.gen_range(1..10000)}).collect();
    c.bench_function("your function: ", |b| {
        b.iter(|| my_function(black_box(l.clone()))) // Clone to avoid modifying the original vector
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
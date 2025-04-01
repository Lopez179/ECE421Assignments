use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;



pub trait VecExt<T: PartialOrd> {
    fn selection_sort(&mut self);
}

impl<T: PartialOrd + Ord> VecExt<T> for Vec<T> {
    fn selection_sort(&mut self) {
        let n = self.len();

        (0..n - 1).for_each(|bound| {
            if let Some(current_min_pos) = (bound + 1..n).min_by_key(|&j| &self[j])
            {
                if self[current_min_pos] < self[bound] {
                    self.swap(bound, current_min_pos);
                }
            }
        });
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
use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

struct Person {
    age: u32,
}

fn test_run_parallel(size: u32) {
    let mut v: Vec<Person> = Vec::new();
    for i in 1..size {
        v.push(Person { age: i });
    }

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let _avg_over_30 = sum_over_30 as f32/ num_over_30;
    //println!("The average age of people older than 30 is {}", avg_over_30);
}

fn test_run_serial(size: u32) {
    let mut v: Vec<Person> = Vec::new();
    for i in 1..size {
        v.push(Person { age: i });
    }

    let num_over_30 = v.iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let _avg_over_30 = sum_over_30 as f32/ num_over_30;
    //println!("The average age of people older than 30 is {}", avg_over_30);
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ser iter 100000",|b| b.iter(|| test_run_serial(100000)));
    c.bench_function("par iter 100000",|b| b.iter(|| test_run_parallel(100000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
#![feature(portable_simd)]
mod simd_vs_if;

use simd_vs_if::*;
use rand::random;
use criterion::{criterion_group, criterion_main, Criterion};

fn simd_test(c: &mut Criterion) {
    let array = [0,0,1,0,1,2,3];

     c.bench_function("simd",
        |b| b.iter(|| {
            let element: i32 = random();
            search_3_level_tree_for_lower_bound_simd(element%4, &array)
     }));
}

fn if_test(c: &mut Criterion) {
    let array = [0,0,1,0,1,2,3];

     c.bench_function("if",
        |b| b.iter(|| {
            let element: i32 = random();
            search_3_level_tree_for_lower_bound(element%4, &array)
     }));
}

criterion_group!(benches,simd_test,if_test);


criterion_main!(benches);
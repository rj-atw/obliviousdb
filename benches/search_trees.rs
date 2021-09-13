use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;

use obliviousdb::search_tree::SearchTree;

fn benchmark_search_oblivious_static_search_tree(c: &mut Criterion) {
    let max: u32 = 268_435_456_0;/*536_870_912_0;*//*1_073_741_824_0;*/
    let min = 0;
    let gen = (min..max).step_by(10).into_iter();

    let tree = SearchTree::new(gen, ((max - min) / 10) as usize).unwrap();

    c.bench_function("cache-oblivious search", 
    |b| b.iter(|| {
        let element = random();//i.next().unwrap();
        tree.search(element)
    }));
}

fn benchmark_search_std_collection_btreemap(c: &mut Criterion) {
    use std::collections::BTreeMap;
    let max: u32 = 268_435_456_0;/*536_870_912_0;*//*1_073_741_824_0;*/
    let min = 0;
    let gen = (min..max).step_by(10).into_iter();

    let mut tree = BTreeMap::<u32, bool>::new();
    gen.for_each(|i| { tree.insert(i, true); });

    c.bench_function("btreemap search",
    |b| b.iter(|| {
        let element: u32 = random();//i.next().unwrap();
        tree.contains_key(&element)
    }));
}

criterion_group!(benches, benchmark_search_oblivious_static_search_tree, benchmark_search_std_collection_btreemap);
criterion_main!(benches);


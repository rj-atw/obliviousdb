use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;

mod perf;

use obliviousdb::search_tree::SearchTree;

fn benchmark_search_oblivious_static_search_tree(c: &mut Criterion) {
    let max: i32 = 268_435_456;
        //536_870_912;
    //1_073_741_824;
    let min = 0;
    let gen = (min..max).step_by(1).into_iter();

    let tree = SearchTree::new(gen, (max - min) as usize).unwrap();

    c.bench_function("cache-oblivious search", 
    |b| b.iter(|| {
        let element = random();//i.next().unwrap();
        tree.search(element)
    }));
}

fn benchmark_search_std_collection_btreemap(c: &mut Criterion) {
    use std::collections::BTreeMap;
    let max: i32 =  268_435_456;
    //536_870_912;
    //1_073_741_824;
    let min = 0;
    let gen = (min..max).step_by(1).into_iter();

    let mut tree = BTreeMap::<i32, bool>::new();
    gen.for_each(|i| { tree.insert(i, true); });

    c.bench_function("btreemap search",
    |b| b.iter(|| {
        let element: i32 = random();//i.next().unwrap();
        tree.contains_key(&element)
    }));
}
/*


*/




criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = benchmark_search_oblivious_static_search_tree, benchmark_search_std_collection_btreemap
}

//criterion_group!(benches,benchmark_search_std_collection_btreemap,benchmark_search_oblivious_static_search_tree);

criterion_main!(benches);

#![feature(portable_simd)]
mod search_tree;

use search_tree::SearchTree;

fn main() {
    let array: Vec<u32> = (0..8).collect();
    if let Ok(search_tree) = SearchTree::new(array.into_iter(), 8) {
        search_tree.search(6);
    }
}

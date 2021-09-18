
mod search;
mod create;
mod util;


use search::SearchTreeIndex;
use search::search_for_lower_bound;
use crate::search_tree::create::layout;
use crate::search_tree::search::SearchTreeIndex::{NotInTree};
use crate::search_tree::search::Leaf;

pub struct SearchTree {
    array: Box<[i32]>,
    height: u16
}

impl SearchTree {
    pub fn search(&self, element: i32) -> SearchTreeIndex {
        let Leaf { index, leaf_number } = search_for_lower_bound(element, self.height, &self.array);

        return if index == 0 && leaf_number > 0 {
            NotInTree
        } else {
            SearchTreeIndex::Leaf { index, leaf_number}
        }
    }

    pub fn new<'a>(generator: impl Iterator<Item=i32>, count: usize) -> Result<SearchTree, ()>{
        assert_eq!(count.count_ones(), 1,
                   "Search Tree must be a full binary tree. Number of leaves: {}", count);

        let height = count.trailing_zeros()  as u16 + 1;
        let size = count*2-1;

        let mut reserved_space = vec![0; size];
        layout(&mut reserved_space, Box::new(generator), height).map(
            move |_i| SearchTree { array: reserved_space.into_boxed_slice(), height}
        )
    }
}



#[cfg(test)]
mod tests {
    use crate::search_tree::search::{SearchTreeIndex};
    use crate::search_tree::SearchTree;

    #[test]
    fn create_and_search() {
        let leaves : Vec<i32> = (0..32).chain((40..=360).step_by(10)).collect();
        let search_tree = SearchTree::new(leaves.into_iter(), 64).unwrap();

        assert_eq!(search_tree.search(39),
            SearchTreeIndex::Leaf{index: 70, leaf_number: 31},
            "Searching for element in tree's span, but not in tree. Expecting lower bound"
        );
    }
}

mod search;
mod create;
mod util;

use search::SearchTreeIndex;
use search::search_for_lower_bound;

pub struct SearchTree<'a> {
    array: &'a[i32],
    height: u16
}

impl SearchTree<'_> {
  pub fn new(array: &[i32], height: u16) -> SearchTree {
      SearchTree { array, height }
  }

  pub fn search(&self, element: i32) -> SearchTreeIndex {
      search_for_lower_bound(element, self.height, self.array)
  }
}
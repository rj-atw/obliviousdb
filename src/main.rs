mod search_tree;

use search_tree::SearchTree;


/*
             0
         0       4
       0   2   4   6
      0 1 2 3 4 5 6 7
 */
fn main() {
    let array = [0,0,4,  0,0,1,  2,2,3,  4,4,5,  6,6,7];
    let search_tree = SearchTree::new(&array, 3);
    search_tree.search(6);
}
use crate::search_tree::util::{is_odd, size_of_tree_with_height, number_of_leaves_in_tree};

use core_simd::*;

#[derive(Eq, PartialEq, Debug)]
pub struct Leaf { pub index: i32, pub leaf_number: i32 }

#[derive(Eq, PartialEq, Debug)]
pub enum SearchTreeIndex {
    NotInTree,
    Leaf { index: i32, leaf_number: i32 }
}

fn search_3_level_tree_for_lower_bound_simd(of: i32, array: &[i32]) -> Leaf {
    const NULL: Simd<i32, 4> = i32x4::splat(6);
    const LEAF: Simd<i32, 4> = i32x4::from_array([0, 3, 4, 5]);

    let of_simd = i32x4::splat(of);
    let base_simd = i32x4::from_array([array[3], array[4], array[5], array[6]]);

    let s = of_simd.lanes_lt(base_simd);

    let selected = s.select(LEAF, NULL);
    let idx = selected.horizontal_min();

    Leaf { index: idx , leaf_number: idx - 3 }
}

fn search_2_level_tree_for_lower_bound(of: i32, array: &[i32]) -> Leaf {
   let lower_bound_is_2nd_leaf = (of >= array[2]) as i32;

   Leaf { index: 1 + lower_bound_is_2nd_leaf, leaf_number: 0 + lower_bound_is_2nd_leaf }
}

fn search_single_node_tree_for_lower_bound(_of: i32, _array: &[i32]) -> Leaf {
    Leaf {index: 0, leaf_number: 0}
}

pub fn search_for_lower_bound_in_top_subtree(element: i32, height: u16, array: &[i32]) -> i32 {
    let top_subtree_is_taller = is_odd(height);
    let subtree_height = height >> 1;
    let top_subtree_height = subtree_height + top_subtree_is_taller as u16;
    let top_subtree_size = size_of_tree_with_height(top_subtree_height);
    let bottom_subtree_size = size_of_tree_with_height(subtree_height);

    let  Leaf { index: _, leaf_number } = search_for_lower_bound(element, top_subtree_height, &array);

    let right_subtree_root_index = (top_subtree_size + bottom_subtree_size*(2*leaf_number+1)) as usize;
    let right_subtree_root = array[right_subtree_root_index];

    let is_right_subtree = element >= right_subtree_root;
    let subtree_number = 2*leaf_number + is_right_subtree as i32;

    subtree_number
}

pub fn subtree_root_index_generator(height: u16) -> impl Fn(i32) -> i32 {
    let top_subtree_is_taller = is_odd(height);
    let subtree_height = height >> 1;
    let bottom_subtree_size = size_of_tree_with_height(subtree_height);
    let top_subtree_height = subtree_height + if top_subtree_is_taller {1} else {0};
    let top_subtree_size = size_of_tree_with_height(top_subtree_height);

    move | subtree_number: i32 | top_subtree_size + bottom_subtree_size * subtree_number
}

//Lower bound must exist
pub fn search_for_lower_bound(element: i32, height: u16, array: &[i32]) -> Leaf {
    return match height {
        3 => { search_3_level_tree_for_lower_bound_simd(element, array) }
        2 => { search_2_level_tree_for_lower_bound(element, array) }
        1 => { search_single_node_tree_for_lower_bound(element, array) }
        _ => {
           let subtree_height = height >> 1;
           let subtree_root_index = subtree_root_index_generator(height);

           let subtree_number = search_for_lower_bound_in_top_subtree(element, height, array);

            let bottom_subtree_index = {
                let start_index = subtree_root_index(subtree_number) as usize;
                let end_index = subtree_root_index(subtree_number+1) as usize;
                search_for_lower_bound(element, subtree_height, &array[start_index..end_index])
            };

            let Leaf { index: index_in_subtree, leaf_number: leaf_number_in_subtree } = bottom_subtree_index;
            Leaf {
                index: subtree_root_index(subtree_number) + index_in_subtree,
                leaf_number: number_of_leaves_in_tree(subtree_height) * subtree_number + leaf_number_in_subtree
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::search_tree::search::{search_single_node_tree_for_lower_bound,
                                     search_for_lower_bound,
                                     search_3_level_tree_for_lower_bound_simd, Leaf,
                                     search_2_level_tree_for_lower_bound};

    #[test]
    fn search_in_base_case_height3() {
        let tree_of_height_3 = [1,1,4,  1,2,4,  6];

        let test_case = | of: i32, expected_index: i32, expected_leaf: i32, on_fail: &str |
            assert_eq!(search_3_level_tree_for_lower_bound_simd(of, &tree_of_height_3),
                       Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(5,5,2,
                  "Search for element in middle of Tree with height 3");

        test_case(6,6,3,
                   "Search for largest element of Tree with height 3");

        test_case(1,3,0,
                   "Search for smallest element of Tree with height 3");

        test_case(2,4,1,
                   "Search for element not in tree in middle of range");

        test_case(2000,6,3,
                   "Search for element not in tree which is greater than range");
    }

    #[test]
    fn search_in_base_case_height2() {
        let tree_of_height_2 = [10,10,16];

        let test_case = | of: i32, expected_index: i32, expected_leaf: i32, on_fail: &str |
            assert_eq!(search_2_level_tree_for_lower_bound(of, &tree_of_height_2),
                       Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(16,2,1,
                   "Search for largest element of Tree with height 2");

        test_case(10,1,0,
                   "Search for smallest element of Tree with height 2");

        test_case(14,1,0,
                   "Search for element not in tree within range");

        test_case(200,2,1,
                   "Search for element not in tree greater than range");
    }

    #[test]
    fn search_in_base_case_height1() {
        let tree_of_height_1 = [23];

        let test_case = | of: i32, expected_index: i32, expected_leaf: i32, on_fail: &str |
            assert_eq!(search_single_node_tree_for_lower_bound(of, &tree_of_height_1),
                       Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(23, 0,0, "search for element");
        test_case(30, 0,0, "search greater than element");
    }

    #[test]
    fn test_search_for_elements_in_tree() {
        let tree = [0,0,4,  0,0,1,  2,2,3,  4,4,5,  6,6,7];

        let test_case = | of: i32, expected_index: i32, expected_leaf: i32, on_fail: &str |
            assert_eq!(search_for_lower_bound(of, 4, &tree),
                       Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(2,7,2,"searching for element in middle of tree");
        test_case(0,4,0,"searching for the smallest element");
        test_case(7,14,7,"searching for the greatest element");
    }

    #[test]
    fn test_search_for_elements_not_in_tree() {
        let tree = [1,1,5,  1,1,2,  3,3,4,  5,5,57,  77,77,78];

        let test_case = | of: i32, expected_index: i32, expected_leaf: i32, on_fail: &str |
            assert_eq!(search_for_lower_bound(of, 4, &tree),
                       Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(58,11,5, "element in range but not in tree");
        test_case(800,14,7, "element greater than tree");
    }
}

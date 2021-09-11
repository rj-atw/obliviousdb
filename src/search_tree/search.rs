use crate::search_tree::util::{is_odd, size_of_tree_with_height, number_of_leaves_in_tree};

#[derive(Eq, PartialEq, Debug)]
pub enum SearchTreeIndex {
    NotInTree,
    Leaf { index: u32, leaf_number: u32 }
}


fn search_3_level_tree_for_lower_bound(of: i32, array: &[i32]) -> SearchTreeIndex {
    if of < array[3] {
        SearchTreeIndex::NotInTree
    } else if of < array[4] {
        SearchTreeIndex::Leaf { index: 3, leaf_number: 0 }
    } else if of < array[5] {
        SearchTreeIndex::Leaf { index: 4, leaf_number: 1 }
    } else if of < array[6] {
        SearchTreeIndex::Leaf { index: 5, leaf_number: 2 }
    } else {
        SearchTreeIndex::Leaf { index: 6, leaf_number: 3 }
    }
}

fn search_2_level_tree_for_lower_bound(of: i32, array: &[i32]) -> SearchTreeIndex {
    if of < array[1] {
        SearchTreeIndex::NotInTree
    } else if of < array[2] {
        SearchTreeIndex::Leaf { index: 1, leaf_number: 0 }
    } else {
        SearchTreeIndex::Leaf { index: 2, leaf_number: 1 }
    }
}

fn search_single_node_tree_for_lower_bound(of: i32, array: &[i32]) -> SearchTreeIndex {
    if of >= array[0] {
        SearchTreeIndex::Leaf {index: 0, leaf_number: 0}
    } else {
        SearchTreeIndex::NotInTree
    }
}

pub fn search_for_lower_bound(element: i32, height: u16, array: &[i32]) -> SearchTreeIndex {
    return match height {
        3 => { search_3_level_tree_for_lower_bound(element, array) }
        2 => { search_2_level_tree_for_lower_bound(element, array) }
        1 => { search_single_node_tree_for_lower_bound(element, array) }
        _ => {
            let top_subtree_is_taller = is_odd(height);
            let subtree_height = height >> 1;

            let top_subtree_height = subtree_height + if top_subtree_is_taller {1} else {0};
            let top_subtree_size = size_of_tree_with_height(top_subtree_height);
            let bottom_subtree_size = size_of_tree_with_height(subtree_height);

            let leaf_number =
                if let SearchTreeIndex::Leaf { index: _, leaf_number } =
                search_for_lower_bound(element, top_subtree_height, &array[0..top_subtree_size as usize]) {
                    leaf_number
                } else {
                    return SearchTreeIndex::NotInTree
                };

            let subtree_root_index =
                | subtree_number: u32 | top_subtree_size + bottom_subtree_size * subtree_number;

            let right_subtree_root = array[subtree_root_index(2*leaf_number+1) as usize];

            let subtree_number =
                if element >= right_subtree_root { 2*leaf_number + 1 } else { 2*leaf_number };

            let bottom_subtree_index = {
                let start_index = subtree_root_index(subtree_number) as usize;
                let end_index = subtree_root_index(subtree_number+1) as usize;
                search_for_lower_bound(element, subtree_height, &array[start_index..end_index])
            };

            match bottom_subtree_index {
                SearchTreeIndex::NotInTree => { SearchTreeIndex::NotInTree }
                SearchTreeIndex::Leaf { index: index_in_subtree, leaf_number: leaf_number_in_subtree } =>
                    SearchTreeIndex::Leaf {
                        index: subtree_root_index(subtree_number) + index_in_subtree,
                        leaf_number: number_of_leaves_in_tree(subtree_height) * subtree_number + leaf_number_in_subtree
                    }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::search_tree::search::{search_3_level_tree_for_lower_bound, SearchTreeIndex, search_2_level_tree_for_lower_bound, search_single_node_tree_for_lower_bound, search_for_lower_bound};

    #[test]
    fn search_in_base_case_height3() {
        let tree_of_height_3 = [0,0,1,  0,1,5,  6];

        let test_case = | of: i32, expected_index: u32, expected_leaf: u32, on_fail: &str |
            assert_eq!(search_3_level_tree_for_lower_bound(of, &tree_of_height_3),
                       SearchTreeIndex::Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(5,5,2,
                  "Search for element in middle of Tree with height 3");

        test_case(6,6,3,
                   "Search for largest element of Tree with height 3");

        test_case(0,3,0,
                   "Search for smallest element of Tree with height 3");

        test_case(2,4,1,
                   "Search for element not in tree in middle of range");

        test_case(2000,6,3,
                   "Search for element not in tree which is greater than range");

        assert_eq!(search_3_level_tree_for_lower_bound(-5, &tree_of_height_3),
                   SearchTreeIndex::NotInTree,
                   "Search for element not in tree which is less than range");
    }

    #[test]
    fn search_in_base_case_height2() {
        let tree_of_height_2 = [10,10,16];

        let test_case = | of: i32, expected_index: u32, expected_leaf: u32, on_fail: &str |
            assert_eq!(search_2_level_tree_for_lower_bound(of, &tree_of_height_2),
                       SearchTreeIndex::Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(16,2,1,
                   "Search for largest element of Tree with height 3");

        test_case(10,1,0,
                   "Search for smallest element of Tree with height 3");

        test_case(14,1,0,
                   "Search for element not in tree within range");

        test_case(200,2,1,
                   "Search for element not in tree greater than range");

        assert_eq!(search_2_level_tree_for_lower_bound(-5, &tree_of_height_2),
                   SearchTreeIndex::NotInTree,
                   "Search for element not in tree less than range");
    }

    #[test]
    fn search_in_base_case_height1() {
        let tree_of_height_1 = [23];

        let test_case = | of: i32, expected_index: u32, expected_leaf: u32, on_fail: &str |
            assert_eq!(search_single_node_tree_for_lower_bound(of, &tree_of_height_1),
                       SearchTreeIndex::Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(23, 0,0, "search for element");
        test_case(30, 0,0, "search greater than element");

        assert_eq!(search_single_node_tree_for_lower_bound(0, &tree_of_height_1),
                   SearchTreeIndex::NotInTree,
                   "search less than element");
    }

    #[test]
    fn test_search_for_elements_in_tree() {
        let tree = [0,0,4,  0,0,1,  2,2,3,  4,4,5,  6,6,7];

        let test_case = | of: i32, expected_index: u32, expected_leaf: u32, on_fail: &str |
            assert_eq!(search_for_lower_bound(of, 4, &tree),
                       SearchTreeIndex::Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        test_case(2,7,2,"searching for element in middle of tree");
        test_case(0,4,0,"searching for the smallest element");
        test_case(7,14,7,"searching for the greatest element");
    }

    #[test]
    fn test_search_for_elements_not_in_tree() {
        let tree = [0,0,4,  0,0,1,  2,2,3,  4,4,55,  66,66,77];

        let test_case = | of: i32, expected_index: u32, expected_leaf: u32, on_fail: &str |
            assert_eq!(search_for_lower_bound(of, 4, &tree),
                       SearchTreeIndex::Leaf { index: expected_index, leaf_number: expected_leaf },
                       "{}", on_fail);

        assert_eq!(search_for_lower_bound(-5, 4, &tree),
            SearchTreeIndex::NotInTree,
            "element lower than tree range"
        );

        test_case(58,11,5, "element in range but not in tree");
        test_case(800,14,7, "element greater than tree");
    }
}
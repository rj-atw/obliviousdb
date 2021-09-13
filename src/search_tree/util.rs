pub fn is_odd(element: u16) -> bool {
    return (1 & element) > 0
}

pub fn size_of_tree_with_height(height: u16) -> u64 {
    // 2 ^ (height) - 1
    (1 << height) - 1
}

pub fn number_of_leaves_in_tree(height: u16) -> u64 {
    // 2^(height-1)
    1 << (height - 1)
}

#[cfg(test)]
mod tests {
    use crate::search_tree::util::{is_odd};

    #[test]
    fn test_is_odd_helper() {
        assert_eq!(is_odd(129), true, "Test is_odd on an odd number");
        assert_eq!(is_odd(524), false, "Test is_odd on an even number");
    }


}
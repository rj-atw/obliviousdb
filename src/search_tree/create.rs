use crate::search_tree::util::{is_odd, size_of_tree_with_height};

//https://fasterthanli.me/articles/recursive-iterators-rust

struct LayoutSubtreesAndGetMinValues<'a, T>
    where T : Iterator<Item=i32> {
    reserved_space: &'a mut [i32],
    generator: T,
    height_of_subtree: u16,
    subtree_number: usize,
}

impl <T> Iterator for LayoutSubtreesAndGetMinValues<'_, T> where T: Iterator<Item=i32> {
    type Item=i32;

    fn next(&mut self) -> Option<Self::Item> {
        let size_of_subtree = size_of_tree_with_height(self.height_of_subtree) as usize;
        let (_, cdr) =
            self.reserved_space.split_at_mut((self.subtree_number*2)*size_of_subtree);
        let (two_subtrees, _) =
            cdr.split_at_mut(size_of_subtree*2);
        let (first_tree, second_tree) = two_subtrees.split_at_mut(size_of_subtree);

        self.subtree_number+=1;

        //ToDo Handle errors
        if let Ok(min) = layout(first_tree, Box::new(self.generator.by_ref()), self.height_of_subtree) {
            layout(second_tree, Box::new(self.generator.by_ref()), self.height_of_subtree).unwrap();
            Some(min)
        } else {
            None
        }
    }
}

fn layout_tree_of_height_1<'a>(
    reserved_space: &mut [i32],
    mut generator:  Box<dyn Iterator<Item=i32> + 'a>,
) -> Result<i32, ()> {
    if let Some(element) = generator.next() {
        reserved_space[0] = element;
        Ok(reserved_space[0])
    } else {
        Err(())
    }
}

fn layout_tree_of_height_2<'a>(
    reserved_space: &mut [i32],
    mut generator:  Box<dyn Iterator<Item=i32> + 'a>,
) -> Result<i32, ()> {
    for leaf_offset in 1..=2 {
        if let Some(element) = generator.next() {
            reserved_space[leaf_offset] = element;
        } else {
            return Err(())
        }
    }
    reserved_space[0] = reserved_space[1];
    Ok(reserved_space[0])
}


fn layout_tree_of_height_3<'a>(
    reserved_space: &mut [i32],
    mut generator:  Box<dyn Iterator<Item=i32> + 'a>,
) -> Result<i32, ()> {
    for leaf_offset in 3..=6 {
        if let Some(element) = generator.next() {
            reserved_space[leaf_offset] = element;
        } else {
            return Err(())
        }
    }
    reserved_space[0] = reserved_space[3];
    reserved_space[1] = reserved_space[3];
    reserved_space[2] = reserved_space[5];

    Ok(reserved_space[3])
}

pub fn layout<'a>(
    reserved_space: &mut [i32],
    generator:  Box<dyn Iterator<Item=i32> + 'a>,
    height: u16
) -> Result<i32, ()> {
    match height {
        1 => { layout_tree_of_height_1(reserved_space, generator) }
        2 => { layout_tree_of_height_2(reserved_space, generator) }
        3 => { layout_tree_of_height_3(reserved_space, generator) }
        _ => {
            let height_of_bottom_subtree = height / 2;
            let height_of_top_subtree = height_of_bottom_subtree + if is_odd(height) {1} else {0};

            let size_of_top_subtree = size_of_tree_with_height(height_of_top_subtree);

            let (top_subtree, bottom_subtree) =
                reserved_space.split_at_mut(size_of_top_subtree as usize);

            let min_values = LayoutSubtreesAndGetMinValues {
                reserved_space: bottom_subtree,
                generator,
                height_of_subtree: height_of_bottom_subtree,
                subtree_number: 0
            };

            layout(top_subtree, Box::new(min_values), height_of_top_subtree)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::search_tree::create::layout;

    #[test]
    fn base_case_layout_tree_of_height_1() {
        let mut reserved_space = [1];
        assert_eq!(layout( &mut reserved_space,  Box::new(vec![6].into_iter()), 1), Ok(6));

        assert_eq!(reserved_space, [6]);
    }

    #[test]
    fn base_case_layout_tree_of_height_3() {
        let mut reserved_space = [0;7];
        let leafs = vec![0,1,2,3];

        assert_eq!(layout(&mut reserved_space, Box::new(leafs.into_iter()), 3), Ok(0));

        assert_eq!(reserved_space, [0,0,2, 0,1, 2,3]);
    }

    #[test]
    fn recursive_case_layout_tree_of_height_4() {
        let mut reserved_space = [0; 15];
        let leafs = vec![0,1,2,3,4,5,6,7];

        assert_eq!(layout(&mut reserved_space, Box::new(leafs.into_iter()), 4), Ok(0));

        assert_eq!(reserved_space, [0,0,4, 0,0,1, 2,2,3, 4,4,5, 6,6,7]);
    }

    #[test]
    fn recursive_case_layout_tree_of_height_7() {
        let mut reserved_space = [0; 127];
        let leaves : Vec<i32> = (0..64).collect();

        let expected = [
             0,  0, 32,

             0,  0,  8,
            16, 16, 24,
            32, 32, 40,
            48, 48, 56,

             0,  0,  2,
             0,  1,  2,  3,

             4,  4,  6,
             4,  5,  6,  7,

             8,  8, 10,
             8,  9, 10, 11,

            12, 12, 14,
            12, 13, 14, 15,

            16, 16, 18,
            16, 17, 18, 19,

            20, 20, 22,
            20, 21, 22, 23,

            24, 24, 26,
            24, 25, 26, 27,

            28, 28, 30,
            28, 29, 30, 31,

            32, 32, 34,
            32, 33, 34, 35,

            36, 36, 38,
            36, 37, 38, 39,

            40, 40, 42,
            40, 41, 42, 43,

            44, 44, 46,
            44, 45, 46, 47,

            48, 48, 50,
            48, 49, 50, 51,

            52, 52, 54,
            52, 53, 54, 55,

            56, 56, 58,
            56, 57, 58, 59,

            60, 60, 62,
            60, 61, 62, 63
        ];

        assert_eq!(layout(&mut reserved_space, Box::new(leaves.into_iter()), 7), Ok(0));

        assert_eq!(reserved_space, expected);
    }
}
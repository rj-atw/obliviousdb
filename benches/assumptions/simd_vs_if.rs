use core_simd::*;

pub fn search_3_level_tree_for_lower_bound_simd(of: i32, array: &[i32]) -> i32 {
    const NULL: Simd<i32, 4> = i32x4::splat(6);
    const LEAF: Simd<i32, 4> = i32x4::from_array([0, 3, 4, 5]);

    let of_simd = i32x4::splat(of);
    let base_simd = i32x4::from_array([array[3], array[4], array[5], array[6]]);

    let s = of_simd.lanes_lt(base_simd);

    let selected = s.select(LEAF, NULL);
    let idx = selected.horizontal_min();

    idx - 3
}

pub fn search_3_level_tree_for_lower_bound(of: i32, array: &[i32]) -> i32 {
    return if of < array[3] {
        9
    } else if of < array[4] {
        0
    } else if of < array[5] {
        1
    } else if of < array[6] {
        2
    } else {
        3
    }
}




//! # Overview
//! Gives all combinations of a vec that contains unique items.
//!
//! ## The algorithm
//! This crate uses binary iteration to generate the possible combinations by
//! utilizing the 1s bits and 1s counts to get the combinations.
//!
//! If given a vec \[1, 2, 3\] it will loop from 1 to 1 << 3 (that is binary 1000
//! and decimal 8) which gives the 7 combinations of \[1, 2, 3\].
//!
//! This gives the combinations: \[001, 010, 011, 100, 101, 110, 111\]
//! Which is: \[\[1\], \[2\], \[1, 2\], \[3\], \[1, 3\], \[2, 3\], \[1, 2, 3\]\]
//!
//! ## functionality
//! - all combinations
//! - all qualifying combinations
//! - positions of all qualifying combinations
//! - combinations of a particular length
//! - positions of combinations of a particular length
//! - positions of qualifying combinations of a particular length
//!
//! *NOTE: positions is the position in the vector of vectors of all combinations
//! possible, where the list is 1 indexed rather than 0. Therefore if you have
//! vector \[1, 2\] and your total set is \[\[1\], \[2\], \[1, 2\]\] then position 1 is
//! the vector \[1\] and position 3 is the vector \[1, 2\]*
//!
//! ## Why unique only
//! Providing a vec with non-unique items, it will give you more than you want.
//!
//! This crate could have been made more robust to prevent duplication, but
//! that would reduce its almost completely generic application (only requiring
//! Copy) as well as make it less performant. If you require a crate that
//! handles cases of non-unique vectors, please see:
//! <https://crates.io/crates/combinations>
//!
//! ### Example:
//! this fails:
//! ```
//!  use combinations::combinations;
//!
//!  let actual = combinations(&vec![1, 2, 2, 3], 3);
//!  let expected = vec![
//!      vec![1, 2, 2],
//!      vec![1, 2, 3],
//!      vec![2, 2, 3],
//!  ];
//!  // Following would fail!
//!  // assert_eq!(actual, expected);
//! ```
//! this passes:
//! ```
//!  use combinations::combinations;
//!
//!  let actual = combinations(&vec![1, 2, 2, 3], 3);
//!  let expected = vec![
//!      vec![1, 2, 2],
//!      vec![1, 2, 3],
//!      vec![1, 2, 3], // note duplicate
//!      vec![2, 2, 3],
//!  ];
//! assert_eq!(actual, expected);
//! ```

/// get_subset gets the nth subset of all possible combination subsets
///that could be gotten from v when using the all function
pub fn get_subset<T: Copy>(v: &Vec<T>, pos: u128) -> Vec<T> {
    let mut subset: Vec<T> = vec![];
    for i in 0..v.len() {
        if (pos >> i) & 1 == 1 {
            subset.push(v[i]);
        }
    }
    subset
}

/// all gets every combination subset that is possible for the set v
pub fn all<T: Copy>(v: &Vec<T>) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = vec![];

    let u_one: u128 = 1;

    for pos in 1..(u_one << v.len()) {
        subsets.push(get_subset(&v, pos));
    }
    subsets
}

/// all_qualifying gets the combination subsets that qualify according to the
/// criteria defined in the qualifies callback function
pub fn all_qualifying<T: Copy>(v: &Vec<T>, qualifies: fn(&Vec<T>) -> bool) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = vec![];

    let u_one: u128 = 1;

    for pos in 1..(u_one << v.len()) {
        let subset = get_subset(&v, pos);
        if qualifies(&subset) {
            subsets.push(subset);
        }
    }
    subsets
}

/// all_qualifying_positions gets the positions of the combination subsets that
/// would be generated on the nth iteration from the all function if that
/// position's subset qualifies according to the criteria defined in the
/// qualifies callback function
pub fn all_qualifying_positions<T: Copy>(v: &Vec<T>, qualifies: fn(&Vec<T>) -> bool) -> Vec<u128> {
    let mut positions: Vec<u128> = vec![];

    let u_one: u128 = 1;

    for pos in 1..(u_one << v.len()) {
        let subset = get_subset(&v, pos);
        if qualifies(&subset) {
            positions.push(pos);
        }
    }
    positions
}

/// combinations gets the combination subsets that are possible for the set v
/// that are the length of the sample size r
pub fn combinations<T: Copy>(v: &Vec<T>, r: u32) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = vec![];

    let u_one: u128 = 1;

    for pos in 1..(u_one << v.len()) {
        if pos.count_ones() != r {
            continue;
        }
        subsets.push(get_subset(&v, pos));
    }
    subsets
}

/// combinations_positions gets the positions of the combination subsets that
/// are possible for the set v that are the length of the sample size r where
/// the position is the nth iteration from the all function
pub fn combinations_positions<T: Copy>(v: &Vec<T>, r: u32) -> Vec<u128> {
    let mut positions: Vec<u128> = vec![];

    let u_one: u128 = 1;

    for pos in 1..(u_one << v.len()) {
        if pos.count_ones() != r {
            continue;
        }
        positions.push(pos);
    }
    positions
}

/// qualifying_combinations_positions gets the positions of the combination
/// subsets that are possible for the set v that are the length of the sample
/// size r where the position is the nth iteration from the all function and
/// the position's subset qualifies according to the criteria defined in the
/// qualifies callback function
pub fn combinations_qualifying_positions<T: Copy>(
    v: &Vec<T>,
    r: u32,
    qualifies: fn(&Vec<T>) -> bool,
) -> Vec<u128> {
    let mut positions: Vec<u128> = vec![];

    let u_one: u128 = 1;

    for pos in 1..(u_one << v.len()) {
        if pos.count_ones() != r {
            continue;
        }
        let subset = get_subset(&v, pos);
        if qualifies(&subset) {
            positions.push(pos);
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subset() {
        let result = get_subset(&(1..4).collect(), 3);
        assert_eq!(result, vec![1, 2]);
        let result = get_subset(&(1..4).collect(), 6);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_all() {
        let result = all(&(1..4).collect());
        assert_eq!(
            result,
            vec![
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_all_qualifying() {
        let result = all_qualifying(&(1..4).collect(), |v: &Vec<i32>| -> bool {
            let sum: i32 = v.iter().sum();
            sum < 5
        });
        assert_eq!(
            result,
            vec![vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3]]
        );
    }

    #[test]
    fn test_all_qualifying_positions() {
        let result = all_qualifying_positions(&(1..4).collect(), |v: &Vec<i32>| -> bool {
            let sum: i32 = v.iter().sum();
            sum < 5
        });
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_combinations() {
        let result = combinations(&(1..4).collect(), 2);
        assert_eq!(result, vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
    }

    #[test]
    fn test_combinations_positions() {
        let result = combinations_positions(&(1..21).collect(), 10);
        assert_eq!(result.len(), 184756);
        let result = combinations_positions(&(1..21).collect(), 11);
        assert_eq!(result.len(), 167960);
        let result = combinations_positions(&(1..21).collect(), 9);
        assert_eq!(result.len(), 167960);
    }

    #[test]
    fn test_combinations_qualifying_positions() {
        let result =
            combinations_qualifying_positions(&(1..4).collect(), 2, |v: &Vec<i32>| -> bool {
                let sum: i32 = v.iter().sum();
                sum < 5
            });
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_complicated() {
        let actual = combinations(&vec![1, 2, 2, 3], 3);
        let expected = vec![vec![1, 2, 2], vec![1, 2, 3], vec![1, 2, 3], vec![2, 2, 3]];
        assert_eq!(actual, expected)
    }
}

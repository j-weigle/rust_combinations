# Overview
Gives all combinations of a vec that contains unique items.

## The algorithm
This crate uses binary iteration to generate the possible combinations by
utilizing the 1s bits and 1s counts to get the combinations.

If given a vec \[1, 2, 3\] it will loop from 1 to 1 << 3 (that is binary 1000
and decimal 8) which gives the 7 combinations of \[1, 2, 3\].

This gives the combinations: \[001, 010, 011, 100, 101, 110, 111\]
Which is: \[\[1\], \[2\], \[1, 2\], \[3\], \[1, 3\], \[2, 3\], \[1, 2, 3\]\]

## functionality
- all combinations
- all qualifying combinations
- positions of all qualifying combinations
- combinations of a particular length
- positions of combinations of a particular length
- positions of qualifying combinations of a particular length

*NOTE: positions is the position in the vector of vectors of all combinations
possible, where the list is 1 indexed rather than 0. Therefore if you have
vector \[1, 2\] and your total set is \[\[1\], \[2\], \[1, 2\]\] then position 1 is
the vector \[1\] and position 3 is the vector \[1, 2\]*

## Why unique only
Providing a vec with non-unique items, it will give you more than you want.

This crate could have been made more robust to prevent duplication, but
that would reduce its almost completely generic application (only requiring
Copy) as well as make it less performant. If you require a crate that
handles cases of non-unique vectors, please see:
<https://crates.io/crates/combinations>

### Example:
this fails:
```
 use combinations::combinations;

 let actual = combinations(&vec![1, 2, 2, 3], 3);
 let expected = vec![
     vec![1, 2, 2],
     vec![1, 2, 3],
     vec![2, 2, 3],
 ];
 // Following would fail!
 // assert_eq!(actual, expected);
```
this passes:
```
 use combinations::combinations;

 let actual = combinations(&vec![1, 2, 2, 3], 3);
 let expected = vec![
     vec![1, 2, 2],
     vec![1, 2, 3],
     vec![1, 2, 3], // note duplicate
     vec![2, 2, 3],
 ];
assert_eq!(actual, expected);
```
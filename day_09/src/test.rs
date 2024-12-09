use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_09() {
    let solution = Solution::new(INPUT);
    assert_eq!(6301895872542, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::new("2333133121414131402");
    assert_eq!(1928, solution.part1().unwrap());
    assert_eq!(2858, solution.part2().unwrap());
}

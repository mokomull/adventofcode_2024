use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_11() {
    let solution = Solution::new(INPUT);
    assert_eq!(175006, solution.part1().unwrap());
    assert_eq!(207961583799296, solution.part2().unwrap());
}

use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_05() {
    let solution = Solution::new(INPUT);
    assert_eq!(5391, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

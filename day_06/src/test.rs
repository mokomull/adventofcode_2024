use crate::Solution;
use prelude::Day;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_06() {
    let solution = Solution::new(INPUT);
    assert_eq!(5305, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

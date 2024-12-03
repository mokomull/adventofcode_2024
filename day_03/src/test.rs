use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_03() {
    let solution = Solution::new(INPUT);
    assert_eq!(192767529, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn example_part2() {
    let solution = Solution::new("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    assert_eq!(48, solution.part2().unwrap());
}
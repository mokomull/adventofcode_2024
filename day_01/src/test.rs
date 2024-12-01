use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_01() {
    let solution = Solution::new(INPUT);
    assert_eq!(3508942, solution.part1().unwrap());
    assert_eq!(26593248, solution.part2().unwrap());
}

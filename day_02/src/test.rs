use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_02() {
    let day_02 = Solution::new(INPUT);
    assert_eq!(0, day_02.part1().unwrap());
    assert_eq!(0, day_02.part2().unwrap());
}

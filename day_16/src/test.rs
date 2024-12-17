use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_16() {
    let solution = Solution::new(INPUT);
    assert_eq!(74392, solution.part1().unwrap());
    assert_eq!(426, solution.part2().unwrap());
}

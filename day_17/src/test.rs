use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_17() {
    let solution = Solution::new(INPUT);
    assert_eq!("", &solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

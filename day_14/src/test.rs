use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_14() {
    let solution = Solution::new(INPUT);
    assert_eq!(0, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

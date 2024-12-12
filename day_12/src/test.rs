use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_12() {
    let solution = Solution::new(INPUT);
    assert_eq!(0, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::new(
        "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
    );
    assert_eq!(772, solution.part1().unwrap());
}

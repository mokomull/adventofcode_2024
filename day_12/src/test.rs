use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_12() {
    let solution = Solution::new(INPUT);
    assert_eq!(1361494, solution.part1().unwrap());
    assert_eq!(830516, solution.part2().unwrap());
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

#[test]
fn small_example() {
    let solution = Solution::new(
        "AAAA
BBCD
BBCC
EEEC",
    );
    assert_eq!(80, solution.part2().unwrap());
}

use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_10() {
    let solution = Solution::new(INPUT);
    assert_eq!(593, solution.part1().unwrap());
    assert_eq!(1192, solution.part2().unwrap());
}

#[test]
fn simple_example() {
    let solution = Solution::new(
        "0123
1234
8765
9876",
    );
    assert_eq!(1, solution.part1().unwrap());
}

#[test]
fn larger_example() {
    let solution = Solution::new(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );
    assert_eq!(36, solution.part1().unwrap());
    assert_eq!(81, solution.part2().unwrap());
}

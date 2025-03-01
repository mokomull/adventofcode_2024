use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_04() {
    let solution = Solution::new(INPUT);
    assert_eq!(2593, solution.part1().unwrap());
    assert_eq!(1950, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::new(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
    );

    assert_eq!(18, solution.part1().unwrap());
    assert_eq!(9, solution.part2().unwrap());
}

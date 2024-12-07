use prelude::Day;

use crate::Solution;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_07() {
    let solution = Solution::new(INPUT);
    assert_eq!(1153997401072, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::new(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    );
    assert_eq!(3749, solution.part1().unwrap());
    assert_eq!(11387, solution.part2().unwrap());
}

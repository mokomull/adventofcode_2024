use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_17() {
    let solution = Solution::new(INPUT);
    assert_eq!("", &solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::new(
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    );
    assert_eq!("4,6,3,5,6,3,5,2,1,0", &solution.part1().unwrap())
}

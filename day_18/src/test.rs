use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_18() {
    let solution = Solution::<1024, 70>::new(INPUT);
    assert_eq!(380, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::<12, 6>::new(EXAMPLE_INPUT);
    assert_eq!(22, solution.part1().unwrap());
}

static EXAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

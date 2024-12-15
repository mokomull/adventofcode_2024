use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_15() {
    let solution = Solution::new(INPUT);
    assert_eq!(1429911, solution.part1().unwrap());
    assert_eq!(1453087, solution.part2().unwrap());
}

#[test]
fn example() {
    let solution = Solution::new(
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    );
    assert_eq!(105 + 207 + 306, solution.part2().unwrap());
}

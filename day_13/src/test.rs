use prelude::Day;

use crate::*;

static INPUT: &str = include_str!("input.txt");

#[test]
fn day_13() {
    let solution = Solution::new(INPUT);
    assert_eq!(31552, solution.part1().unwrap());
    assert_eq!(0, solution.part2().unwrap());
}

#[test]
fn first_claw_machine_example() {
    let machine = Machine {
        button_a: Pair(94, 34),
        button_b: Pair(22, 67),
        prize: Pair(8400, 5400),
    };

    assert_eq!(Some(Pair(80, 40)), machine.solve())
}

#[test]
fn second_claw_machine_example() {
    let machine = Machine {
        button_a: Pair(26, 66),
        button_b: Pair(67, 21),
        prize: Pair(12748, 12167),
    };

    assert_eq!(None, machine.solve())
}

#[test]
fn third_claw_machine_example() {
    let machine = Machine {
        button_a: Pair(17, 86),
        button_b: Pair(84, 37),
        prize: Pair(7870, 6450),
    };

    assert_eq!(Some(Pair(38, 86)), machine.solve())
}

#[test]
fn fourth_claw_machine_example() {
    let machine = Machine {
        button_a: Pair(69, 23),
        button_b: Pair(27, 71),
        prize: Pair(18641, 10279),
    };

    assert_eq!(None, machine.solve())
}

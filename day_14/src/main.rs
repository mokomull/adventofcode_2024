use std::io::stdin;

use day_14::Solution;
use prelude::Day;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut solution = Solution::new(INPUT);
    for i in 1.. {
        solution.step();
        println!("after {i} steps:");
        solution.render();

        let mut junk = String::new();
        stdin().read_line(&mut junk).unwrap();
    }
}

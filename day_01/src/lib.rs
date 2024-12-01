#[cfg(test)]
mod test;

use prelude::*;

struct Solution {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut left = vec![];
        let mut right = vec![];

        for line in input.lines() {
            let mut split = line.split_whitespace();
            left.push(split.next().unwrap().parse().unwrap());
            right.push(split.next().unwrap().parse().unwrap());
        }

        Self { left, right }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

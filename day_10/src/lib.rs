#[cfg(test)]
mod test;

use prelude::*;

struct Solution {
    map: Vec<Vec<u8>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect();
        Self { map }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

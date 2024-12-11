use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    stones: Vec<u64>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Self {
            stones: input
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

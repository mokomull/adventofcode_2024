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
        Ok(self
            .stones
            .iter()
            .map(|&s| count_stones(25, s) as u64)
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        Ok(self
            .stones
            .iter()
            .map(|&s| count_stones(75, s) as u64)
            .sum())
    }
}

fn count_stones(remaining_count: u8, stone: u64) -> usize {
    if remaining_count == 0 {
        return 1;
    }

    if stone == 0 {
        return count_stones(remaining_count - 1, 1);
    }

    let digits = stone.to_string();
    if digits.len() % 2 == 0 {
        return count_stones(
            remaining_count - 1,
            digits[..digits.len() / 2].parse().unwrap(),
        ) + count_stones(
            remaining_count - 1,
            digits[digits.len() / 2..].parse().unwrap(),
        );
    }

    count_stones(remaining_count - 1, stone * 2024)
}

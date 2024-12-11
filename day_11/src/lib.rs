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
            .map(|&s| count_stones(25, s, &mut HashMap::new()))
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        Ok(self
            .stones
            .iter()
            .map(|&s| count_stones(75, s, &mut HashMap::new()))
            .sum())
    }
}

fn count_stones(remaining_count: u8, stone: u64, memo: &mut HashMap<(u8, u64), u64>) -> u64 {
    if remaining_count == 0 {
        return 1;
    }

    if let Some(&x) = memo.get(&(remaining_count, stone)) {
        return x;
    }

    if stone == 0 {
        let res = count_stones(remaining_count - 1, 1, memo);
        memo.insert((remaining_count, stone), res);
        return res;
    }

    let digits = stone.to_string();
    if digits.len() % 2 == 0 {
        let res = count_stones(
            remaining_count - 1,
            digits[..digits.len() / 2].parse().unwrap(),
            memo,
        ) + count_stones(
            remaining_count - 1,
            digits[digits.len() / 2..].parse().unwrap(),
            memo,
        );
        memo.insert((remaining_count, stone), res);
        return res;
    }

    let res = count_stones(remaining_count - 1, stone * 2024, memo);
    memo.insert((remaining_count, stone), res);
    res
}

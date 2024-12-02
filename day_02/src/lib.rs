use std::cmp::{max, min};

use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    levels: Vec<Vec<u64>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let levels = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        Self { levels }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self.levels.iter().filter(|&levels| is_safe(levels)).count() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        Ok(self
            .levels
            .iter()
            .filter(|&levels| -> bool {
                if is_safe(levels) {
                    return true;
                }

                for i in 0..levels.len() {
                    let mut test_level = levels.clone();
                    test_level.remove(i);
                    if is_safe(&test_level) {
                        return true;
                    }
                }

                false
            })
            .count() as u64)
    }
}

fn is_safe(levels: &[u64]) -> bool {
    let mut direction = None;
    let mut last = levels[0];

    for &i in &levels[1..] {
        if max(i, last) - min(i, last) > 3 {
            return false;
        }

        if i == last {
            return false;
        }

        let this_direction = i.cmp(&last);
        match direction {
            Some(x) if x != this_direction => {
                return false;
            }
            _ => {}
        }

        last = i;
        direction = Some(this_direction);
    }

    true
}

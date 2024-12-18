use std::collections::VecDeque;

use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    bytes: Vec<(u8, u8)>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Self {
            bytes: input
                .lines()
                .map(|l| {
                    let (left, right) = l.split_once(',').unwrap();
                    (left.parse().unwrap(), right.parse().unwrap())
                })
                .collect(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let corrupted = self
            .bytes
            .iter()
            .cloned()
            .take(1024)
            .collect::<HashSet<_>>();

        let mut to_visit = VecDeque::from([((0, 0), 0)]);
        let mut visited = HashSet::new();

        while let Some(((i, j), d)) = to_visit.pop_front() {
            if !visited.insert((i, j)) {
                // we've already been here
                continue;
            }

            if corrupted.contains(&(i, j)) {
                // we can't enter a corrupted square
                continue;
            }

            if (i, j) == (70, 70) {
                return Ok(d);
            }

            if i > 0 {
                to_visit.push_back(((i - 1, j), d + 1));
            }
            if j > 0 {
                to_visit.push_back(((i, j - 1), d + 1));
            }
            if i < 70 {
                to_visit.push_back(((i + 1, j), d + 1));
            }
            if j < 70 {
                to_visit.push_back(((i, j + 1), d + 1));
            }
        }

        anyhow::bail!("No path found :'(");
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

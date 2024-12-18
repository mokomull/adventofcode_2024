use std::collections::VecDeque;

use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution<const COUNT: usize, const DIMENSION: u8> {
    bytes: Vec<(u8, u8)>,
}

impl<const COUNT: usize, const DIMENSION: u8> Day for Solution<COUNT, DIMENSION> {
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
            .take(COUNT)
            .collect::<HashSet<_>>();

        assert_eq!(COUNT, corrupted.len());

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

            if (i, j) == (DIMENSION, DIMENSION) {
                return Ok(d);
            }

            if i > 0 {
                to_visit.push_back(((i - 1, j), d + 1));
            }
            if j > 0 {
                to_visit.push_back(((i, j - 1), d + 1));
            }
            if i < DIMENSION {
                to_visit.push_back(((i + 1, j), d + 1));
            }
            if j < DIMENSION {
                to_visit.push_back(((i, j + 1), d + 1));
            }
        }

        anyhow::bail!("No path found :'(");
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

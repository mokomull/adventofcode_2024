use std::collections::VecDeque;

use anyhow::Ok;
use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution<const COUNT: usize, const DIMENSION: u8> {
    bytes: Vec<(u8, u8)>,
}

impl<const COUNT: usize, const DIMENSION: u8> Solution<COUNT, DIMENSION> {
    pub fn new(input: &str) -> Self {
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

    pub fn part1(&self) -> anyhow::Result<u64> {
        let corrupted = self
            .bytes
            .iter()
            .cloned()
            .take(COUNT)
            .collect::<HashSet<_>>();

        assert_eq!(COUNT, corrupted.len());

        distance_to_end::<DIMENSION>(&corrupted)
    }

    pub fn part2(&self) -> anyhow::Result<String> {
        let mut corrupted = HashSet::new();

        for &(i, j) in &self.bytes {
            corrupted.insert((i, j));

            if let Err(_) = distance_to_end::<DIMENSION>(&corrupted) {
                return Ok(format!("{i},{j}"));
            }
        }

        anyhow::bail!("the end is always reachable");
    }
}

fn distance_to_end<const DIMENSION: u8>(corrupted: &HashSet<(u8, u8)>) -> anyhow::Result<u64> {
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

use std::collections::VecDeque;

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

        find_end::<DIMENSION>(&corrupted).map(|p| p.len() as u64)
    }

    pub fn part2(&self) -> anyhow::Result<String> {
        let mut corrupted = HashSet::new();
        let mut last_path: Option<Vec<(u8, u8)>> = None;

        for &(i, j) in &self.bytes {
            corrupted.insert((i, j));

            if last_path.is_none() || last_path.as_ref().unwrap().contains(&(i, j)) {
                match find_end::<DIMENSION>(&corrupted) {
                    Err(_) => return Ok(format!("{i},{j}")),
                    Ok(p) => last_path = Some(p),
                }
            }
        }

        anyhow::bail!("the end is always reachable");
    }
}

fn find_end<const DIMENSION: u8>(corrupted: &HashSet<(u8, u8)>) -> anyhow::Result<Vec<(u8, u8)>> {
    let mut to_visit = VecDeque::from([((0, 0), vec![])]);
    let mut visited = HashSet::new();

    while let Some(((i, j), mut p)) = to_visit.pop_front() {
        if !visited.insert((i, j)) {
            // we've already been here
            continue;
        }

        if corrupted.contains(&(i, j)) {
            // we can't enter a corrupted square
            continue;
        }

        if (i, j) == (DIMENSION, DIMENSION) {
            return Ok(p);
        }

        p.push((i, j));

        if i > 0 {
            to_visit.push_back(((i - 1, j), p.clone()));
        }
        if j > 0 {
            to_visit.push_back(((i, j - 1), p.clone()));
        }
        if i < DIMENSION {
            to_visit.push_back(((i + 1, j), p.clone()));
        }
        if j < DIMENSION {
            to_visit.push_back(((i, j + 1), p));
        }
    }

    anyhow::bail!("No path found :'(");
}

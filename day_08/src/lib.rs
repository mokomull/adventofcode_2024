#[cfg(test)]
mod test;

use prelude::*;

pub struct Solution {
    antennas: HashMap<u8, HashSet<(usize, usize)>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut antennas: HashMap<u8, HashSet<(usize, usize)>> = HashMap::new();

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                if c != b'.' {
                    antennas.entry(c).or_default().insert((i, j));
                }
            }
        }
        Self { antennas }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

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
        let mut antinodes = HashSet::new();

        for (_frequency, locations) in self.antennas.iter() {
            for (p, q) in locations.iter().tuple_combinations() {
                if p == q {
                    continue;
                }

                // The antinode will be one-third of the way from p to q, which, in order to be
                // on an integer grid, means that both differences will be multiples of 3.
                let di = q.0 as i32 - p.0 as i32;
                let dj = q.1 as i32 - p.1 as i32;

                if di % 3 == 0 && dj % 3 == 0 {
                    antinodes.insert((p.0 as i32 + (di / 3), p.1 as i32 + (dj / 3)));
                }
            }
        }

        Ok(antinodes.len() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

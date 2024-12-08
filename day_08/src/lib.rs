#[cfg(test)]
mod test;

use prelude::*;

pub struct Solution {
    antennas: HashMap<u8, HashSet<(usize, usize)>>,
    height: usize,
    width: usize,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut antennas: HashMap<u8, HashSet<(usize, usize)>> = HashMap::new();
        let mut height = 0;
        let mut width = 0;

        for (i, line) in input.lines().enumerate() {
            height += 1;
            width = line.len();

            for (j, c) in line.bytes().enumerate() {
                if c != b'.' {
                    antennas.entry(c).or_default().insert((i, j));
                }
            }
        }
        Self {
            antennas,
            height,
            width,
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut antinodes = HashSet::new();

        for i in 0..self.height {
            for j in 0..self.width {
                for (_frequency, locations) in self.antennas.iter() {
                    for (p, q) in locations.iter().cartesian_product(locations.iter()) {
                        if p == q {
                            continue;
                        }

                        let location = (i, j);

                        if distance_squared(location, *p) == distance_squared(location, *q) {
                            antinodes.insert((i, j));
                        }
                    }
                }
            }
        }

        Ok(antinodes.len() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

fn distance_squared(a: (usize, usize), b: (usize, usize)) -> usize {
    use std::cmp::{max, min};
    let dx = max(a.0, b.0) - min(a.0, b.0);
    let dy = max(a.1, b.1) - min(a.1, b.1);

    dx * dx + dy * dy
}

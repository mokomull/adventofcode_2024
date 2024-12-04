use prelude::anyhow::anyhow;
use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    loc: HashMap<u8, HashSet<(i32, i32)>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut loc: HashMap<u8, HashSet<(i32, i32)>> = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                loc.entry(c).or_default().insert((i as i32, j as i32));
            }
        }

        Self { loc }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut count = 0;

        'x: for &(x_i, x_j) in self.loc.get(&b'X').ok_or_else(|| anyhow!("no Xes found"))? {
            for (di, dj) in [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ] {
                let mut i = x_i;
                let mut j = x_j;

                for letter in b"MAS" {
                    i += di;
                    j += dj;

                    let Some(letter_loc) = self.loc.get(letter) else {
                        continue 'x;
                    };

                    if !letter_loc.contains(&(i, j)) {
                        continue 'x;
                    }
                }

                count += 1;
            }
        }

        Ok(count)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

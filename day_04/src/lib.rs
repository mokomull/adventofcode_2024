use anyhow::bail;
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

        for &(x_i, x_j) in self.loc.get(&b'X').ok_or_else(|| anyhow!("no Xes found"))? {
            'next_direction: for (di, dj) in [
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

                eprintln!("found X at {:?}", (x_i, x_j));

                for letter in b"MAS" {
                    i += di;
                    j += dj;

                    let Some(letter_loc) = self.loc.get(letter) else {
                        bail!("wow, no {}s anywhere at all", letter);
                    };

                    if !letter_loc.contains(&(i, j)) {
                        eprintln!(
                            "looking for {} in direction {:?}; didn't find at {:?}",
                            letter,
                            (di, dj),
                            (i, j)
                        );
                        continue 'next_direction;
                    }
                }

                count += 1;
            }
        }

        Ok(count)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut count = 0;

        let a_s = self.loc.get(&b'A').ok_or_else(|| anyhow!("no A's found"))?;
        let m_s = self.loc.get(&b'M').ok_or_else(|| anyhow!("no M's found"))?;
        let s_s = self.loc.get(&b'S').ok_or_else(|| anyhow!("no S's found"))?;

        for (a_i, a_j) in a_s {
            let mut count_at_this_a = 0;

            for (di, dj) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                // M and S must be in opposite directions from the A.
                let m_i = a_i + di;
                let s_i = a_i - di;
                let m_j = a_j + dj;
                let s_j = a_j - dj;

                if m_s.contains(&(m_i, m_j)) && s_s.contains(&(s_i, s_j)) {
                    count_at_this_a += 1;
                }
            }

            if count_at_this_a == 2 {
                count += 1;
            }
        }

        Ok(count)
    }
}

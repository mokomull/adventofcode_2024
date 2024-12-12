use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    map: Vec<Vec<u8>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input.lines().map(|line| line.bytes().collect()).collect(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut to_visit = (0..self.map.len())
            .cartesian_product(0..self.map[0].len())
            .collect::<HashSet<_>>();

        let mut total = 0;

        while !to_visit.is_empty() {
            let mut perimeter = 0;
            let mut area = 0;
            let start = to_visit.iter().cloned().next().unwrap();
            let mut to_visit_this = HashSet::from([start]);

            eprintln!("visiting region {}", self.map[start.0][start.1] as char);

            while !to_visit_this.is_empty() {
                let this = to_visit_this.iter().cloned().next().unwrap();
                area += 1;
                to_visit.remove(&this);
                to_visit_this.remove(&this);

                let (i, j) = this;
                if i > 0 && self.map[i - 1][j] == self.map[i][j] {
                    if to_visit.contains(&(i - 1, j)) {
                        to_visit_this.insert((i - 1, j));
                    }
                } else {
                    perimeter += 1;
                }

                if j > 0 && self.map[i][j - 1] == self.map[i][j] {
                    if to_visit.contains(&(i, j - 1)) {
                        to_visit_this.insert((i, j - 1));
                    }
                } else {
                    perimeter += 1;
                }

                if i < self.map.len() - 1 && self.map[i + 1][j] == self.map[i][j] {
                    if to_visit.contains(&(i + 1, j)) {
                        to_visit_this.insert((i + 1, j));
                    }
                } else {
                    perimeter += 1;
                }

                if j < self.map[i].len() - 1 && self.map[i][j + 1] == self.map[i][j] {
                    if to_visit.contains(&(i, j + 1)) {
                        to_visit_this.insert((i, j + 1));
                    }
                } else {
                    perimeter += 1;
                }
            }

            eprintln!("area {}, perimiter {}", area, perimeter);
            total += area + perimeter;
        }

        Ok(total)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

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
            total += area * perimeter;
        }

        Ok(total)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut to_visit = (0..self.map.len())
            .cartesian_product(0..self.map[0].len())
            .collect::<HashSet<_>>();

        let mut total = 0;

        while !to_visit.is_empty() {
            #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
            enum Side {
                Top,
                Left,
                Bottom,
                Right,
            };
            use Side::*;

            let mut perimeter = HashSet::new();
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
                    perimeter.insert((i, j, Top));
                }

                if j > 0 && self.map[i][j - 1] == self.map[i][j] {
                    if to_visit.contains(&(i, j - 1)) {
                        to_visit_this.insert((i, j - 1));
                    }
                } else {
                    perimeter.insert((i, j, Left));
                }

                if i < self.map.len() - 1 && self.map[i + 1][j] == self.map[i][j] {
                    if to_visit.contains(&(i + 1, j)) {
                        to_visit_this.insert((i + 1, j));
                    }
                } else {
                    perimeter.insert((i, j, Bottom));
                }

                if j < self.map[i].len() - 1 && self.map[i][j + 1] == self.map[i][j] {
                    if to_visit.contains(&(i, j + 1)) {
                        to_visit_this.insert((i, j + 1));
                    }
                } else {
                    perimeter.insert((i, j, Right));
                }
            }

            dbg!(&perimeter);

            let mut sides = 0;

            while !perimeter.is_empty() {
                let side = perimeter.iter().cloned().next().unwrap();
                perimeter.remove(&(side));
                eprintln!("counting {:?}", side);
                sides += 1;

                let (i, j, s) = side;
                match s {
                    // Remove all the other things in perimeter that are on the same "side" as this
                    // one.
                    Left | Right => {
                        let mut up_i = i - 1;
                        while perimeter.contains(&(up_i, j, s)) {
                            eprintln!("ignoring {:?}", (up_i, j, s));
                            perimeter.remove(&(up_i, j, s));
                            up_i -= 1;
                        }

                        let mut down_i = i + 1;
                        while perimeter.contains(&(down_i, j, s)) {
                            eprintln!("ignoring {:?}", (down_i, j, s));
                            perimeter.remove(&(down_i, j, s));
                            down_i += 1;
                        }
                    }
                    Top | Bottom => {
                        let mut left_j = j - 1;
                        while perimeter.contains(&(i, left_j, s)) {
                            eprintln!("ignoring {:?}", (i, left_j, s));
                            perimeter.remove(&(i, left_j, s));
                            left_j -= 1;
                        }

                        let mut right_j = j + 1;
                        while perimeter.contains(&(i, right_j, s)) {
                            eprintln!("ignoring {:?}", (i, right_j, s));
                            perimeter.remove(&(i, right_j, s));
                            right_j += 1;
                        }
                    }
                }
            }

            eprintln!("area {}, sides {}", area, sides);
            total += area * sides;
        }

        Ok(total)
    }
}

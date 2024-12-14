use prelude::*;

#[cfg(test)]
mod test;

struct Robot {
    pub position: (i64, i64),
    velocity: (i64, i64),
}

pub struct Solution {
    robots: Vec<Robot>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut robots = Vec::new();

        for line in input.lines() {
            let (left, right) = line.split_once(' ').expect("no space");
            let left = left.strip_prefix("p=").expect("no p=");
            let right = right.strip_prefix("v=").expect("no v=");
            let (px, py) = left.split_once(',').expect("no comma");
            let (vx, vy) = right.split_once(',').expect("no comma");

            robots.push(Robot {
                position: (px.parse().unwrap(), py.parse().unwrap()),
                velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
            })
        }

        Self { robots }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut ending = Vec::new();
        for robot in &self.robots {
            ending.push((
                robot.position.0 + 100 * robot.velocity.0,
                robot.position.1 + 100 * robot.velocity.1,
            ));
        }

        for position in &mut ending {
            position.0 %= 101;
            if position.0 < 0 {
                position.0 += 101;
            }

            position.1 %= 103;
            if position.1 < 0 {
                position.1 += 103;
            }
        }

        let mut quadrants = [0; 4];
        for (x, y) in ending {
            let left = (0..50).contains(&x);
            let right = (51..101).contains(&x);
            let top = (0..51).contains(&y);
            let bottom = (52..103).contains(&y);

            if left && top {
                quadrants[0] += 1;
            } else if left && bottom {
                quadrants[1] += 1;
            } else if right && top {
                quadrants[2] += 1;
            } else if right && bottom {
                quadrants[3] += 1;
            }
        }

        Ok(quadrants.into_iter().product::<i64>() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

impl Solution {
    pub fn step(&mut self) {
        for robot in &mut self.robots {
            robot.position = (
                robot.position.0 + 100 * robot.velocity.0,
                robot.position.1 + 100 * robot.velocity.1,
            );

            robot.position.0 %= 101;
            if robot.position.0 < 0 {
                robot.position.0 += 101;
            }

            robot.position.1 %= 103;
            if robot.position.1 < 0 {
                robot.position.1 += 103;
            }
        }
    }

    fn to_bits(&self) -> [[bool; 101]; 103] {
        let mut map: [[bool; 101]; 103] = [[false; 101]; 103];
        for robot in &self.robots {
            map[robot.position.1 as usize][robot.position.0 as usize] = true;
        }
        map
    }

    pub fn render(&self) {
        for line in self.to_bits() {
            for c in line {
                if c {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    /// Assume that in this "picture of a Christmas tree", the top left 20x20 pixels will be blank
    pub fn is_maybe_christmas_tree(&self) -> bool {
        let bits = self.to_bits();
        for row in 0..20 {
            for col in 0..20 {
                if bits[row][col] {
                    return false;
                }
            }
        }

        true
    }
}

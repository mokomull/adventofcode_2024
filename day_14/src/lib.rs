use prelude::*;

#[cfg(test)]
mod test;

struct Robot {
    position: (i64, i64),
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
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

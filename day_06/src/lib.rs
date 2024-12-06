use std::fmt::Display;

use prelude::*;

#[cfg(test)]
mod test;

#[derive(Debug)]
struct CycleError();

type Point = (i32, i32);

impl Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("cycle detected")
    }
}

impl std::error::Error for CycleError {}

#[derive(Clone)]
pub struct Solution {
    obstacles: HashSet<(i32, i32)>,
    height: i32,
    width: i32,
    start: (i32, i32),
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut start = None;
        let mut obstacles = HashSet::new();

        for (i, line) in input.lines().enumerate() {
            height += 1;
            width = line.len() as i32;

            for (j, c) in line.bytes().enumerate() {
                match c {
                    b'^' => start = Some((i as i32, j as i32)),
                    b'#' => {
                        obstacles.insert((i as i32, j as i32));
                    }
                    _ => (),
                }
            }
        }

        Self {
            obstacles,
            height,
            width,
            start: start.unwrap(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(uniqueify(self.visit_path()?).len() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut count = 0;
        let normally_visited = uniqueify(self.visit_path()?);

        for i in 0..self.height {
            for j in 0..self.width {
                if !normally_visited.contains(&(i, j)) {
                    // we wouldn't even hit this location in the normal walking path, so the path is
                    // completely unchanged by putting an obstacle here.
                    continue;
                }

                let mut new_map = self.clone();
                new_map.obstacles.insert((i, j));

                match new_map.part1() {
                    Err(e) if e.is::<CycleError>() => {
                        count += 1;
                    }
                    _ => (),
                }
            }
        }
        Ok(count)
    }
}

impl Solution {
    fn visit_path(&self) -> anyhow::Result<HashSet<(Point, Point)>> {
        let mut visited = HashSet::new();
        let mut direction = (-1, 0);
        let mut location = self.start;

        while (0..self.height).contains(&location.0) && (0..self.width).contains(&location.1) {
            if !visited.insert((location, direction)) {
                return Err(anyhow::Error::from(CycleError()));
            }

            let mut next = (location.0 + direction.0, location.1 + direction.1);
            if self.obstacles.contains(&next) {
                let mut turns = 0;

                while self.obstacles.contains(&next) && turns < 4 {
                    direction = match direction {
                        (-1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        x => panic!("direction unexpected: {:?}", x),
                    };
                    next = (location.0 + direction.0, location.1 + direction.1);
                    turns += 1;
                }
                if turns >= 4 {
                    anyhow::bail!(
                        "we turned four times, and kept hitting an obstacle; nowhere to go"
                    );
                }
            }

            location = next;
        }

        Ok(visited)
    }
}

fn uniqueify(visited: HashSet<(Point, Point)>) -> HashSet<Point> {
    visited
        .into_iter()
        .map(|(location, _direction)| location)
        .collect()
}

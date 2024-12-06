use prelude::*;

#[cfg(test)]
mod test;

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
        let mut visited = HashSet::new();
        let mut direction = (-1, 0);
        let mut location = self.start;

        while (0..self.height).contains(&location.0) && (0..self.width).contains(&location.1) {
            visited.insert(location);

            let next = (location.0 + direction.0, location.1 + direction.1);
            if self.obstacles.contains(&next) {
                direction = match direction {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    x => panic!("direction unexpected: {:?}", x),
                };
                location = (location.0 + direction.0, location.1 + direction.1);
                if self.obstacles.contains(&location) {
                    anyhow::bail!("we turned, and we still hit an obstacle");
                }
            } else {
                location = next;
            }
        }

        Ok(visited.len() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

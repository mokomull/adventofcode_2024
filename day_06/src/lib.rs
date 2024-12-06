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
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

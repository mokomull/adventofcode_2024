use prelude::*;

#[cfg(test)]
mod test;

struct Solution {
    map: Vec<Vec<u8>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input.lines().map(|line| line.to_owned().into()).collect(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

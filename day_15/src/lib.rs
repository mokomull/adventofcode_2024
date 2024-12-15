use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    map: Vec<Vec<u8>>,
    moves: Vec<u8>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let mut map = Vec::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            map.push(line.into());
        }
        let mut moves = Vec::new();
        for line in lines {
            moves.extend_from_slice(line.trim().as_bytes());
        }

        Self { map, moves }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

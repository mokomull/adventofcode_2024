use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let register_a = lines.next().unwrap();
        let register_a = register_a.strip_prefix("Register A: ").unwrap();
        let a = register_a.parse().unwrap();

        let register_b = lines.next().unwrap();
        let register_b = register_b.strip_prefix("Register B: ").unwrap();
        let b = register_b.parse().unwrap();

        let register_c = lines.next().unwrap();
        let register_c = register_c.strip_prefix("Register C: ").unwrap();
        let c = register_c.parse().unwrap();

        let blank = lines.next().unwrap();
        assert!(blank.is_empty());

        let program = lines.next().unwrap();
        let program = program.strip_prefix("Program: ").unwrap();
        let program = program.split(',').map(|s| s.parse().unwrap()).collect();

        Self { a, b, c, program }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

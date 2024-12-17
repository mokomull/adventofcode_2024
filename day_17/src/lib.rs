use core::panic;
use std::ops::ControlFlow;

use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
}

impl Solution {
    pub fn new(input: &str) -> Self {
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

    pub fn part1(&self) -> anyhow::Result<String> {
        let mut computer = Computer::from(self);

        let mut output = Vec::new();

        while let ControlFlow::Continue(x) = computer.step() {
            if let Some(x) = x {
                output.push(x);
            }
        }

        Ok(output.into_iter().join(","))
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        for i in 0..=7 {
            // We can't actually look for zero length because the `out` instruction always seems to
            // execute at least once.  So try all of our initial conditions.
            if let Ok(res) = self.search(i, 1) {
                return Ok(res);
            }
        }
        anyhow::bail!("None was ever found")
    }

    fn search(&self, a: u64, target_length: usize) -> Result<u64, ()> {
        eprintln!("looking for length {target_length} with {a:b}");

        let mut computer = Computer::from(self);
        computer.a = a;
        let mut output = Vec::new();

        let mut seen = HashSet::new();
        while let ControlFlow::Continue(x) = computer.step() {
            if !seen.insert((computer.a, computer.b, computer.c, computer.ip)) {
                // we've hit an infinite loop.
                return Err(());
            }
            if let Some(x) = x {
                output.push(x);
            }
        }

        if output.len() == target_length && self.program.ends_with(&output) {
            if target_length == self.program.len() {
                return Ok(a);
            }

            for i in 0..=7 {
                if let Ok(res) = self.search((a << 3) + i, target_length + 1) {
                    return Ok(res);
                }
            }
        }

        Err(())
    }
}

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    ip: usize,
}

impl From<&Solution> for Computer {
    fn from(value: &Solution) -> Self {
        Self {
            a: value.a,
            b: value.b,
            c: value.c,
            program: value.program.clone(),
            ip: 0,
        }
    }
}

impl Computer {
    fn step(&mut self) -> ControlFlow<(), Option<u64>> {
        if self.ip >= self.program.len() {
            return ControlFlow::Break(());
        }

        let mut output = None;
        let mut dont_inc = false;
        match self.program[self.ip] {
            0 => {
                self.a >>= self.this_combo();
            }
            1 => {
                self.b = self.b ^ self.program[self.ip + 1];
            }
            2 => {
                self.b = self.this_combo() % 8;
            }
            3 => {
                if self.a != 0 {
                    self.ip = self.program[self.ip + 1] as usize;
                    dont_inc = true;
                }
            }
            4 => {
                self.b = self.b ^ self.c;
            }
            5 => {
                output = Some(self.this_combo() % 8);
            }
            6 => {
                self.b = self.a >> self.this_combo();
            }
            7 => {
                self.c = self.a >> self.this_combo();
            }

            x => panic!("unexpected instruction {x}"),
        }

        if !dont_inc {
            self.ip += 2;
        }

        ControlFlow::Continue(output)
    }

    fn this_combo(&self) -> u64 {
        match self.program[self.ip + 1] {
            x @ 0..=3 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            x => panic!("unexpected combo operand {x}"),
        }
    }
}

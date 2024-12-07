#[cfg(test)]
mod test;

use prelude::*;

pub struct Solution {
    equations: Vec<(i64, Vec<i64>)>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let equations = input
            .lines()
            .map(|line| {
                let Some((target, values)) = line.split_once(": ") else {
                    panic!("\": \" not found");
                };

                let target = target.parse().expect("bad integer");
                let values = values
                    .split_ascii_whitespace()
                    .map(|s| s.parse().expect("bad integer in values"))
                    .collect();

                (target, values)
            })
            .collect();

        Self { equations }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut total = 0;

        for (target, values) in &self.equations {
            if all_evaluations(values).contains(target) {
                total += target;
            }
        }

        Ok(total as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

fn all_evaluations<'a>(values: &'a [i64]) -> impl Iterator<Item = i64> + 'a {
    // use bits starting at the bottom (ones) to represent add (0) or multiply (1)
    if values.len() > 12 {
        // making more than 2^11 decisions is going to take too long
        panic!("too many values: {:?}", values);
    }

    (0..(1 << (values.len() - 1))).into_iter().map(|operators| {
        let mut accumulator: i64 = values[0];
        for (idx, &value) in values[1..].iter().enumerate() {
            if operators & (1 << idx) > 0 {
                accumulator *= value
            } else {
                accumulator += value
            }
        }

        accumulator
    })
}

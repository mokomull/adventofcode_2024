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

        'equation: for (target, values) in &self.equations {
            // use bits starting at the bottom (ones) to represent add (0) or multiply (1)
            if values.len() > 12 {
                // making more than 2^11 decisions is going to take too long
                anyhow::bail!("too many values: {}: {:?}", target, values);
            }

            dbg!((target, values));

            for operators in 0..(1 << (values.len())) {
                let mut accumulator = values[0];
                dbg!(operators);
                for (idx, &value) in values[1..].iter().enumerate() {
                    if operators & (1 << idx) > 0 {
                        eprintln!("multiplying by {}", value);
                        accumulator *= value
                    } else {
                        eprintln!("adding {}", value);
                        accumulator += value
                    }
                }

                dbg!(accumulator);

                if accumulator == *target {
                    eprintln!("found target with operators {operators}");
                    total += accumulator;
                    continue 'equation;
                }
            }

            eprintln!("did not find target at all...");
        }

        Ok(total as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

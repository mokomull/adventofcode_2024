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
        let mut total: i64 = 0;

        'equation: for (target, values) in &self.equations {
            // use bits starting at the bottom (ones) to represent add (0) or multiply (1)
            if values.len() > 12 {
                // making more than 2^11 decisions is going to take too long
                anyhow::bail!("too many values: {}: {:?}", target, values);
            }

            for operators in 0..(1 << (values.len() - 1)) {
                let mut accumulator = values[0];
                for (idx, &value) in values[1..].iter().enumerate() {
                    if operators & (1 << idx) > 0 {
                        accumulator *= value
                    } else {
                        accumulator += value
                    }
                }

                if accumulator == *target {
                    total += accumulator;
                    continue 'equation;
                }
            }
        }

        Ok(total as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut total: i64 = 0;

        'equation: for (target, values) in &self.equations {
            // use ternary starting at the bottom (ones) to represent add (0) or multiply (1) or concatenate (2)
            if values.len() > 12 {
                // making more than 3^11 decisions is going to take too long
                anyhow::bail!("too many values: {}: {:?}", target, values);
            }

            for mut operators in 0u64.. {
                let mut accumulator = values[0];
                for &value in &values[1..] {
                    match operators % 3 {
                        0 => accumulator += value,
                        1 => accumulator *= value,
                        2 => {
                            let mut concatenated = accumulator.to_string();
                            concatenated.push_str(&value.to_string());

                            accumulator = concatenated
                                .parse()
                                .expect("concatenation resulted in garbage");
                        }
                        _ => panic!("modulo three yielded something that wasn't 0 1 or 2"),
                    }
                    operators /= 3;
                }

                if operators > 0 {
                    // we went through all the values and still have a nonzero operator left, which
                    // means we've *previously* gone through all of the operators for the values
                    // that we have.  Stop.
                    break;
                }

                if accumulator == *target {
                    total += accumulator;
                    continue 'equation;
                }
            }
        }

        Ok(total as u64)
    }
}

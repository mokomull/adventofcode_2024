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

            for accumulator in all_options(values[0], &values[1..], &[|a, b| a + b, |a, b| a * b]) {
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

            for accumulator in all_options(
                values[0],
                &values[1..],
                &[
                    |a, b| a + b,
                    |a, b| a * b,
                    |a, b| {
                        let mut concatenated = a.to_string();
                        concatenated.push_str(&b.to_string());

                        concatenated
                            .parse()
                            .expect("concatenation resulted in garbage")
                    },
                ],
            ) {
                if accumulator == *target {
                    total += accumulator;
                    continue 'equation;
                }
            }
        }

        Ok(total as u64)
    }
}

fn all_options<'a>(
    first: i64,
    rest: &'a [i64],
    operators: &'a [fn(i64, i64) -> i64],
) -> Box<dyn Iterator<Item = i64> + 'a> {
    if rest.is_empty() {
        return Box::new(std::iter::once(first));
    }

    Box::new(
        operators
            .iter()
            .flat_map(move |operator| all_options(operator(first, rest[0]), &rest[1..], operators)),
    )
}

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

fn all_options<'a>(
    first: i64,
    rest: &'a [i64],
    operators: &'a [fn(i64, i64) -> i64],
) -> impl Iterator<Item = i64> + 'a {
    enum State {
        Done,
        One(i64),
        PassingDown(Box<dyn Iterator<Item = i64>>),
    }

    let mut state;
    if rest.is_empty() {
        state = State::One(first);
    } else {
        state = State::PassingDown(Box::new(std::iter::empty()));
    }

    let mut remaining_operators = operators;

    std::iter::from_fn(move || match state {
        State::Done => None,
        State::One(i) => {
            state = State::Done;
            Some(i)
        }
        State::PassingDown(mut it) => match it.next() {
            Some(i) => {
                state = State::PassingDown(it);
                Some(i)
            }
            None => {
                if operators.is_empty() {
                    state = State::Done;
                    return None;
                }

                let operator = remaining_operators[0];
                remaining_operators = &remaining_operators[1..];

                let mut new_it = all_options(operator(first, rest[0]), &rest[1..], operators);
                let res = new_it.next();
                state = State::PassingDown(Box::new(new_it));
                res
            }
        },
    })
}

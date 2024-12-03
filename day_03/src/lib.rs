use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    lines: Vec<String>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Self {
            lines: input.lines().map(str::to_owned).collect(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        Ok(self
            .lines
            .iter()
            .flat_map(|line| {
                mul_regex.captures_iter(line).map(|c| {
                    c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                        * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
                })
            })
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

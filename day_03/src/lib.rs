use log::debug;
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
        let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let do_regex = Regex::new(r"do\(\)").unwrap();
        let dont_regex = Regex::new(r"don't\(\)").unwrap();

        let data = self.lines.iter().join("\n");

        let dos = do_regex.find_iter(&data).map(|m| m.start());
        let donts = dont_regex.find_iter(&data).map(|m| m.end());

        let mut do_or_dont = dos
            .map(|i| (i, true))
            .chain(donts.map(|i| (i, false)))
            .collect_vec();
        do_or_dont.sort_unstable();

        let mut ranges = Vec::new();
        let mut last_start = 0;
        let mut last_do = true;
        for (pos, is_do) in do_or_dont {
            match (last_do, is_do) {
                // if we receive the same instruction, just ignore it.
                (true, true) | (false, false) => {}
                // was do, but now we should don't
                (true, false) => {
                    dbg!((last_start, pos));
                    dbg!(&data[last_start..pos]);
                    ranges.push(last_start..pos);
                }
                // was don't, but now we should do... record the start of this range
                (false, true) => {
                    last_start = pos;
                }
            }

            last_do = is_do;
        }

        // and if we ended with a `do`, also add that to ranges
        if last_do {
            ranges.push(last_start..data.len());
        }

        Ok(ranges
            .into_iter()
            .flat_map(|idxs| {
                mul_regex.captures_iter(&data[idxs]).map(|c| {
                    c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                        * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
                })
            })
            .sum())
    }
}

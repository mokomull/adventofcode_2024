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

        Ok(self
            .lines
            .iter()
            .flat_map(|line| {
                let dos = do_regex.find_iter(line).map(|m| m.start());
                let donts = dont_regex.find_iter(line).map(|m| m.start());

                let mut do_or_dont = dos
                    .map(|i| (i, true))
                    .chain(donts.map(|i| (i, false)))
                    .collect_vec();
                do_or_dont.sort_unstable();

                let mut ranges = Vec::new();
                let mut last_idx = 0;
                let mut last_do = true;
                for (pos, is_do) in do_or_dont {
                    match (last_do, is_do) {
                        // if we receive the same instruction, just ignore it.  don't even store the
                        // current index, just completely pretend this never happened.
                        (true, true) | (false, false) => {
                            continue;
                        }
                        // was do, but now we should don't
                        (true, false) => {
                            ranges.push(last_idx..pos);
                        }
                        // was don't, but now we should do... just fall through
                        (false, true) => {}
                    }

                    last_idx = pos;
                    last_do = is_do;
                }

                // and if we ended with a `do`, also add that to ranges
                if last_do {
                    ranges.push(last_idx..line.len());
                }

                ranges.into_iter().flat_map(|idxs| {
                    mul_regex.captures_iter(&line[idxs]).map(|c| {
                        c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                            * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
                    })
                })
            })
            .sum())
    }
}

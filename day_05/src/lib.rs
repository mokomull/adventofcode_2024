#[cfg(test)]
mod test;

use prelude::*;

struct Solution {
    rules: HashMap<usize, HashSet<usize>>,
    pages: Vec<Vec<usize>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let (before, after) = line.split_once('|').expect("no pipe found");
            let before = before.parse().expect("bad integer in before");
            let after = after.parse().expect("bad integer in after");

            rules.entry(before).or_default().insert(after);
        }

        let mut pages = Vec::new();
        for line in lines {
            pages.push(
                line.split(',')
                    .map(|s| s.parse().expect("bad integer in pages"))
                    .collect_vec(),
            );
        }

        Self { rules, pages }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

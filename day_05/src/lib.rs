#[cfg(test)]
mod test;

use prelude::*;

struct Solution {
    rules: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
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

        let mut updates = Vec::new();
        for line in lines {
            updates.push(
                line.split(',')
                    .map(|s| s.parse().expect("bad integer in pages"))
                    .collect_vec(),
            );
        }

        Self { rules, updates }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut total = 0;

        'update: for pages in &self.updates {
            for (i, page) in pages.iter().enumerate() {
                for earlier_page in &pages[..i] {
                    if let Some(must_be_after) = self.rules.get(page) {
                        if must_be_after.contains(earlier_page) {
                            // this update is out-of order
                            continue 'update;
                        }
                    }
                }
            }

            // we went through all the pages and there aren't any mis-ordered ones
            total += pages[pages.len() / 2];
        }

        Ok(total as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

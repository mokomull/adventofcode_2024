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
        let mut out_of_order = Vec::new();

        'update: for pages in &self.updates {
            for (i, page) in pages.iter().enumerate() {
                for earlier_page in &pages[..i] {
                    if let Some(must_be_after) = self.rules.get(page) {
                        if must_be_after.contains(earlier_page) {
                            // this update is out-of order
                            out_of_order.push(pages);
                            continue 'update;
                        }
                    }
                }
            }
        }

        let mut total = 0;

        for update in out_of_order {
            let mut update = update.clone();

            'again: loop {
                for i in 0..update.len() {
                    for earlier_page in 0..i {
                        if let Some(must_be_after) = self.rules.get(&update[i]) {
                            if must_be_after.contains(&update[earlier_page]) {
                                // naive but ... can we get away with just swapping them?
                                update.swap(i, earlier_page);
                                continue 'again;
                            }
                        }
                    }
                }

                // if we didn't find anything out of order, then we're all set!
                total += update[update.len() / 2];
                break;
            }
        }

        Ok(total as u64)
    }
}

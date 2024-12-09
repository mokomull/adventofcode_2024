use anyhow::anyhow;
use prelude::*;

#[cfg(test)]
mod test;

struct Solution {
    disk: Vec<Option<i32>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut bytes = input.bytes();
        let mut disk = Vec::new();

        let mut file_id = 0;
        loop {
            let Some(c) = bytes.next() else {
                break;
            };
            disk.append(&mut vec![Some(file_id); (c - b'0') as usize]);
            file_id += 1;

            let Some(c) = bytes.next() else {
                break;
            };
            disk.append(&mut vec![None; (c - b'0') as usize]);
        }

        Self { disk }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut disk = self.disk.clone();

        let mut free_idx = disk
            .iter()
            .position(Option::is_none)
            .ok_or_else(|| anyhow!("there are somehow no free blocks anywhere"))?;

        // find the first file block to move
        let mut file_idx = disk.len() - 1;
        while disk[file_idx].is_none() {
            file_idx -= 1;
        }

        while file_idx > free_idx {
            // move the block
            disk.swap(file_idx, free_idx);

            // find the next free block
            while disk[free_idx].is_some() {
                free_idx += 1;
            }

            // and find the next file block to move
            while disk[file_idx].is_none() {
                file_idx -= 1;
            }
        }

        Ok(disk
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(i, file_id)| i as u64 * file_id as u64)
            .product())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

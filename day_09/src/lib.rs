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
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut disk = self.disk.clone();

        // find the first file block to move
        let mut file_end = disk.len() - 1;
        while disk[file_end].is_none() {
            file_end -= 1;
        }
        // find the start of that file
        let mut file_start = file_end;
        while disk[file_start] == disk[file_end] {
            file_start -= 1;
        }
        file_start += 1;

        loop {
            let length = file_end - file_start + 1;
            let file_id = disk[file_end].expect("file_end should never point to free space");

            let mut free_start = 0;
            let mut free = None;
            // find the leftmost suitable spot
            'find_free: loop {
                if disk[free_start].is_some() {
                    free_start += 1;
                    continue;
                }

                for i in 0..length {
                    if free_start + i > file_start {
                        break 'find_free;
                    }

                    if disk[free_start + i].is_some() {
                        free_start += i + 1;
                        continue 'find_free;
                    }
                }

                free = Some(free_start..(free_start + length));
                break;
            }

            // if we found a spot, move the block
            if let Some(free) = free {
                let (left, right) = disk.split_at_mut(file_start);
                (&mut left[free]).swap_with_slice(&mut right[..length]);
            }

            // and find the next file block to move
            if file_id == 0 {
                // we're done once we've dispatched file 0... which, trivially, won't move.
                break;
            }

            while disk[file_end] != Some(file_id - 1) {
                // the next file we're moving is the one with the next-lower *id*.  We may have
                // moved other stuff between this file and the file we seek.
                file_end -= 1;
            }
            // find the start of that file
            file_start = file_end;
            while disk[file_start] == disk[file_end] {
                if file_start == 0 {
                    // don't try to read disk[-1] (which is probably undefined...)
                    break;
                }
                file_start -= 1;
            }
            if file_start > 0 || disk[file_start] != disk[file_end] {
                // correct for having searched just beyond the beginning of the file.
                file_start += 1;
            }
        }

        Ok(disk
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(i, file_id)| i as u64 * file_id as u64)
            .sum())
    }
}

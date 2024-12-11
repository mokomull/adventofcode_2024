use std::collections::BTreeSet;

use anyhow::anyhow;
use prelude::*;

#[cfg(test)]
mod test;

struct Solution {
    disk: Vec<Option<i32>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut bytes = input.trim().bytes();
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
            .enumerate()
            .filter_map(|(i, file_id)| file_id.map(|file_id| i as u64 * file_id as u64))
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut disk = self.disk.clone();

        // find free runs.  tuples of (start, end) which DO NOT include the end point because we can
        // search this left-to-right
        let mut free_blocks = BTreeSet::new();
        let mut last_seen_free = false;
        let mut free_start = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..disk.len() {
            match (last_seen_free, disk[i].is_none()) {
                (true, true) | (false, false) => {
                    // nothing to do here.
                }
                (true, false) => {
                    free_blocks.insert((free_start, i));
                    last_seen_free = false;
                }
                (false, true) => {
                    last_seen_free = true;
                    free_start = i;
                }
            }
        }
        if last_seen_free {
            free_blocks.insert((free_start, disk.len()));
        }

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
        // file_start and file_end are *inclusive* because we have to find this from
        // right-to-left...

        loop {
            let length = file_end - file_start + 1;
            let file_id = disk[file_end].expect("file_end should never point to free space");

            let mut free = None;
            // find the leftmost suitable spot
            for &(start, end) in free_blocks.iter() {
                if start >= file_start {
                    // nowhere to put it
                    break;
                }

                if end - start >= length {
                    free = Some((start, end));
                    break;
                }
            }

            // if we found a spot, move the block and account for our free space
            if let Some((start, end)) = free {
                let (left, right) = disk.split_at_mut(file_start);
                left[start..(start + length)].swap_with_slice(&mut right[..length]);

                if !free_blocks.remove(&(start, end)) {
                    panic!("we found free blocks that didn't exist!?");
                }
                let start = start + length;
                if start < end {
                    // if we didn't use it up, put it back in the pool
                    free_blocks.insert((start, end));
                }
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
            .enumerate()
            .filter_map(|(i, file_id)| file_id.map(|file_id| i as u64 * file_id as u64))
            .sum())
    }
}

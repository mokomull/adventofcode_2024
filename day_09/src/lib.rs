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
        todo!()
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

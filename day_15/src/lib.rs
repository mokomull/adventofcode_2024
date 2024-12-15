use core::str;
use std::collections::VecDeque;

use prelude::*;

#[cfg(test)]
mod test;

pub struct Solution {
    map: Vec<Vec<u8>>,
    moves: Vec<u8>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let mut map = Vec::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            map.push(line.into());
        }
        let mut moves = Vec::new();
        for line in lines {
            moves.extend_from_slice(line.trim().as_bytes());
        }

        Self { map, moves }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut map: Vec<Vec<u8>> = self.map.clone();
        let mut robot = self
            .map
            .iter()
            .enumerate()
            .filter_map(|(i, line)| line.iter().position(|&c| c == b'@').map(|j| (i, j)))
            .next()
            .unwrap();

        for &m in &self.moves {
            let f = match m {
                b'<' => |(i, j)| (i, j - 1),
                b'>' => |(i, j)| (i, j + 1),
                b'^' => |(i, j)| (i - 1, j),
                b'v' => |(i, j)| (i + 1, j),
                x => panic!("unexpected direction {:?}", x),
            };
            robot = push(f, &mut map, robot);
        }

        Ok(map
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter().enumerate().flat_map(
                    move |(j, c)| {
                        if c == b'O' {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
            })
            .map(|(i, j)| 100 * i + j)
            .sum::<usize>() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut map: Vec<Vec<u8>> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|&c| -> [u8; 2] {
                        match c {
                            b'#' => *b"##",
                            b'O' => *b"[]",
                            b'.' => *b"..",
                            b'@' => *b"@.",
                            x => panic!("unexpected map tile {}", x),
                        }
                    })
                    .collect()
            })
            .collect();

        let mut robot = map
            .iter()
            .enumerate()
            .filter_map(|(i, line)| line.iter().position(|&c| c == b'@').map(|j| (i, j)))
            .next()
            .unwrap();

        for &m in &self.moves {
            let f = match m {
                b'<' => |(i, j)| (i, j - 1),
                b'>' => |(i, j)| (i, j + 1),
                b'^' => |(i, j)| (i - 1, j),
                b'v' => |(i, j)| (i + 1, j),
                x => panic!("unexpected direction {:?}", x),
            };
            robot = wide_push(f, &mut map, robot);
        }

        Ok(map
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter().enumerate().flat_map(
                    move |(j, c)| {
                        if c == b'[' {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
            })
            .map(|(i, j)| 100 * i + j)
            .sum::<usize>() as u64)
    }
}

// Returns the next position of the robot, which might be the same as we started, or it might have moved.
#[allow(clippy::ptr_arg)]
fn push<F>(step: F, map: &mut Vec<Vec<u8>>, robot: (usize, usize)) -> (usize, usize)
where
    F: Fn((usize, usize)) -> (usize, usize),
{
    let (mut i, mut j) = robot;
    loop {
        (i, j) = step((i, j));
        match map[i][j] {
            b'O' => {
                // still a stone so keep looking
            }
            b'#' => {
                // a wall, so we can't move anything
                return robot;
            }
            x => {
                // this must be an empty space
                assert_eq!(b'.', x);
                break;
            }
        }
    }

    // stone grew one-step-past-the-last-stone-we-saw
    map[i][j] = b'O';
    // and the robot moves one step
    let (new_robot_i, new_robot_j) = step(robot);
    map[robot.0][robot.1] = b'.';
    map[new_robot_i][new_robot_j] = b'@';

    (new_robot_i, new_robot_j)
}

// Returns the next position of the robot.  This is very similar to the above solution, except we
// keep a list of work to do since one visit can create two squares to look at.
#[allow(clippy::ptr_arg)]
fn wide_push<F>(step: F, map: &mut Vec<Vec<u8>>, robot: (usize, usize)) -> (usize, usize)
where
    F: Fn((usize, usize)) -> (usize, usize),
{
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visit_order: Vec<(usize, usize)> = Vec::new();
    to_visit.push_back(robot);
    while let Some((i, j)) = to_visit.pop_front() {
        if !visited.insert((i, j)) {
            continue;
        }
        visit_order.push((i, j));

        let (i, j) = step((i, j));
        match map[i][j] {
            b'[' => {
                // still a stone so keep looking
                to_visit.push_back((i, j));
                to_visit.push_back((i, j + 1));
            }
            b']' => {
                // other side of a stone
                to_visit.push_back((i, j));
                to_visit.push_back((i, j - 1));
            }
            b'#' => {
                // a wall, so we can't move anything
                return robot;
            }
            x => {
                // this must be an empty space
                assert_eq!(b'.', x);
            }
        }
    }

    // everything that is in `visited` is a stone or the robot, and should be moved.  Do this in the
    // reverse-order in which we discovered things so that we can always replace the "from" with a
    // '.', which will get overwritten if something else gets pushed into this square
    let old_map = map.clone();
    for (i, j) in visit_order.into_iter().rev() {
        let (new_i, new_j) = step((i, j));
        map[new_i][new_j] = old_map[i][j];
        map[i][j] = b'.';
    }
    // and we just copied over the robot rather than overwriting it, so clear it out now
    map[robot.0][robot.1] = b'.';

    step(robot)
}

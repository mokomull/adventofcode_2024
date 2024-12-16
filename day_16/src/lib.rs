use anyhow::anyhow;
use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
};
use prelude::*;

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    East,
    West,
    North,
    South,
}
use Direction::*;

struct Solution {
    map: Vec<Vec<u8>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input.lines().map(|line| line.to_owned().into()).collect(),
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut start = None;
        let mut end = None;

        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                match c {
                    b'S' => start = Some((i, j)),
                    b'E' => end = Some((i, j)),
                    _ => {}
                }
            }
        }

        let start = start.ok_or_else(|| anyhow!("no start found"))?;
        let end = end.ok_or_else(|| anyhow!("no end found"))?;

        let mut graph = Graph::from(&self.map);
        let start = graph.get_or_create_node((start.0, start.1, East));

        let distances = dijkstra(&graph.graph, start, None, |e| -> u64 { *e.weight() });

        Ok([
            graph.get_or_create_node((end.0, end.1, East)),
            graph.get_or_create_node((end.0, end.1, West)),
            graph.get_or_create_node((end.0, end.1, North)),
            graph.get_or_create_node((end.0, end.1, South)),
        ]
        .into_iter()
        .flat_map(|n| distances.get(&n).cloned())
        .min()
        .unwrap())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

#[derive(Default)]
struct Graph {
    graph: DiGraph<(), u64>,
    node_indexes: HashMap<(usize, usize, Direction), NodeIndex>,
}

impl Graph {
    fn from(map: &[Vec<u8>]) -> Self {
        let mut ret = Self::default();

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == b'#' {
                    // you can't go through walls
                    continue;
                }

                if j > 0 {
                    // we could go west
                    let west = ret.get_or_create_node((i, j - 1, West));
                    let no_turn = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(no_turn, west, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(one_turn_a, west, 1000);
                    let one_turn_b = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(one_turn_b, west, 1000);
                    let two_turns = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(two_turns, west, 2000);
                }

                if i > 0 {
                    // we could go north
                    let north = ret.get_or_create_node((i - 1, j, North));
                    let no_turn = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(no_turn, north, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(one_turn_a, north, 1000);
                    let one_turn_b = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(one_turn_b, north, 1000);
                    let two_turns = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(two_turns, north, 2000);
                }

                // We can always go east and south without underflow -- if they're beyond the grid,
                // we simply won't have any paths back out of there.
                {
                    let east = ret.get_or_create_node((i, j + 1, East));
                    let no_turns = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(no_turns, east, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(one_turn_a, east, 1000);
                    let one_turn_b = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(one_turn_b, east, 1000);
                    let two_turns = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(two_turns, east, 2000);
                }
                {
                    let south = ret.get_or_create_node((i + 1, j, South));
                    let no_turns = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(no_turns, south, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(one_turn_a, south, 1000);
                    let one_turn_b = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(one_turn_b, south, 1000);
                    let two_turns = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(two_turns, south, 2000);
                }
            }
        }

        ret
    }

    fn get_or_create_node(&mut self, name: (usize, usize, Direction)) -> NodeIndex {
        *self
            .node_indexes
            .entry(name)
            .or_insert_with(|| self.graph.add_node(()))
    }
}

use anyhow::anyhow;
use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
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

        let end_nodes = [
            graph.get_or_create_node((end.0, end.1, East)),
            graph.get_or_create_node((end.0, end.1, West)),
            graph.get_or_create_node((end.0, end.1, North)),
            graph.get_or_create_node((end.0, end.1, South)),
        ];
        let target = end_nodes
            .into_iter()
            .flat_map(|n| distances.get(&n).cloned())
            .min()
            .unwrap();

        struct Visitation {
            path: Vec<NodeIndex>,
            total_cost: u64,
        }
        let mut visited = HashSet::new();
        let mut to_visit = vec![Visitation {
            path: vec![start],
            total_cost: 0,
        }];
        while let Some(v) = to_visit.pop() {
            if v.total_cost > target {
                // we've already gone too far
                continue;
            }

            let this = v.path.last().unwrap();

            if end_nodes.contains(this) {
                // we found the end, and we must have met our target distance, so mark the whole
                // path as "on a shortest path".  Going any further will add more cost, so also
                // don't bother.
                for node in v.path {
                    visited.insert(node);
                }
                continue;
            }

            for e in graph.graph.edges(*this) {
                let mut path = v.path.clone();
                path.push(e.target());
                to_visit.push(Visitation {
                    path,
                    total_cost: v.total_cost + e.weight(),
                });
            }
        }

        Ok(graph
            .node_indexes
            .into_iter()
            .filter(|(_name, idx)| visited.contains(&idx))
            .map(|((i, j, _dir), _idx)| (i, j))
            .collect::<HashSet<_>>()
            .len() as u64)
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
                    ret.graph.add_edge(one_turn_a, west, 1001);
                    let one_turn_b = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(one_turn_b, west, 1001);
                    let two_turns = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(two_turns, west, 2001);
                }

                if i > 0 {
                    // we could go north
                    let north = ret.get_or_create_node((i - 1, j, North));
                    let no_turn = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(no_turn, north, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(one_turn_a, north, 1001);
                    let one_turn_b = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(one_turn_b, north, 1001);
                    let two_turns = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(two_turns, north, 2001);
                }

                // We can always go east and south without underflow -- if they're beyond the grid,
                // we simply won't have any paths back out of there.
                {
                    let east = ret.get_or_create_node((i, j + 1, East));
                    let no_turns = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(no_turns, east, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(one_turn_a, east, 1001);
                    let one_turn_b = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(one_turn_b, east, 1001);
                    let two_turns = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(two_turns, east, 2001);
                }
                {
                    let south = ret.get_or_create_node((i + 1, j, South));
                    let no_turns = ret.get_or_create_node((i, j, South));
                    ret.graph.add_edge(no_turns, south, 1);
                    let one_turn_a = ret.get_or_create_node((i, j, East));
                    ret.graph.add_edge(one_turn_a, south, 1001);
                    let one_turn_b = ret.get_or_create_node((i, j, West));
                    ret.graph.add_edge(one_turn_b, south, 1001);
                    let two_turns = ret.get_or_create_node((i, j, North));
                    ret.graph.add_edge(two_turns, south, 2001);
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

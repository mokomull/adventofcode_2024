#[cfg(test)]
mod test;

use petgraph::{
    algo::{all_simple_paths, dijkstra},
    visit::{
        Data, GraphBase, GraphProp, IntoEdgeReferences, IntoEdges, IntoNeighbors,
        IntoNeighborsDirected, IntoNodeIdentifiers, NodeCompactIndexable, NodeCount, NodeIndexable,
        Visitable,
    },
    Directed,
    Direction::{self, Incoming, Outgoing},
};
use prelude::*;

struct Solution {
    map: Vec<Vec<u8>>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect();
        Self { map }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut possible_trailheads = Vec::new();
        let mut peaks = Vec::new();

        for (i, row) in self.map.iter().enumerate() {
            for (j, &height) in row.iter().enumerate() {
                if height == 0 {
                    possible_trailheads.push((i, j));
                }
                if height == 9 {
                    peaks.push((i, j));
                }
            }
        }

        possible_trailheads.sort_unstable();

        Ok(possible_trailheads
            .into_iter()
            .map(|trailhead| {
                let reachable = dijkstra(&self, trailhead, None, |_| 1u32);
                let count = peaks
                    .iter()
                    .filter(|&peak| match reachable.get(peak) {
                        None | Some(&u32::MAX) => false,
                        _ => true,
                    })
                    .count();
                eprintln!("peak at {:?} has score {}", trailhead, count);
                count
            })
            .sum::<usize>() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut possible_trailheads = Vec::new();
        let mut peaks = Vec::new();

        for (i, row) in self.map.iter().enumerate() {
            for (j, &height) in row.iter().enumerate() {
                if height == 0 {
                    possible_trailheads.push((i, j));
                }
                if height == 9 {
                    peaks.push((i, j));
                }
            }
        }

        Ok(possible_trailheads
            .into_iter()
            .cartesian_product(&peaks)
            .map(|(trailhead, peak)| {
                all_simple_paths::<Vec<_>, _>(&self, trailhead, *peak, 0, None).count() as u64
            })
            .sum())
    }
}

impl GraphBase for Solution {
    type EdgeId = ((usize, usize), (usize, usize));
    type NodeId = (usize, usize);
}

impl Data for Solution {
    type NodeWeight = ();
    type EdgeWeight = ();
}

impl GraphProp for Solution {
    type EdgeType = Directed;
}

impl Visitable for Solution {
    type Map = HashSet<Self::NodeId>;

    fn visit_map(self: &Self) -> Self::Map {
        HashSet::new()
    }

    fn reset_map(self: &Self, map: &mut Self::Map) {
        map.clear();
    }
}

impl<'a> IntoNodeIdentifiers for &'a Solution {
    type NodeIdentifiers = Box<dyn Iterator<Item = (usize, usize)> + 'a>;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        Box::new((0..self.map.len()).flat_map(|i| (0..self.map[i].len()).map(move |j| (i, j))))
    }
}

impl NodeCount for Solution {
    fn node_count(&self) -> usize {
        self.map.len() * self.map[0].len()
    }
}

impl NodeIndexable for Solution {
    fn node_bound(&self) -> usize {
        self.node_count()
    }

    fn to_index(&self, (i, j): Self::NodeId) -> usize {
        i * self.map[0].len() + j
    }

    #[doc = r" Convert `i` to a node index. `i` must be a valid value in the graph."]
    fn from_index(&self, x: usize) -> Self::NodeId {
        (x / self.map[0].len(), x % self.map[0].len())
    }
}

impl NodeCompactIndexable for Solution {}

impl<'a> IntoEdgeReferences for &'a Solution {
    type EdgeRef = ((usize, usize), (usize, usize), &'static ());

    type EdgeReferences = Box<dyn Iterator<Item = Self::EdgeRef> + 'a>;

    fn edge_references(self) -> Self::EdgeReferences {
        Box::new(
            self.node_identifiers()
                .flat_map(|(i, j)| self.edges((i, j))),
        )
    }
}

impl<'a> IntoEdges for &'a Solution {
    type Edges = Box<dyn Iterator<Item = Self::EdgeRef> + 'a>;

    fn edges(self, (i, j): Self::NodeId) -> Self::Edges {
        Box::new(self.neighbors((i, j)).map(move |next| ((i, j), next, &())))
    }
}

impl<'a> IntoNeighbors for &'a Solution {
    type Neighbors = std::vec::IntoIter<Self::NodeId>;

    fn neighbors(self, n: Self::NodeId) -> Self::Neighbors {
        self.neighbors_directed(n, Outgoing)
    }
}

impl<'a> IntoNeighborsDirected for &'a Solution {
    type NeighborsDirected = std::vec::IntoIter<Self::NodeId>;

    fn neighbors_directed(self, (i, j): Self::NodeId, d: Direction) -> Self::NeighborsDirected {
        let target = match (self.map[i][j], d) {
            (0, Incoming) | (9, Outgoing) => return Vec::new().into_iter(),
            (_, Incoming) => self.map[i][j] - 1,
            (_, Outgoing) => self.map[i][j] + 1,
        };

        let mut neighbors = Vec::new();
        if i > 0 && self.map[i - 1][j] == target {
            neighbors.push((i - 1, j));
        }
        if j > 0 && self.map[i][j - 1] == target {
            neighbors.push((i, j - 1));
        }
        if i < self.map.len() - 1 && self.map[i + 1][j] == target {
            neighbors.push((i + 1, j));
        }
        if j < self.map[i].len() - 1 && self.map[i][j + 1] == target {
            neighbors.push((i, j + 1));
        }

        neighbors.into_iter()
    }
}

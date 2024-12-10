#[cfg(test)]
mod test;

use petgraph::{
    algo::floyd_warshall,
    visit::{
        Data, GraphBase, GraphProp, IntoEdgeReferences, IntoNodeIdentifiers, NodeCompactIndexable,
        NodeCount, NodeIndexable,
    },
    Directed,
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
        let all_pairs =
            floyd_warshall(&self, |_| 1_u32).expect("no negative weights in the first place");

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
                let count = peaks
                    .iter()
                    .filter(|&&peak| match all_pairs.get(&(trailhead, peak)) {
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
        todo!()
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
        Box::new(self.node_identifiers().flat_map(|(i, j)| {
            let mut edges = vec![];
            let one_step_up = self.map[i][j] + 1;
            if i > 0 && self.map[i - 1][j] == one_step_up {
                edges.push(((i, j), (i - 1, j), &()));
            }
            if j > 0 && self.map[i][j - 1] == one_step_up {
                edges.push(((i, j), (i, j - 1), &()));
            }
            if i < self.map.len() - 1 && self.map[i + 1][j] == one_step_up {
                edges.push(((i, j), (i + 1, j), &()));
            }
            if j < self.map[i].len() - 1 && self.map[i][j + 1] == one_step_up {
                edges.push(((i, j), (i, j + 1), &()));
            }

            edges.into_iter()
        }))
    }
}

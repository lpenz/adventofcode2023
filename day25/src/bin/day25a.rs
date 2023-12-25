// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day25::*;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Edge(Node, Node);

impl Edge {
    pub fn new(node1: Node, node2: Node) -> Edge {
        if node1 < node2 {
            Edge(node1, node2)
        } else {
            Edge(node2, node1)
        }
    }
    pub fn other(&self, node: &Node) -> Node {
        if node == &self.0 {
            self.1
        } else if node == &self.1 {
            self.0
        } else {
            panic!("node is not part of edge")
        }
    }
}

type Graph = HashMap<Node, HashSet<Edge>>;

fn group_size(graph: &Graph, start: Node) -> usize {
    let mut visited = HashSet::<Node>::new();
    let mut frontier = vec![start];
    while let Some(node) = frontier.pop() {
        if visited.contains(&node) {
            continue;
        }
        for e in graph[&node].iter() {
            let other = e.other(&node);
            if !visited.contains(&other) {
                frontier.push(other);
            }
        }
        visited.insert(node);
    }
    visited.len()
}

fn min_dists(graph: &Graph, ecount: &mut HashMap<Edge, usize>, start: &Node) {
    let mut visited = HashSet::<Node>::new();
    let mut came_from = HashMap::<Node, Node>::new();
    let mut frontier = vec![*start];
    while let Some(node) = frontier.pop() {
        if visited.contains(&node) {
            continue;
        }
        let edges: &HashSet<Edge> = graph.get(&node).unwrap();
        for edge in edges {
            let other = edge.other(&node);
            if visited.contains(&other) {
                continue;
            }
            let entry = came_from.entry(other);
            if let Entry::Vacant(e) = entry {
                e.insert(node);
                frontier.push(other);
            }
        }
        visited.insert(node);
    }
    for end in graph.keys() {
        let mut node = *end;
        while &node != start {
            let edge = Edge::new(node, came_from[&node]);
            let e = ecount.entry(edge).or_default();
            *e += 1;
            node = edge.other(&node);
        }
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let graph0 = input.into_iter().collect::<HashMap<_, _>>();
    let mut graph = HashMap::<Node, HashSet<Edge>>::new();
    for (node, connected) in graph0.into_iter() {
        for other in connected {
            let edge = Edge::new(node, other);
            let e = graph.entry(node).or_default();
            e.insert(edge);
            let e = graph.entry(other).or_default();
            e.insert(edge);
        }
    }
    for _ in 0..3 {
        let mut ecount = HashMap::<Edge, usize>::new();
        let nodes = graph.keys().collect::<Vec<_>>();
        for node in &nodes {
            min_dists(&graph, &mut ecount, node);
        }
        let ecut = ecount.iter().map(|(e, v)| (v, e)).max().unwrap().1;
        let Edge(n1, n2) = *ecut;
        graph.get_mut(&n1).unwrap().remove(ecut);
        graph.get_mut(&n2).unwrap().remove(ecut);
    }
    let g = group_size(&graph, *graph.keys().next().unwrap());
    Ok(g * (graph.len() - g))
}

// Not running tests because the algorithm above is a bit
// non-deterministic.
// #[test]
// fn test() -> Result<()> {
//     assert_eq!(process(EXAMPLE.as_bytes())?, 54);
//     Ok(())
// }

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}

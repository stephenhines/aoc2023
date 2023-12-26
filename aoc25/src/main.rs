use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

struct MyGraph {
    node_map: HashMap<String, NodeIndex>,
    graph: UnGraph<String, usize>,
}

impl MyGraph {
    fn new() -> Self {
        let graph = UnGraph::new_undirected();
        MyGraph {
            node_map: HashMap::new(),
            graph,
        }
    }

    fn get_node(&mut self, name: &str) -> NodeIndex {
        if self.node_map.contains_key(name) {
            return *self.node_map.get(name).unwrap();
        }
        let idx = self.graph.add_node(name.to_string());
        self.node_map.insert(name.to_string(), idx);
        idx
    }
}

fn compute_sizes(lines: &[String]) -> usize {
    let mut graph = MyGraph::new();

    for line in lines {
        let toks: Vec<_> = line.split(": ").collect();
        let rtoks: Vec<_> = toks[1].split_whitespace().collect();

        let lhs = toks[0];
        let l = graph.get_node(lhs);
        for rhs in rtoks {
            let r = graph.get_node(rhs);
            graph.graph.add_edge(l, r, 1);
        }
    }

    let node_count = graph.graph.node_count();

    let min_cut_res: Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph.graph, |_| Ok(1));
    //dbg!(graph.graph);

    let (min_cut, partition) = min_cut_res.unwrap().unwrap();

    //println!("min_cut: {:?} partition: {:?}", min_cut, partition);

    assert_eq!(min_cut, 3);
    let left = partition.len();
    let right = node_count - left;
    let sizes = left * right;
    println!("sizes: {:?}", sizes);
    sizes
}

#[test]
fn test_prelim() {
    let sizes = compute_sizes(&get_input("prelim.txt"));
    assert_eq!(sizes, 54);
}

#[test]
fn test_part1() {
    let sizes = compute_sizes(&get_input("input.txt"));
    assert_eq!(sizes, 601344);
}

fn main() {
    compute_sizes(&get_input("prelim.txt"));
    compute_sizes(&get_input("input.txt"));
}

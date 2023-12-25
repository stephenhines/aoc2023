use combinations::Combinations;
use std::collections::HashMap;
use std::collections::HashSet;
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

type VertexId = usize;
type Edge = (VertexId, VertexId);

#[derive(Debug)]
struct Vertex {
    id: VertexId,
    edges: Vec<usize>,
}

impl Vertex {
    fn new(id: VertexId) -> Self {
        Vertex {
            id,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, id: VertexId) {
        self.edges.push(id);
    }

    fn degree(&self) -> usize {
        self.edges.len()
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Vertex>,
    node_map: HashMap<String, VertexId>,
    edges: Vec<Edge>,
}

impl Graph {
    fn get_or_create_vertex(&mut self, name: &str) -> VertexId {
        let name_str = String::from(name);
        if self.node_map.contains_key(&name_str) {
            return *self.node_map.get(&name_str).unwrap();
        }
        let id = self.nodes.len();
        self.nodes.push(Vertex::new(id));
        self.node_map.insert(name_str, id);

        id
    }

    fn get_vertex(&self, name: &str) -> VertexId {
        let name_str = String::from(name);
        if self.node_map.contains_key(&name_str) {
            return *self.node_map.get(&name_str).unwrap();
        }
        panic!("Can't find node: {}", name);
    }

    fn add_edge(&mut self, left: &str, right: &str) {
        let left_id = self.get_or_create_vertex(left);
        let right_id = self.get_or_create_vertex(right);
        self.nodes[left_id].add_edge(right_id);
        self.nodes[right_id].add_edge(left_id);
        self.edges.push((left_id, right_id));
    }

    fn find_cut_ignoring_three(
        &self,
        ignore_list: HashSet<Edge>,
    ) -> (HashSet<VertexId>, HashSet<VertexId>) {
        let mut left: HashSet<usize> = HashSet::new();
        let mut right: HashSet<usize> = HashSet::new();

        // Put everything in the right side
        for i in 0..self.nodes.len() {
            right.insert(i);
        }

        let mut worklist: Vec<VertexId> = Vec::new();
        worklist.push(0);

        loop {
            let id = worklist.pop().unwrap();
            // Only handle things if we haven't moved this to the left yet.
            if right.contains(&id) {
                left.insert(id);
                right.remove(&id);
                for edge in &self.nodes[id].edges {
                    if ignore_list.contains(&(id, *edge)) || ignore_list.contains(&(*edge, id)) {
                        // Ignore this edge
                    } else {
                        worklist.push(*edge);
                    }
                }
            }
            if worklist.is_empty() {
                // We probably should verify that right is connected too, but I'm going to be lazy.
                return (left, right);
            }
        }
    }

    fn find_cut(&self) -> (HashSet<VertexId>, HashSet<VertexId>) {
        let mut edge_list = Vec::new();
        self.edges.iter().for_each(|x| edge_list.push(x));
        println! {"Before combo: {}", edge_list.len()};
        let combo = Combinations::new(edge_list, 3);
        println! {"After combo"};
        //println!("Combo: {:?}", combo);
        //let combo_len = combo.len();
        let combo_len = 7000000000u64;
        let mut i = 0;
        for c in combo {
            let mut ignore_list: HashSet<Edge> = HashSet::new();
            for (v1, n2) in c {
                ignore_list.insert((*v1, *n2));
            }

            i += 1;
            println! {"Try {} of {}: {}%", i, combo_len, i as f64 / combo_len as f64};

            //ignore_list.insert((self.get_node_const("hfx"), self.get_node_const("pzl")));
            //ignore_list.insert((self.get_node_const("bvb"), self.get_node_const("cmg")));
            //ignore_list.insert((self.get_node_const("nvd"), self.get_node_const("jqt")));
            let (left, right) = self.find_cut_ignoring_three(ignore_list);
            if right.len() > 0 {
                return (left, right);
            }
        }

        (HashSet::new(), HashSet::new())
    }
}

fn compute_sizes(lines: &[String]) -> usize {
    let mut graph: Graph = Default::default();
    for line in lines {
        let toks: Vec<_> = line.split(": ").collect();
        let rtoks: Vec<_> = toks[1].split_whitespace().collect();

        let lhs = toks[0];
        rtoks.iter().for_each(|rhs| graph.add_edge(lhs, rhs));
    }

    let (left, right) = graph.find_cut();
    let sizes = left.len() * right.len();
    //println!{"ids: {:?}", graph_arena};
    println! {"sizes: {:?}", sizes};

    sizes
}

#[test]
fn test_prelim() {
    let sizes = compute_sizes(&get_input("prelim.txt"));
    assert_eq!(sizes, 54);
}

fn main() {
    compute_sizes(&get_input("prelim.txt"));
    //compute_sizes(&get_input("input.txt"));
}

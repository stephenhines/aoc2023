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

type Node = [char; 3];

trait ToNode {
    fn to_node(&self) -> Node;
}

impl ToNode for str {
    fn to_node(&self) -> Node {
        self.chars().collect::<Vec<_>>().try_into().unwrap()
    }
}

struct Network {
    directions: Vec<bool>,
    netmap: HashMap<Node, [Node; 2]>,
}

impl Network {
    fn new(lines: &[String]) -> Self {
        let mut directions = Vec::new();
        let line = lines.first().unwrap();
        for i in line.chars() {
            if i == 'L' {
                directions.push(true);
            } else {
                directions.push(false);
            }
        }

        let mut netmap = HashMap::new();
        for line in &lines[2..] {
            let toks: Vec<&str> = line.split("=").collect();
            let sym = toks[0].trim();
            let node: Node = sym.to_node();

            let dir_toks: Vec<&str> = toks[1].split(",").collect();
            let left_tok = dir_toks[0].split("(").collect::<Vec<_>>()[1]
                .split(",")
                .collect::<Vec<_>>();
            let left = left_tok[0].to_node();

            let right_tok = dir_toks[1].split_whitespace().collect::<Vec<_>>()[0]
                .split(")")
                .collect::<Vec<_>>();
            let right = right_tok[0].to_node();

            let leftright = [left, right];
            netmap.insert(node, leftright);
        }
        //println! {"directions {:?}", directions};
        //println! {"netmap {:?}", netmap};
        Self { directions, netmap }
    }

    fn next_step(&self, dir_ctr: usize) -> bool {
        self.directions[dir_ctr]
    }
}

fn compute_steps(lines: &[String]) -> u64 {
    let network = Network::new(lines);

    let mut steps = 0;
    let mut dir_ctr = 0;
    let start: Node = "AAA".to_node();
    let stop: Node = "ZZZ".to_node();
    let mut cur_node = start;
    while cur_node != stop {
        steps += 1;
        let left = network.next_step(dir_ctr);
        if dir_ctr == network.directions.len() - 1 {
            dir_ctr = 0;
        } else {
            dir_ctr += 1;
        }
        //println!{"Visiting {:?} {}", cur_node, if left { "Left" } else { "Right" }};
        //println!{"{:?}", network.netmap[&cur_node]};
        if left {
            cur_node = network.netmap[&cur_node][0];
        } else {
            cur_node = network.netmap[&cur_node][1];
        }
    }

    println! {"Steps: {}", steps};
    steps
}

fn ghost_finished(nodes: &Vec<&Node>) -> bool {
    for node in nodes {
        if node[2] != 'Z' {
            return false;
        }
    }
    true
}

fn compute_ghost_steps(lines: &[String]) -> u64 {
    let network = Network::new(lines);

    let mut steps = 0;
    let mut dir_ctr = 0;
    let mut nodes = Vec::new();
    network.netmap.keys().for_each(|k| {
        if k[2] == 'A' {
            nodes.push(k);
        }
    });

    println! {"nodes: {:?}", nodes};
    while !ghost_finished(&nodes) {
        steps += 1;
        let left = network.next_step(dir_ctr);
        if dir_ctr == network.directions.len() - 1 {
            dir_ctr = 0;
        } else {
            dir_ctr += 1;
        }
        let mut next_nodes = Vec::new();
        for node in nodes {
            if left {
                next_nodes.push(&network.netmap[node][0]);
            } else {
                next_nodes.push(&network.netmap[node][1]);
            }
        }
        nodes = next_nodes;
    }

    println! {"Steps: {}", steps};
    steps
}

#[test]
fn test_prelim() {
    let steps = compute_steps(&get_input("prelim.txt"));
    assert_eq!(steps, 2);
}

#[test]
fn test_prelim_a() {
    let steps = compute_steps(&get_input("prelim_a.txt"));
    assert_eq!(steps, 6);
}

#[test]
fn test_part1() {
    let steps = compute_steps(&get_input("input.txt"));
    assert_eq!(steps, 18157);
}

#[test]
fn test_prelim_2() {
    let steps = compute_ghost_steps(&get_input("prelim2.txt"));
    assert_eq!(steps, 6);
}

fn main() {
    compute_steps(&get_input("prelim.txt"));
    compute_steps(&get_input("prelim_a.txt"));
    compute_steps(&get_input("input.txt"));
    compute_ghost_steps(&get_input("prelim2.txt"));
    compute_ghost_steps(&get_input("input.txt"));
}

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
    dir_ctr: usize,
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
        Self {
            directions,
            netmap,
            dir_ctr: 0,
        }
    }

    fn next_step(&mut self) -> bool {
        let next = self.directions[self.dir_ctr];
        if self.dir_ctr == (self.directions.len() - 1) {
            self.dir_ctr = 0;
        } else {
            self.dir_ctr += 1;
        }
        next
    }
}

fn compute_steps(lines: &[String]) -> u64 {
    let mut network = Network::new(lines);

    let mut steps = 0;
    let start: Node = "AAA".to_node();
    let stop: Node = "ZZZ".to_node();
    let mut cur_node = start;
    while cur_node != stop {
        steps += 1;
        let left = network.next_step();
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

fn main() {
    compute_steps(&get_input("prelim.txt"));
    compute_steps(&get_input("prelim_a.txt"));
    compute_steps(&get_input("input.txt"));
}

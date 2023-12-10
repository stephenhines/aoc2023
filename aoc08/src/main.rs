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
    directions: Vec<usize>,
    netmap: HashMap<Node, [Node; 2]>,
}

impl Network {
    fn new(lines: &[String]) -> Self {
        let mut directions = Vec::new();
        let line = lines.first().unwrap();
        for i in line.chars() {
            if i == 'L' {
                directions.push(0);
            } else {
                directions.push(1);
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

    fn next_step(&self, dir_ctr: usize) -> usize {
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
        let left_or_right = network.next_step(dir_ctr);
        if dir_ctr == network.directions.len() - 1 {
            dir_ctr = 0;
        } else {
            dir_ctr += 1;
        }
        cur_node = network.netmap[&cur_node][left_or_right];
    }

    println! {"Steps: {}", steps};
    steps
}

fn find_loop_count(network: &Network, start: &Node) -> usize {
    let mut steps = 0;
    let mut dir_ctr = 0;
    let mut node = start;
    while node[2] != 'Z' {
        steps += 1;
        let left_or_right = network.next_step(dir_ctr);
        if dir_ctr == network.directions.len() - 1 {
            dir_ctr = 0;
        } else {
            dir_ctr += 1;
        }
        node = &network.netmap[node][left_or_right];
    }

    println! {"loop count: {}", steps};
    println! {"dir_len: {}", network.directions.len()};

    // Each of the loops of these nodes has the same period, even if the exact
    // path isn't repeated.

    steps
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if b > a {
        return gcd(b, a);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    // According to wikipedia, `lcm(a, b) = \abs(ab) / gcd(a, b)`
    a * (b / gcd(a, b))
}

fn lcm_multi(nums: &Vec<usize>) -> usize {
    let mut val = nums[0];
    for num in &nums[1..] {
        val = lcm(val, *num);
    }
    val
}

fn compute_ghost_steps(lines: &[String]) -> usize {
    let network = Network::new(lines);

    let mut nodes = Vec::new();
    network.netmap.keys().for_each(|k| {
        if k[2] == 'A' {
            nodes.push(k);
        }
    });

    //println! {"nodes: {:?}", nodes};
    let mut loop_counts = Vec::new();
    for node in nodes {
        loop_counts.push(find_loop_count(&network, node));
    }
    //println! {"loop_counts: {:?}", loop_counts};

    // Now that we have all the counts, we need to find the least common multiple
    // of all the values. We can do this by factoring all the numbers

    let lcm_steps = lcm_multi(&loop_counts);

    println! {"Steps: {}", lcm_steps};
    lcm_steps
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

#[test]
fn test_part2() {
    let steps = compute_ghost_steps(&get_input("input.txt"));
    assert_eq!(steps, 14299763833181);
}

fn main() {
    compute_steps(&get_input("prelim.txt"));
    compute_steps(&get_input("prelim_a.txt"));
    compute_steps(&get_input("input.txt"));
    compute_ghost_steps(&get_input("prelim2.txt"));
    compute_ghost_steps(&get_input("input.txt"));
}

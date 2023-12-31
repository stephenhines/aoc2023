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

const DEBUG: bool = false;

fn find_reflection(graph: &Vec<Vec<bool>>) -> usize {
    // Check for vertical reflections first
    // Check for horizontal reflections next
    let width = graph[0].len();
    let height = graph.len();

    'check_outer: for midpoint in 1..width {
        // go left and right simultaneously to check.
        // midpoint is the actual right side
        //println!("Checking horizontal midpoint: {}", midpoint);
        for row in graph {
            //println!("row: {:?}", row);
            let mut keep_going = true;
            let mut left = midpoint - 1;
            let mut right = midpoint;
            while keep_going {
                //println!("left: {} right: {}", left, right);
                if row[left] != row[right] {
                    //println!("failed l: {} r: {}", row[left], row[right]);
                    continue 'check_outer;
                }
                if left == 0 {
                    keep_going = false;
                } else {
                    left -= 1;
                }
                if right == width - 1 {
                    keep_going = false;
                } else {
                    right += 1;
                }
            }
        }
        // If we get here, we found a vertical mirroring around midpoint.
        if DEBUG {
            println!("Found midpoint: {}", midpoint);
            for y in 0..height {
                for x in 0..midpoint {
                    print!("{}", if graph[y][x] { "#" } else { "." });
                }
                print!("|");
                for x in midpoint..width {
                    print!("{}", if graph[y][x] { "#" } else { "." });
                }
                println!("");
            }
        }
        return midpoint;
    }

    //println!("width: {} height: {}", width, height);
    'check_outer: for midpoint in 1..height {
        //println!("Checking vertical midpoint: {}", midpoint);
        let mut top = midpoint - 1;
        let mut bottom = midpoint;
        let mut keep_going = true;
        while keep_going {
            //println!("Checking: top: {} bottom: {}", top, bottom);
            if graph[top] != graph[bottom] {
                //println!("width: {} height: {}", width, height);
                //println!("fail: t: {:?} b: {:?}", graph[top], graph[bottom]);
                continue 'check_outer;
            }
            if top == 0 {
                keep_going = false;
            } else {
                top -= 1;
            }
            if bottom == height - 1 {
                keep_going = false;
            } else {
                bottom += 1;
            }
        }

        if DEBUG {
            println!("Found vertical midpoint {}", midpoint);
            for y in 0..midpoint {
                for x in 0..width {
                    print!("{}", if graph[y][x] { "#" } else { "." });
                }
                println!("");
            }
            for _ in 0..width {
                print!("-");
            }
            println!("");
            for y in midpoint..height {
                for x in 0..width {
                    print!("{}", if graph[y][x] { "#" } else { "." });
                }
                println!("");
            }
        }
        return midpoint * 100;
    }

    if DEBUG {
        println!("BROKEN:");
        for y in 0..height {
            for x in 0..width {
                print!("{}", if graph[y][x] { "#" } else { "." });
            }
            println!("");
        }
    }
    panic!("Shouldn't get here");
}

fn get_reflection_score(lines: &[String]) -> usize {
    let mut score = 0;
    let mut rows: Vec<_> = Vec::new();
    for line in lines {
        if line.is_empty() {
            score += find_reflection(&rows);
            rows = Vec::new();
        } else {
            let mut cols: Vec<_> = Vec::new();
            for c in line.chars() {
                cols.push(if c == '#' { true } else { false });
            }
            rows.push(cols);
        }
    }
    score += find_reflection(&rows);
    //dbg!(rows);
    println!("Score: {}", score);
    score
}

#[test]
fn test_prelim() {
    let score = get_reflection_score(&get_input("prelim.txt"));
    assert_eq!(score, 405);
}

#[test]
fn test_part1() {
    let score = get_reflection_score(&get_input("input.txt"));
    assert_eq!(score, 42974);
}

fn main() {
    get_reflection_score(&get_input("prelim.txt"));
    get_reflection_score(&get_input("input.txt"));
}

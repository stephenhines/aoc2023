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

#[derive(Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Coord>,
    x_coords: HashSet<usize>,
    y_coords: HashSet<usize>,
    x_gaps: Vec<usize>,
    y_gaps: Vec<usize>,
}

impl Universe {
    fn new(lines: &[String]) -> Self {
        let mut galaxies = Vec::new();
        let mut x_coords = HashSet::new();
        let mut y_coords = HashSet::new();
        let mut x_gaps = Vec::new();
        let mut y_gaps = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    galaxies.push(Coord::new(x, y));
                    x_coords.insert(x);
                    y_coords.insert(y);
                }
            }
        }

        let y_max = lines.len();
        let x_max = lines[0].len();

        for x in 0..x_max {
            if !x_coords.contains(&x) {
                x_gaps.push(x);
            }
        }

        for y in 0..y_max {
            if !y_coords.contains(&y) {
                y_gaps.push(y);
            }
        }

        Self {
            galaxies,
            x_coords,
            y_coords,
            x_gaps,
            y_gaps,
        }
    }

    fn distance(&self, c1: &Coord, c2: &Coord) -> usize {
        let mut dist = 0;
        if c1 == c2 {
            return 0;
        }
        let low_x = std::cmp::min(c1.x, c2.x);
        let high_x = std::cmp::max(c1.x, c2.x);
        let low_y = std::cmp::min(c1.y, c2.y);
        let high_y = std::cmp::max(c1.y, c2.y);
        (low_x..high_x)
            .filter(|x| self.x_gaps.contains(x))
            .for_each(|_| dist += 1);
        (low_y..high_y)
            .filter(|y| self.y_gaps.contains(y))
            .for_each(|_| dist += 1);
        dist += high_x - low_x;
        dist += high_y - low_y;
        dist
    }
}

fn compute_shortest_paths(lines: &[String]) -> usize {
    let mut dist = 0;
    let universe = Universe::new(lines);

    //println! {"universe: {:?}", universe};

    let galaxies = universe.galaxies.len();
    for i in 0..galaxies {
        for j in i..galaxies {
            dist += universe.distance(&universe.galaxies[i], &universe.galaxies[j]);
        }
    }

    println! {"Shortest paths: {}", dist};
    dist
}

#[test]
fn test_prelim() {
    let steps = compute_shortest_paths(&get_input("prelim.txt"));
    assert_eq!(steps, 374);
}

#[test]
fn test_part1() {
    let steps = compute_shortest_paths(&get_input("input.txt"));
    assert_eq!(steps, 10289334);
}

fn main() {
    compute_shortest_paths(&get_input("prelim.txt"));
    compute_shortest_paths(&get_input("input.txt"));
}

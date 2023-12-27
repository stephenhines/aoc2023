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

// Set to true to enable debug prints.
const DEBUG: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    start: Coord,
    pipemap: Vec<Vec<char>>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "pipemap[{}][{}]", self.height, self.width)?;
        writeln!(f, "start: {:?}", self.start)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.pipemap[y][x])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(lines: &[String]) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut start = Coord::default();
        let mut pipemap = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            let mut piperow: Vec<_> = Vec::new();
            for (x, c) in line.char_indices() {
                piperow.push(c);
                if c == 'S' {
                    start = (x, y);
                }
            }
            pipemap.push(piperow);
        }

        // Replace S with the correctly shaped piece to make things easier.
        let (x, y) = start;
        let mut valid = (false, false, false, false);
        // Check up
        if y > 0 {
            let up = pipemap[y - 1][x];
            match up {
                'F' | '7' | '|' => {
                    valid.0 = true;
                }
                _ => {}
            }
        }
        if y + 1 < height {
            let down = pipemap[y + 1][x];
            match down {
                'L' | 'J' | '|' => {
                    valid.1 = true;
                }
                _ => {}
            }
        }
        if x > 0 {
            let left = pipemap[y][x - 1];
            match left {
                'F' | 'L' | '-' => {
                    valid.2 = true;
                }
                _ => {}
            }
        }
        if x + 1 < width {
            let right = pipemap[y][x + 1];
            match right {
                'J' | '7' | '-' => {
                    valid.3 = true;
                }
                _ => {}
            }
        }

        let replace_start = match valid {
            // (Up, Down, Left, Right)
            (true, true, false, false) => '|',
            (true, false, true, false) => 'J',
            (true, false, false, true) => 'L',
            (false, true, true, false) => '7',
            (false, true, false, true) => 'F',
            (false, false, true, true) => '-',
            (_, _, _, _) => {
                panic!("Invalid Start state: {:?}", start);
            }
        };

        pipemap[y][x] = replace_start;

        Grid {
            height,
            width,
            start,
            pipemap,
        }
    }

    fn get_next_dir(&self, pos: Coord, from: Direction) -> Direction {
        let (x, y) = pos;
        let c = self.pipemap[y][x];
        match c {
            'F' => match from {
                Direction::Up => {
                    return Direction::Right;
                }
                Direction::Left => {
                    return Direction::Down;
                }
                _ => {}
            },
            'L' => match from {
                Direction::Down => {
                    return Direction::Right;
                }
                Direction::Left => {
                    return Direction::Up;
                }
                _ => {}
            },
            'J' => match from {
                Direction::Down => {
                    return Direction::Left;
                }
                Direction::Right => {
                    return Direction::Up;
                }
                _ => {}
            },
            '7' => match from {
                Direction::Up => {
                    return Direction::Left;
                }
                Direction::Right => {
                    return Direction::Down;
                }
                _ => {}
            },
            '|' => match from {
                Direction::Up | Direction::Down => {
                    return from;
                }
                _ => {}
            },
            '-' => match from {
                Direction::Left | Direction::Right => {
                    return from;
                }
                _ => {}
            },
            'S' | '.' => {}
            _ => {
                panic!("Unknown tile: {}", c);
            }
        }
        Direction::Invalid
    }

    fn get_loop(&self) -> HashSet<Coord> {
        let mut set: HashSet<Coord> = HashSet::new();
        let mut pos = self.start;

        let start_pipe = self.pipemap[pos.1][pos.0];
        let dir = match start_pipe {
            'J' | 'L' | '|' => Direction::Up,
            '7' | 'F' => Direction::Down,
            '-' => Direction::Left,
            _ => Direction::Invalid,
        };
        //println!("{}", self);

        match dir {
            Direction::Up => {
                if pos.1 > 0 {
                    pos.1 -= 1;
                }
            }
            Direction::Down => {
                if pos.1 + 1 < self.height {
                    pos.1 += 1;
                }
            }
            Direction::Left => {
                if pos.0 > 0 {
                    pos.0 -= 1;
                }
            }
            Direction::Right => {
                if pos.0 + 1 < self.width {
                    pos.0 += 1;
                }
            }
            _ => {
                panic!("Unknown direction: {:?}", dir);
            }
        }
        let mut from = dir;

        //println!("Adding start {:?} and going {:?}", self.start, from);
        if self.get_next_dir(pos, from) == Direction::Invalid {
            return /* empty */ set;
        }

        set.insert(self.start);
        loop {
            if set.contains(&pos) {
                break;
            }
            set.insert(pos);
            from = self.get_next_dir(pos, from);
            //println!("Visiting {:?} and going {:?}", pos, from);

            match from {
                Direction::Up => {
                    pos.1 -= 1;
                }
                Direction::Down => {
                    pos.1 += 1;
                }
                Direction::Left => {
                    pos.0 -= 1;
                }
                Direction::Right => {
                    pos.0 += 1;
                }
                _ => {
                    panic!("Unknown direction: {:?} {:?}", dir, pos);
                }
            }
        }

        set
    }

    fn get_max_distance(&self) -> usize {
        let loop_len = self.get_loop().len();

        // Max distance is halfway around the loop (rounding up)
        (loop_len + 1) / 2
    }

    fn get_enclosed_area(&self) -> usize {
        let mut area = 0;
        let set = self.get_loop();

        // Use parity to determine inside/outside. Odd number of border
        // crossings will be inside, and even numbers will be outside.A
        // We only need to track |, L, and J (and maybe S) for border
        // crossing points. F and 7
        for y in 0..self.height {
            // Always start outside
            let mut inside = false;
            for x in 0..self.width {
                let c = (x, y);
                if set.contains(&c) {
                    let shape = self.pipemap[y][x];
                    if DEBUG {
                        print!("{}", shape);
                    }
                    match shape {
                        '|' | 'J' | 'L' => {
                            inside = !inside;
                        }
                        '7' | 'F' | '-' => {
                            // Do nothing
                        }
                        _ => {
                            panic!("Unknown symbol: {}", shape);
                        }
                    }
                } else {
                    if inside {
                        area += 1;
                        if DEBUG {
                            print!("I");
                        }
                    } else {
                        if DEBUG {
                            print!("O");
                        }
                    }
                }
            }
            if DEBUG {
                println!("");
            }
        }

        area
    }
}

fn get_max_distance(lines: &[String]) -> usize {
    let grid = Grid::new(lines);

    let len = grid.get_max_distance();
    println!("max length: {}", len);
    len
}

fn get_area(lines: &[String]) -> usize {
    let grid = Grid::new(lines);

    let area = grid.get_enclosed_area();
    println!("area: {}", area);
    area
}

#[test]
fn test_prelim() {
    let dist = get_max_distance(&get_input("prelim.txt"));
    assert_eq!(dist, 8);
}

#[test]
fn test_part1() {
    let dist = get_max_distance(&get_input("input.txt"));
    assert_eq!(dist, 6867);
}

#[test]
fn test_prelim2() {
    let area = get_area(&get_input("prelim2.txt"));
    assert_eq!(area, 10);
}

#[test]
fn test_part2() {
    let area = get_area(&get_input("input.txt"));
    assert_eq!(area, 595);
}

fn main() {
    get_max_distance(&get_input("prelim.txt"));
    get_max_distance(&get_input("input.txt"));
    get_area(&get_input("prelim2.txt"));
    get_area(&get_input("input.txt"));
}

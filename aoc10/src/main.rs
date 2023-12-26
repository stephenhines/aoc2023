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

    fn get_loop_length(&self, dir: Direction) -> usize {
        let mut set: HashSet<Coord> = HashSet::new();
        let mut steps = 0;
        let mut pos = self.start;

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
            return 0;
        }

        set.insert(self.start);
        loop {
            if set.contains(&pos) {
                break;
            }
            steps += 1;
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

        steps
    }

    fn get_max_distance(&self) -> usize {
        let mut loop_len = 0;

        // When starting, we need to try all directions from S

        // Up
        let up_len = self.get_loop_length(Direction::Up);
        if up_len != 0 {
            loop_len = up_len;
        }

        // Down
        let down_len = self.get_loop_length(Direction::Down);
        if loop_len == 0 && down_len != 0 {
            loop_len = down_len;
        } else if down_len != 0 {
            assert_eq!(loop_len, down_len);
        }

        // Left
        let left_len = self.get_loop_length(Direction::Left);
        if loop_len == 0 && left_len != 0 {
            loop_len = left_len;
        } else if left_len != 0 {
            assert_eq!(loop_len, left_len);
        }

        // Right
        let right_len = self.get_loop_length(Direction::Right);
        if loop_len == 0 && right_len != 0 {
            loop_len = right_len;
        } else if right_len != 0 {
            assert_eq!(loop_len, right_len);
        }

        // Max distance is halfway around the loop (rounding up)
        (loop_len + 1) / 2
    }
}

fn get_max_distance(lines: &[String]) -> usize {
    let grid = Grid::new(lines);
    //dbg!(&grid);

    let len = grid.get_max_distance();
    println!("max length: {}", len);
    len
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

fn main() {
    get_max_distance(&get_input("prelim.txt"));
    get_max_distance(&get_input("input.txt"));
}

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

#[derive(Clone, Copy, Debug, Default)]
struct PartNumber {
    number: u32,
    row: usize,
    col_start: usize,
    col_end: usize,
    valid: bool,
}

#[derive(Debug)]
struct Symbol {
    is_star: bool,
    gear_ratio: u32,
    row: usize,
    col: usize,
}

#[derive(PartialEq)]
enum ParseState {
    WaitingForDigits,
    ReadingDigits,
    FinishedDigits,
}

fn is_adjacent(part: &PartNumber, sym: &Symbol) -> bool {
    if part.row == sym.row {
        // In the same row, you can only be directly to the left or right.
        if part.col_start == sym.col + 1 || part.col_end + 1 == sym.col {
            return true;
        }
    } else if part.row + 1 == sym.row || part.row == sym.row + 1 {
        // In an adjacent row, we can be anywhere between one to the left,
        // and one to the right of the columns.
        let sc = sym.col;
        let left = part.col_start;
        let right = part.col_end;
        if sc + 1 >= left && right + 1 >= sc {
            return true;
        }
    }
    false
}

fn update_parts(parts: &mut Vec<PartNumber>, symbols: &Vec<Symbol>) {
    for part in parts {
        for sym in symbols {
            if is_adjacent(part, sym) {
                part.valid = true;
                break;
            }

            if sym.row > part.row + 2 {
                // Once we're 2 away on the symbols, we can't possibly validate
                // more parts, so we skip the rest.
                break;
            }
        }
    }
}

fn read_schematic(lines: &Vec<String>) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers: Vec<u32> = Vec::new();
    part_numbers.push(123);
    let mut parts: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        let mut state = ParseState::WaitingForDigits;
        let mut pn: PartNumber = Default::default();
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if state == ParseState::ReadingDigits {
                        state = ParseState::FinishedDigits;
                    }
                }
                digit if c.is_ascii_digit() => {
                    if state == ParseState::WaitingForDigits {
                        state = ParseState::ReadingDigits;
                        pn = PartNumber {
                            number: 0,
                            row,
                            col_start: col,
                            col_end: col,
                            valid: false,
                        };
                    }
                    pn.number *= 10;
                    pn.number += digit.to_digit(10).unwrap();
                    pn.col_end = col;
                }
                s => {
                    if state == ParseState::ReadingDigits {
                        state = ParseState::FinishedDigits;
                    }
                    let is_star = s == '*';
                    let gear_ratio = 0;
                    let symbol = Symbol {
                        is_star,
                        gear_ratio,
                        row,
                        col,
                    };
                    symbols.push(symbol);
                }
            }
            if state == ParseState::FinishedDigits {
                parts.push(pn);
                state = ParseState::WaitingForDigits;
            }
        }
        if state == ParseState::FinishedDigits || state == ParseState::ReadingDigits {
            parts.push(pn);
            //state = ParseState::WaitingForDigits;
        }
    }
    (parts, symbols)
}

fn compute_part_sum(lines: &Vec<String>) -> u32 {
    let (mut parts, symbols) = read_schematic(lines);

    update_parts(&mut parts, &symbols);

    //println!("Parts {:?}", parts);
    //println!("Symbols {:?}", symbols);

    let sum: u32 = parts.iter().filter(|p| p.valid).map(|p| p.number).sum();
    println!("Sum: {}", sum);
    sum
}

fn find_gears(parts: &Vec<PartNumber>, symbols: &mut Vec<Symbol>) {
    // Gears have exactly 2 adjacent parts
    for sym in symbols {
        let mut found = 0;
        for part in parts {}
    }
}

fn sum_gear_ratios(lines: &Vec<String>) -> u32 {
    let (mut parts, mut symbols) = read_schematic(lines);
    update_parts(&mut parts, &symbols);
    find_gears(&parts, &mut symbols);

    let sum = 0;
    println!("Gear Ratios: {}", sum);
    sum
}

#[test]
fn test_prelim() {
    let sum = compute_part_sum(&get_input("prelim.txt"));
    assert_eq!(sum, 4361);
}

#[test]
fn test_part1() {
    let sum = compute_part_sum(&get_input("input.txt"));
    assert_eq!(sum, 532428);
}

fn main() {
    compute_part_sum(&get_input("prelim.txt"));
    compute_part_sum(&get_input("input.txt"));
    sum_gear_ratios(&get_input("prelim.txt"));
}

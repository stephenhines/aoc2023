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
struct Part {
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

fn is_adjacent(part: &Part, sym: &Symbol) -> bool {
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

fn update_parts(parts: &mut Vec<Part>, symbols: &Vec<Symbol>) {
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

fn read_schematic(lines: &[String]) -> (Vec<Part>, Vec<Symbol>) {
    #[derive(PartialEq)]
    enum ParseDigits {
        Waiting,  // Waiting to read the start of a span of digits
        Reading,  // Reading through a span of digits
        Finished, // Finished reading a span (need to update and clean up)
    }

    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        let mut state = ParseDigits::Waiting;
        let mut p: Part = Default::default();
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if state == ParseDigits::Reading {
                        state = ParseDigits::Finished;
                    }
                }
                digit if c.is_ascii_digit() => {
                    if state == ParseDigits::Waiting {
                        state = ParseDigits::Reading;
                        p = Part {
                            number: 0,
                            row,
                            col_start: col,
                            col_end: col,
                            valid: false,
                        };
                    }
                    p.number *= 10;
                    p.number += digit.to_digit(10).unwrap();
                    p.col_end = col;
                }
                s => {
                    if state == ParseDigits::Reading {
                        state = ParseDigits::Finished;
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
            if state == ParseDigits::Finished {
                parts.push(p);
                state = ParseDigits::Waiting;
            }
        }
        if state == ParseDigits::Finished || state == ParseDigits::Reading {
            parts.push(p);
            //state = ParseDigits::Waiting;
        }
    }
    (parts, symbols)
}

fn compute_part_sum(lines: &[String]) -> u32 {
    let (mut parts, symbols) = read_schematic(lines);

    update_parts(&mut parts, &symbols);

    //println!("Parts {:?}", parts);
    //println!("Symbols {:?}", symbols);

    let sum: u32 = parts.iter().filter(|p| p.valid).map(|p| p.number).sum();
    println!("Sum: {}", sum);
    sum
}

fn find_gears(parts: &[Part], symbols: &mut [Symbol]) {
    // Gears have exactly 2 adjacent parts with a star symbol.
    for sym in symbols.iter_mut().filter(|s| s.is_star) {
        let mut found = 0;
        let mut gear_components: Vec<u32> = Vec::new();
        for part in parts {
            if is_adjacent(part, sym) {
                found += 1;
                gear_components.push(part.number);
            }
            if found >= 3 {
                break;
            }
        }
        if found == 2 {
            sym.gear_ratio = gear_components.iter().product();
            //println!("Gear: {:?}", sym);
        }
    }
}

fn sum_gear_ratios(lines: &[String]) -> u32 {
    let (mut parts, mut symbols) = read_schematic(lines);
    update_parts(&mut parts, &symbols);
    find_gears(&parts, &mut symbols);

    let sum: u32 = symbols
        .iter()
        .filter(|s| s.is_star)
        .map(|s| s.gear_ratio)
        .sum();
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

#[test]
fn test_prelim2() {
    let sum = sum_gear_ratios(&get_input("prelim.txt"));
    assert_eq!(sum, 467835);
}

#[test]
fn test_part2() {
    let sum = sum_gear_ratios(&get_input("input.txt"));
    assert_eq!(sum, 84051670);
}

fn main() {
    compute_part_sum(&get_input("prelim.txt"));
    compute_part_sum(&get_input("input.txt"));
    sum_gear_ratios(&get_input("prelim.txt"));
    sum_gear_ratios(&get_input("input.txt"));
}

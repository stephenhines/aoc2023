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

#[derive(Clone, Copy)]
enum Direction {
    Backward,
    Forward,
}

fn get_differences(values: &Vec<isize>) -> Vec<isize> {
    let mut diff: Vec<_> = values.windows(2).map(|x| x[1] - x[0]).collect();

    if diff.iter().all(|&x| x == 0) {
        diff.push(0);
    } else {
        diff.push(diff.last().unwrap() + get_differences(&diff).last().unwrap());
    }
    //println!("diff: {:?}", diff);
    diff
}

fn get_prev_differences(values: &Vec<isize>) -> Vec<isize> {
    let mut diff: Vec<_> = values.windows(2).map(|x| x[1] - x[0]).collect();

    if diff.iter().all(|&x| x == 0) {
        diff.insert(0, 0);
    } else {
        diff.insert(0, diff.first().unwrap() - get_prev_differences(&diff)[0]);
    }
    //println!("diff: {:?}", diff);
    diff
}

fn extrapolate_line(line: &String, dir: Direction) -> isize {
    let mut vals = Vec::new();
    line.split_whitespace()
        .for_each(|tok| vals.push(tok.parse::<isize>().unwrap()));

    let diff = match dir {
        Direction::Forward => vals.last().unwrap() + get_differences(&vals).last().unwrap(),
        Direction::Backward => vals.first().unwrap() - get_prev_differences(&vals)[0],
    };
    //println!("line diff: {}", diff);
    diff
}

fn extrapolate(lines: &[String], dir: Direction) -> isize {
    let mut sum = 0;

    for line in lines {
        sum += extrapolate_line(line, dir);
    }

    println!("sum: {}", sum);
    sum
}

#[test]
fn test_prelim() {
    let sum = extrapolate(&get_input("prelim.txt"), Direction::Forward);
    assert_eq!(sum, 114);
}

#[test]
fn test_part1() {
    let sum = extrapolate(&get_input("input.txt"), Direction::Forward);
    assert_eq!(sum, 1934898178);
}

#[test]
fn test_prelim2() {
    let sum = extrapolate(&get_input("prelim.txt"), Direction::Backward);
    assert_eq!(sum, 2);
}

#[test]
fn test_part2() {
    let sum = extrapolate(&get_input("input.txt"), Direction::Backward);
    assert_eq!(sum, 1129);
}

fn main() {
    extrapolate(&get_input("prelim.txt"), Direction::Forward);
    extrapolate(&get_input("input.txt"), Direction::Forward);
    extrapolate(&get_input("prelim.txt"), Direction::Backward);
    extrapolate(&get_input("input.txt"), Direction::Backward);
}

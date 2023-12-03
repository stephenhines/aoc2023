use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn get_input(filename : &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn calibrate(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let v: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
        assert_ne!(v.len(), 0);
        //println!("Vec: {:?}", v);
        let value = v[0].to_digit(10).unwrap() * 10 + v.last().unwrap().to_digit(10).unwrap();
        sum += value;
    }
    println!("Calibrate: {:?}", sum);
    sum
}

const PATTERNS : [&str; 10] = ["IGNORED", "one", "two", "three", "four", "five", "six",
"seven", "eight", "nine"];

fn get_line_calibration(line: &String) -> u32 {
    let mut first_index = line.len();
    let mut first_index_end = line.len();
    let mut last_index = 0;
    let mut first_value = 0;
    let mut last_value = 0;

    if let Some(idx) = line.find(|c: char| c.is_ascii_digit()) {
        first_index = idx;
        first_index_end = idx;
        first_value = line.chars().collect::<Vec<_>>()[idx].to_digit(10).unwrap();
    }
    if let Some(idx) = line.rfind(|c: char| c.is_ascii_digit()) {
        last_index = idx;
        last_value = line.chars().collect::<Vec<_>>()[idx].to_digit(10).unwrap();
    }

    let mut i = 1;  // Skip the IGNORED zero entry
    while i < PATTERNS.len() {
        if let Some(idx) = line[..first_index_end].find(PATTERNS[i]) {
            if idx < first_index {
                first_index = idx;
                first_index_end = idx + PATTERNS[i].len();
                first_value = i as u32;
            }
        }
        if let Some(idx) = line[last_index..].rfind(PATTERNS[i]) {
            last_index += idx;  // += since we sliced into the original string
            last_value = i as u32;
        }
        i += 1;
    }

    let v = 10 * first_value + last_value;
    //println!("line: {:?} v: {:?}", line, v);
    v
}

fn calibratetwo(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        sum += get_line_calibration(line);
    }
    println!("CalibrateTwo: {:?}", sum);
    sum
}

#[test]
fn test_prelim() {
    let cal = calibrate(&get_input("prelim.txt"));
    assert_eq!(cal, 142);
}

#[test]
fn test_prelim2() {
    let cal = calibratetwo(&get_input("prelim2.txt"));
    assert_eq!(cal, 281);
}

#[test]
fn test_part1() {
    let cal = calibrate(&get_input("input.txt"));
    assert_eq!(cal, 54388);
}

#[test]
fn test_part2() {
    let cal = calibratetwo(&get_input("input.txt"));
    assert_eq!(cal, 53515);
}

fn main() {
    calibrate(&get_input("prelim.txt"));
    calibrate(&get_input("input.txt"));
    calibratetwo(&get_input("prelim2.txt"));
    calibratetwo(&get_input("input.txt"));
}

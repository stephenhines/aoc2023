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

fn get_differences(values: &Vec<isize>) -> Vec<isize> {
    let mut diff: Vec<_> = values.windows(2).map(|x| x[1] - x[0]).collect();

    if diff.iter().all(|&x| x == 0) {
        diff.push(0);
        diff
    } else {
        //let mut sub_diff = get_differences(&diff);
        diff.push(diff.last().unwrap() + get_differences(&diff).last().unwrap());
        //println!("diff: {:?}", diff);
        diff
    }
}

fn extrapolate_line(line: &String) -> isize {
    let mut vals = Vec::new();
    let toks: Vec<_> = line.split_whitespace().collect();
    for tok in toks {
        vals.push(tok.parse::<isize>().unwrap());
    }
    //println!("vals: {:?}", vals);
    let diff = vals.last().unwrap() + get_differences(&vals).last().unwrap();
    //println!("line diff: {}", diff);
    diff
}

fn extrapolate(lines: &[String]) -> isize {
    let mut sum = 0;

    for line in lines {
        sum += extrapolate_line(line);
    }

    println!("sum: {}", sum);
    sum
}

#[test]
fn test_prelim() {
    let sum = extrapolate(&get_input("prelim.txt"));
    assert_eq!(sum, 114);
}

#[test]
fn test_part1() {
    let sum = extrapolate(&get_input("input.txt"));
    assert_eq!(sum, 1934898178);
}

fn main() {
    extrapolate(&get_input("prelim.txt"));
    extrapolate(&get_input("input.txt"));
}

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

fn compute_wins(time: u64, distance: u64) -> u64 {
    let mut wins = 0;
    for time_pressing_button in 1..time {
        let speed = time_pressing_button;
        let trial_distance = speed * (time - time_pressing_button);
        if trial_distance > distance {
            wins += 1;
        }
    }

    wins
}

fn compute_racing_records(lines: &[String]) -> u64 {
    let mut race_times: Vec<u64> = Vec::new();
    let mut race_distances: Vec<u64> = Vec::new();

    for line in lines {
        let toks: Vec<&str> = line.split(":").collect();
        match toks[0] {
            "Time" => {
                let time_toks: Vec<&str> = toks[1].trim().split_whitespace().collect();
                for t in time_toks {
                    let time = t.parse::<u64>().unwrap();
                    race_times.push(time);
                }
            }
            "Distance" => {
                let dist_toks: Vec<&str> = toks[1].trim().split_whitespace().collect();
                for d in dist_toks {
                    let distance = d.parse::<u64>().unwrap();
                    race_distances.push(distance);
                }
            }
            _ => {
                panic! {"Unknown parse for line {}", line};
            }
        }
    }

    assert_eq!(race_times.len(), race_distances.len());

    //println!{"race_times: {:?}", race_times};
    //println!{"race_distances: {:?}", race_distances};

    let mut records = 1;
    for i in 0..race_times.len() {
        let wins = compute_wins(race_times[i], race_distances[i]);
        records *= wins;
    }

    println! {"Records: {}", records};
    records
}

fn compute_racing_records_kerned(lines: &[String]) -> u64 {
    let mut time_string = String::new();
    let mut distance_string = String::new();

    for line in lines {
        let toks: Vec<&str> = line.split(":").collect();
        match toks[0] {
            "Time" => {
                let time_toks: Vec<&str> = toks[1].trim().split_whitespace().collect();
                for t in time_toks {
                    time_string += t;
                }
            }
            "Distance" => {
                let dist_toks: Vec<&str> = toks[1].trim().split_whitespace().collect();
                for d in dist_toks {
                    distance_string += d;
                }
            }
            _ => {
                panic! {"Unknown parse for line {}", line};
            }
        }
    }

    let time = time_string.parse::<u64>().unwrap();
    let distance = distance_string.parse::<u64>().unwrap();

    //println!{"time: {}", time};
    //println!{"distance: {}", distance};

    let records = compute_wins(time, distance);

    println! {"Records: {}", records};
    records
}

#[test]
fn test_prelim() {
    let records = compute_racing_records(&get_input("prelim.txt"));
    assert_eq!(records, 288);
}

#[test]
fn test_part1() {
    let records = compute_racing_records(&get_input("input.txt"));
    assert_eq!(records, 114400);
}

#[test]
fn test_prelim2() {
    let records = compute_racing_records_kerned(&get_input("prelim.txt"));
    assert_eq!(records, 71503);
}

#[test]
fn test_part2() {
    let records = compute_racing_records_kerned(&get_input("input.txt"));
    assert_eq!(records, 21039729);
}

fn main() {
    compute_racing_records(&get_input("prelim.txt"));
    compute_racing_records(&get_input("input.txt"));
    compute_racing_records_kerned(&get_input("prelim.txt"));
    compute_racing_records_kerned(&get_input("input.txt"));
}

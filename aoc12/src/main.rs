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
enum SpringState {
    Working, // Valid start state
    InBroken,
    CheckEndBroken,
}

fn find_matches(springs: &[u8], groups: &[usize], state: SpringState, broken_pos: usize) -> usize {
    println! {"springs: {}, groups: {:?}, state: {:?}, broken_pos: {}", std::str::from_utf8(springs).unwrap(), groups, state, broken_pos};
    if springs.is_empty() {
        // Need to check more about state of broken springs seen
        //println!{"springs.is_empty()"};
        if state == SpringState::CheckEndBroken || state == SpringState::Working {
            //println!{"verifying"};
            if groups.is_empty() {
                //println!{"groups.is_empty"};
                return 1;
            }
        }
        println! {"failed"};
        return 0;
    }
    match springs[0] {
        b'#' => {
            if groups.is_empty() || state == SpringState::CheckEndBroken {
                // If we're out of broken springs, we're invalid.
                return 0;
            }
            if broken_pos + 1 == groups[0] {
                // Check if we're done on the next iteration.
                return find_matches(&springs[1..], &groups[1..], SpringState::CheckEndBroken, 0);
            }
            return find_matches(&springs[1..], groups, SpringState::InBroken, broken_pos + 1);
        }
        b'.' => {
            if state == SpringState::Working {
                return find_matches(&springs[1..], groups, state, broken_pos);
            } else if state == SpringState::CheckEndBroken {
                return find_matches(&springs[1..], groups, SpringState::Working, 0);
            } else if state == SpringState::InBroken {
                // We didn't get the right value
                return 0;
            }
        }
        b'?' => {
            // Evaluate multiple options possibly
            match state {
                SpringState::CheckEndBroken => {
                    return find_matches(&springs[1..], groups, SpringState::Working, 0);
                }
                SpringState::InBroken => {
                    if broken_pos + 1 == groups[0] {
                        // Check if we're done on the next iteration.
                        //println!{"Checking ? for InBroken"};
                        return find_matches(
                            &springs[1..],
                            &groups[1..],
                            SpringState::CheckEndBroken,
                            0,
                        );
                    }
                    //println!{"Checking continued for InBroken: {} {}", broken_pos, groups[0]};
                    return find_matches(
                        &springs[1..],
                        groups,
                        SpringState::InBroken,
                        broken_pos + 1,
                    );
                }
                SpringState::Working => {
                    let mut total = 0;
                    if !groups.is_empty() {
                        assert_eq!(broken_pos, 0);
                        if groups[0] == 1 {
                            // Handle this case without hassle
                            total += find_matches(
                                &springs[1..],
                                &groups[1..],
                                SpringState::CheckEndBroken,
                                0,
                            );
                        } else {
                            total += find_matches(
                                &springs[1..],
                                groups,
                                SpringState::InBroken,
                                broken_pos + 1,
                            );
                        }
                    }
                    return total + find_matches(&springs[1..], groups, state, 0);
                }
            }
        }
        _ => {
            panic!("Unknown input {:?}", springs);
        }
    }
    0
}

fn get_arrangements(line: &str) -> usize {
    let toks: Vec<_> = line.split_whitespace().collect();
    let groups: Vec<_> = toks[1]
        .split(",")
        .map(|g| g.parse::<usize>().unwrap())
        .collect();
    let springs = toks[0];
    println! {"groups: {:?}", groups};
    //println!{"springs: {:?}", springs};

    let mut arr = 0;
    /*if valid(springs, &groups) {
        arr += 1;
    }*/

    arr += find_matches(
        springs.as_bytes(),
        groups.as_slice(),
        SpringState::Working,
        0,
    );
    println! {"arrangements {}: {}", line, arr};

    arr
}

fn get_total_arrangements(lines: &[String]) -> usize {
    let sum = lines.iter().map(|l| get_arrangements(l)).sum();
    println! {"total arrangements: {}", sum};
    sum
}

fn test_basic() {
    let line = "#.#.### 1,1,3";
    get_arrangements(line);
    let line = "???.### 1,1,3";
    get_arrangements(line);
}

#[test]
fn test_prelim_full() {
    let arr = get_total_arrangements(&get_input("prelim_full.txt"));
    assert_eq!(arr, 6);
}

#[test]
fn test_prelim() {
    let arr = get_total_arrangements(&get_input("prelim.txt"));
    assert_eq!(arr, 21);
}

fn main() {
    get_total_arrangements(&get_input("prelim_full.txt"));
    get_total_arrangements(&get_input("prelim.txt"));
    get_total_arrangements(&get_input("input.txt"));
}

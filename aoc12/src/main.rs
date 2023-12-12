use std::collections::HashMap;
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum SpringState {
    Working, // Valid start state
    InBroken,
    CheckEndBroken,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Context<'a> {
    state: SpringState,
    springs: &'a [u8],
    broken_pos: usize,
    groups: &'a [usize],
}

fn find_matches<'a>(
    memo: &mut HashMap<(SpringState, usize, usize, usize), usize>,
    context: Context,
) -> usize {
    //println! {"springs: {}, groups: {:?}, state: {:?}, broken_pos: {}", std::str::from_utf8(context.springs).unwrap(), context.groups, context.state, context.broken_pos};
    let state = (
        context.state,
        context.springs.len(),
        context.broken_pos,
        context.groups.len(),
    );
    if memo.contains_key(&state) {
        return *memo.get(&state).unwrap();
    }

    if context.springs.is_empty() {
        // Need to check more about state of broken springs seen
        //println!{"springs.is_empty()"};
        if context.state == SpringState::CheckEndBroken || context.state == SpringState::Working {
            //println!{"verifying"};
            if context.groups.is_empty() {
                //println!{"groups.is_empty"};
                memo.insert(state, 1);
                return 1;
            }
        }
        memo.insert(state, 0);
        return 0;
    }

    if context.groups.len() != 0 {
        let remaining_broken: usize = context.groups.iter().sum::<usize>();
        if remaining_broken - context.broken_pos > context.springs.len() {
            memo.insert(state, 0);
            return 0;
        }
    }

    match context.springs[0] {
        b'#' => {
            if context.groups.is_empty() || context.state == SpringState::CheckEndBroken {
                // If we're out of broken springs, we're invalid.
                memo.insert(state, 0);
                return 0;
            }
            if context.broken_pos + 1 == context.groups[0] {
                // Check if we're done on the next iteration.
                let new_context = Context {
                    state: SpringState::CheckEndBroken,
                    springs: &context.springs[1..],
                    broken_pos: 0,
                    groups: &context.groups[1..],
                };
                let ret = find_matches(memo, new_context);
                let state = (
                    SpringState::CheckEndBroken,
                    context.springs.len() - 1,
                    0,
                    context.groups.len() - 1,
                );
                memo.insert(state, ret);
                return ret;
            }
            let new_context = Context {
                state: SpringState::InBroken,
                springs: &context.springs[1..],
                broken_pos: context.broken_pos + 1,
                groups: &context.groups,
            };
            let ret = find_matches(memo, new_context);
            let state = (
                SpringState::InBroken,
                context.springs.len() - 1,
                context.broken_pos + 1,
                context.groups.len(),
            );
            memo.insert(state, ret);
            return ret;
        }
        b'.' => {
            if context.state == SpringState::Working {
                let new_context = Context {
                    state: context.state.clone(),
                    springs: &context.springs[1..],
                    broken_pos: 0,
                    groups: &context.groups,
                };
                let ret = find_matches(memo, new_context);
                let state = (
                    context.state,
                    context.springs.len() - 1,
                    0,
                    context.groups.len(),
                );
                memo.insert(state, ret);
                return ret;
            } else if context.state == SpringState::CheckEndBroken {
                let new_context = Context {
                    state: SpringState::Working,
                    springs: &context.springs[1..],
                    broken_pos: 0,
                    groups: &context.groups,
                };
                let ret = find_matches(memo, new_context);
                let state = (
                    SpringState::Working,
                    context.springs.len() - 1,
                    0,
                    context.groups.len(),
                );
                memo.insert(state, ret);
                return ret;
            } else if context.state == SpringState::InBroken {
                // We didn't get the right value
                memo.insert(state, 0);
                return 0;
            }
        }
        b'?' => {
            // Evaluate multiple options possibly
            match context.state {
                SpringState::CheckEndBroken => {
                    let new_context = Context {
                        state: SpringState::Working,
                        springs: &context.springs[1..],
                        broken_pos: 0,
                        groups: &context.groups,
                    };
                    let ret = find_matches(memo, new_context);
                    let state = (
                        SpringState::Working,
                        context.springs.len() - 1,
                        0,
                        context.groups.len(),
                    );
                    memo.insert(state, ret);
                    return ret;
                }
                SpringState::InBroken => {
                    if context.broken_pos + 1 == context.groups[0] {
                        // Check if we're done on the next iteration.
                        //println!{"Checking ? for InBroken"};
                        let new_context = Context {
                            state: SpringState::CheckEndBroken,
                            springs: &context.springs[1..],
                            broken_pos: 0,
                            groups: &context.groups[1..],
                        };
                        let ret = find_matches(memo, new_context);
                        let state = (
                            SpringState::CheckEndBroken,
                            context.springs.len() - 1,
                            0,
                            context.groups.len() - 1,
                        );
                        memo.insert(state, ret);
                        return ret;
                    }
                    //println!{"Checking continued for InBroken: {} {}", broken_pos, groups[0]};
                    let new_context = Context {
                        state: SpringState::InBroken,
                        springs: &context.springs[1..],
                        broken_pos: context.broken_pos + 1,
                        groups: &context.groups,
                    };
                    let ret = find_matches(memo, new_context);
                    let state = (
                        SpringState::InBroken,
                        context.springs.len() - 1,
                        context.broken_pos + 1,
                        context.groups.len(),
                    );
                    memo.insert(state, ret);
                    return ret;
                }
                SpringState::Working => {
                    let mut total = 0;
                    if !context.groups.is_empty() {
                        assert_eq!(context.broken_pos, 0);
                        if context.groups[0] == 1 {
                            // Handle this case without hassle
                            let new_context = Context {
                                state: SpringState::CheckEndBroken,
                                springs: &context.springs[1..],
                                broken_pos: 0,
                                groups: &context.groups[1..],
                            };
                            let ret = find_matches(memo, new_context);
                            let state = (
                                SpringState::CheckEndBroken,
                                context.springs.len() - 1,
                                0,
                                context.groups.len() - 1,
                            );
                            memo.insert(state, ret);
                            total += ret;
                        } else {
                            let new_context = Context {
                                state: SpringState::InBroken,
                                springs: &context.springs[1..],
                                broken_pos: context.broken_pos + 1,
                                groups: &context.groups,
                            };
                            let ret = find_matches(memo, new_context);
                            let state = (
                                SpringState::InBroken,
                                context.springs.len() - 1,
                                context.broken_pos + 1,
                                context.groups.len(),
                            );
                            memo.insert(state, ret);
                            total += ret;
                        }
                    }
                    let new_context = Context {
                        state: SpringState::Working,
                        springs: &context.springs[1..],
                        broken_pos: 0,
                        groups: &context.groups,
                    };
                    let ret = find_matches(memo, new_context);
                    let state = (
                        SpringState::Working,
                        context.springs.len() - 1,
                        0,
                        context.groups.len(),
                    );
                    memo.insert(state, ret);
                    return ret + total;
                }
            }
        }
        _ => {
            panic!("Unknown input {:?}", &context.springs);
        }
    }
    0
}

fn get_arrangements(line: &str, damaged: bool) -> usize {
    let toks: Vec<_> = line.split_whitespace().collect();
    let mut expanded_springs_str = String::new();
    let mut expanded_groups_str = String::new();
    let groups_tok = if !damaged {
        toks[1]
    } else {
        for _ in 0..4 {
            expanded_groups_str += toks[1];
            expanded_groups_str += ",";
        }
        expanded_groups_str += toks[1];
        //println! {"expanded_groups_str: {}", expanded_groups_str};
        expanded_groups_str.as_str()
    };
    let groups: Vec<_> = groups_tok
        .split(",")
        .map(|g| g.parse::<usize>().unwrap())
        .collect();
    let springs = if !damaged {
        toks[0]
    } else {
        for _ in 0..4 {
            expanded_springs_str += toks[0];
            expanded_springs_str += "?";
        }
        expanded_springs_str += toks[0];
        //println! {"expanded_springs_str: {}", expanded_springs_str};
        expanded_springs_str.as_str()
    };

    let mut arr = 0;

    let mut memo: HashMap<_, _> = HashMap::new();

    let new_context = Context {
        state: SpringState::Working,
        springs: springs.as_bytes(),
        broken_pos: 0,
        groups: groups.as_slice(),
    };
    arr += find_matches(&mut memo, new_context);
    //println! {"arrangements {}: {}", line, arr};

    arr
}

fn get_total_arrangements(lines: &[String], damaged: bool) -> usize {
    let sum = lines.iter().map(|l| get_arrangements(l, damaged)).sum();
    println! {"total arrangements: {}", sum};
    sum
}

#[test]
fn test_basic() {
    let line = "#.#.### 1,1,3";
    assert_eq!(get_arrangements(line, false), 1);
    let line = "???.### 1,1,3";
    assert_eq!(get_arrangements(line, false), 1);
}

#[test]
fn test_prelim_full() {
    let arr = get_total_arrangements(&get_input("prelim_full.txt"), false);
    assert_eq!(arr, 6);
}

#[test]
fn test_prelim() {
    let arr = get_total_arrangements(&get_input("prelim.txt"), false);
    assert_eq!(arr, 21);
}

#[test]
fn test_part1() {
    let arr = get_total_arrangements(&get_input("input.txt"), false);
    assert_eq!(arr, 7718);
}

#[test]
fn test_prelim2() {
    let arr = get_total_arrangements(&get_input("prelim.txt"), true);
    assert_eq!(arr, 525152);
}

#[test]
fn test_part2() {
    let arr = get_total_arrangements(&get_input("input.txt"), true);
    assert_eq!(arr, 128741994134728);
}

fn main() {
    get_total_arrangements(&get_input("prelim_full.txt"), false);
    get_total_arrangements(&get_input("prelim.txt"), false);
    get_total_arrangements(&get_input("input.txt"), false);
    get_total_arrangements(&get_input("prelim.txt"), true);
    get_total_arrangements(&get_input("input.txt"), true);
}

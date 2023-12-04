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

fn score_cards(lines: &[String]) -> u32 {
    let mut total_score = 0;
    //let mut symbols: Vec<Symbol> = Vec::new();

    for line in lines {
        let mut winners = HashSet::new();
        let toks: Vec<&str> = line.split(":").collect();
        let data_toks: Vec<&str> = toks[1].split("|").collect();

        let win_toks: Vec<&str> = data_toks[0].trim().split_whitespace().collect();
        for n in win_toks {
            let winning_num = n.parse::<u32>().unwrap();
            winners.insert(winning_num);
        }

        let our_toks: Vec<&str> = data_toks[1].trim().split_whitespace().collect();
        let mut score = 0;
        for n in our_toks {
            let our_num = n.parse::<u32>().unwrap();
            if winners.contains(&our_num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        total_score += score;
    }

    println!("Total Score: {}", total_score);
    total_score
}

#[test]
fn test_prelim() {
    let score = score_cards(&get_input("prelim.txt"));
    assert_eq!(score, 13);
}

#[test]
fn test_part1() {
    let score = score_cards(&get_input("input.txt"));
    assert_eq!(score, 21558);
}

fn main() {
    score_cards(&get_input("prelim.txt"));
    score_cards(&get_input("input.txt"));
}

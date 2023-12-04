use std::cmp::min;
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

fn read_cards(lines: &[String]) -> Vec<u32> {
    let mut card_wins: Vec<u32> = Vec::new();

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
                score += 1;
            }
        }
        card_wins.push(score);
    }

    card_wins
}

fn score_cards(lines: &[String]) -> u32 {
    let card_wins = read_cards(lines);

    //let total_score = card_wins.iter().filter_map::<u32,_>(|&w| if w == 0 { None } else { Some(2_u32.pow(w - 1))}).sum::<u32>();
    let total_score = card_wins
        .iter()
        .filter(|&&w| w != 0)
        .map(|&w| 2_u32.pow(w - 1))
        .sum();

    println!("Total Score: {}", total_score);
    total_score
}

fn copy_cards(lines: &[String]) -> u32 {
    let card_wins = read_cards(lines);
    let num_card_games = card_wins.len();

    // Start with 1 of each card.
    let mut cards: Vec<u32> = Vec::new();
    for _i in 0..num_card_games {
        cards.push(1);
    }

    for i in 0..num_card_games {
        let wins = card_wins[i] as usize;
        if wins > 0 {
            let limit = min(i + wins, num_card_games);
            for j in i + 1..=limit {
                cards[j] += cards[i]; // We get n copies of the new cards.
            }
        }
    }

    let total_cards = cards.iter().sum();
    println!("Total Cards: {}", total_cards);
    total_cards
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

#[test]
fn test_prelim2() {
    let cards = copy_cards(&get_input("prelim.txt"));
    assert_eq!(cards, 30);
}

#[test]
fn test_part2() {
    let cards = copy_cards(&get_input("input.txt"));
    assert_eq!(cards, 10425665);
}

fn main() {
    score_cards(&get_input("prelim.txt"));
    score_cards(&get_input("input.txt"));
    copy_cards(&get_input("prelim.txt"));
    copy_cards(&get_input("input.txt"));
}

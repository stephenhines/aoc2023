use std::cmp::Ordering;
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    high_cards: [usize; 5],
    bid: u64,
    hand_str: String,
    card_vals_in_string_order: [usize; 5],
}

impl Hand {
    fn new(hand_str: &str, bid: u64, jokers: bool) -> Self {
        // Just zero initialize everything
        let mut hand = Hand {
            hand_type: HandType::HighCard,
            high_cards: [0; 5],
            bid,
            hand_str: hand_str.to_string(),
            card_vals_in_string_order: [0; 5],
        };

        // Just use simple 15 element array since J-A get marked as 11-14,
        // and we don't care about the nonexistent 0 and 1 entries.
        // Later we use 1 for Jokers.
        let mut card: [u8; 15] = [0; 15];
        let mut val;
        for (i, c) in hand_str.chars().enumerate() {
            match c {
                'T' => {
                    val = 10;
                }
                'J' => {
                    if jokers {
                        val = 1;
                    } else {
                        val = 11;
                    }
                }
                'Q' => {
                    val = 12;
                }
                'K' => {
                    val = 13;
                }
                'A' => {
                    val = 14;
                }
                '2'..='9' => {
                    val = c as usize - '0' as usize;
                }
                _ => {
                    panic! {"Error: unknown card \'{}\'", c};
                }
            }
            card[val] += 1;
            hand.card_vals_in_string_order[i] = val;
        }

        // No longer needs to be mutable.
        let card = card;
        let num_jokers = card[1];

        // Search for hands in decreasing order of importance
        for i in (2..15).rev() {
            if card[i] + num_jokers == 5 {
                hand.hand_type = HandType::FiveOfAKind;
                hand.high_cards[0] = i;
                return hand;
            }
        }

        for i in (2..15).rev() {
            if card[i] + num_jokers == 4 {
                hand.hand_type = HandType::FourOfAKind;
                hand.high_cards[0] = i;
                for j in (2..15).rev() {
                    if card[j] == 1 {
                        hand.high_cards[1] = j;
                        return hand;
                    }
                }
                panic! {"4 of a kind with no other cards! {}", hand_str};
            }
        }

        for i in (2..15).rev() {
            if card[i] + num_jokers == 3 {
                hand.high_cards[0] = i;
                for j in (2..15).rev() {
                    if card[j] == 2 && j != i {
                        hand.hand_type = HandType::FullHouse;
                        hand.high_cards[1] = j;
                        return hand;
                    } else if card[j] == 1 {
                        hand.hand_type = HandType::ThreeOfAKind;
                        hand.high_cards[1] = j;
                        for k in (2..15).rev() {
                            if card[k] == 1 && k != j {
                                hand.high_cards[2] = k;
                                return hand;
                            }
                        }
                        panic! {"Missing a 5th card for 3 of a kind! {}", hand_str};
                    }
                }
                panic! {"3 of a kind with no other cards! {}", hand_str};
            }
        }

        // Two pair and One pair
        for i in (2..15).rev() {
            if card[i] + num_jokers == 2 {
                let mut next_high_card = 0;
                hand.hand_type = HandType::OnePair;
                hand.high_cards[next_high_card] = i;
                next_high_card += 1;
                for j in (2..15).rev() {
                    if card[j] == 2 && j != i {
                        hand.hand_type = HandType::TwoPair;
                        hand.high_cards[1] = j;
                        for k in (2..15).rev() {
                            if card[k] == 1 {
                                hand.high_cards[2] = k;
                                return hand;
                            }
                        }
                        panic! {"Missing a 5th card for 2 pair! {}", hand_str};
                    } else if card[j] == 1 {
                        hand.high_cards[next_high_card] = j;
                        next_high_card += 1;
                        if next_high_card == 4 {
                            return hand;
                        }
                    }
                }
                panic! {"1 pair with not enough other cards! {}", hand_str};
            }
        }

        // We can't have any jokers if we reach this point.
        assert_eq!(num_jokers, 0);

        // High card
        let mut next_high_card = 0;
        hand.hand_type = HandType::HighCard;
        for i in (2..15).rev() {
            if card[i] == 1 {
                hand.high_cards[next_high_card] = i;
                next_high_card += 1;
            }
        }
        assert_eq!(next_high_card, 5);

        hand
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        //self.height.cmp(&other.height)
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        assert_eq!(self.hand_type, other.hand_type);

        /* Don't compare actual hands. It only wants the order of the cards!
        for i in 0..5 {
            if self.high_cards[i] > other.high_cards[i] {
                return Ordering::Greater;
            } else if self.high_cards[i] < other.high_cards[i] {
                return Ordering::Less;
            }
        }
        */
        for i in 0..5 {
            if self.card_vals_in_string_order[i] > other.card_vals_in_string_order[i] {
                return Ordering::Greater;
            } else if self.card_vals_in_string_order[i] < other.card_vals_in_string_order[i] {
                return Ordering::Less;
            }
        }

        panic!("Found equal hands in the input!");
        //return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_winnings(lines: &[String], jokers: bool) -> u64 {
    let mut hands = Vec::new();

    for line in lines {
        let toks: Vec<_> = line.split_whitespace().collect();
        let bid: u64 = toks[1].parse().unwrap();
        let hand_str = toks[0].trim();

        let hand = Hand::new(hand_str, bid, jokers);
        //println! {"hand: {:?}", hand};

        hands.push(hand);
    }

    hands.sort();
    //println! {"hands: {:?}", hands};
    /*for (i, hand) in hands.iter().enumerate() {
        println!{"hand {:3}: {:?}", i, hand};
    }*/

    let mut winnings = 0;
    let mut multiplier = 1;
    for hand in hands {
        winnings += multiplier * hand.bid;
        multiplier += 1;
    }

    println! {"Winnings: {}", winnings};
    winnings
}

#[test]
fn test_prelim() {
    let winnings = compute_winnings(&get_input("prelim.txt"), false);
    assert_eq!(winnings, 6440);
}

#[test]
fn test_part1() {
    let winnings = compute_winnings(&get_input("input.txt"), false);
    assert_eq!(winnings, 248569531);
}

#[test]
fn test_prelim2() {
    let winnings = compute_winnings(&get_input("prelim.txt"), true);
    assert_eq!(winnings, 5905);
}

#[test]
fn test_part2() {
    let winnings = compute_winnings(&get_input("input.txt"), true);
    assert_eq!(winnings, 250382098);
}

fn main() {
    compute_winnings(&get_input("prelim.txt"), false);
    compute_winnings(&get_input("input.txt"), false);
    compute_winnings(&get_input("prelim.txt"), true);
    compute_winnings(&get_input("input.txt"), true);
}

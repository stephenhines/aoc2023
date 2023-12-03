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

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn check_game(line: &String) -> u32 {
    let toks: Vec<&str> = line.split(":").collect();
    let game_number_toks: Vec<&str> = toks[0].split(" ").collect();
    let game_number = game_number_toks[1].parse::<u32>().unwrap();

    let games_toks: Vec<&str> = toks[1].trim().split(";").collect();
    for game in games_toks {
        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;
        let colors_toks: Vec<&str> = game.trim().split(",").collect();
        for color in colors_toks {
            let cube_toks: Vec<&str> = color.trim().split(" ").collect();
            //println!("cube_toks {:?}", cube_toks);
            match (cube_toks[0], cube_toks[1]) {
                (v, "red") => {
                    num_red = v.parse::<u32>().unwrap();
                }
                (v, "green") => {
                    num_green = v.parse::<u32>().unwrap();
                }
                (v, "blue") => {
                    num_blue = v.parse::<u32>().unwrap();
                }
                (_, _) => {
                    panic!("Unknown color combination: {}", color);
                }
            }
        }

        // Verify color maximums
        if num_red > MAX_RED {
            return 0;
        }
        if num_green > MAX_GREEN {
            return 0;
        }
        if num_blue > MAX_BLUE {
            return 0;
        }
    }

    game_number
}

fn check_games(lines: &Vec<String>) -> u32 {
    let mut games = 0;
    for line in lines {
        games += check_game(line);
    }
    println!("Games: {}", games);
    games
}

fn power(line: &String) -> u32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let toks: Vec<&str> = line.split(":").collect();
    //let game_number_toks: Vec<&str> = toks[0].split(" ").collect();
    //let game_number = game_number_toks[1].parse::<u32>().unwrap();

    let games_toks: Vec<&str> = toks[1].trim().split(";").collect();
    for game in games_toks {
        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;
        let colors_toks: Vec<&str> = game.trim().split(",").collect();
        for color in colors_toks {
            let cube_toks: Vec<&str> = color.trim().split(" ").collect();
            //println!("cube_toks {:?}", cube_toks);
            match (cube_toks[0], cube_toks[1]) {
                (v, "red") => {
                    num_red = v.parse::<u32>().unwrap();
                }
                (v, "green") => {
                    num_green = v.parse::<u32>().unwrap();
                }
                (v, "blue") => {
                    num_blue = v.parse::<u32>().unwrap();
                }
                (_, _) => {
                    panic!("Unknown color combination: {}", color);
                }
            }
        }

        // Find color maximums
        if num_red > max_red {
            max_red = num_red;
        }
        if num_green > max_green {
            max_green = num_green;
        }
        if num_blue > max_blue {
            max_blue = num_blue;
        }
    }

    max_red * max_green * max_blue
}

fn sum_power(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        sum += power(line);
    }
    println!("sum power: {}", sum);
    sum
}

#[test]
fn test_prelim() {
    let games = check_games(&get_input("prelim.txt"));
    assert_eq!(games, 8);
}

#[test]
fn test_part1() {
    let games = check_games(&get_input("input.txt"));
    assert_eq!(games, 3099);
}

#[test]
fn test_prelim2() {
    let sum = sum_power(&get_input("prelim.txt"));
    assert_eq!(sum, 2286);
}

#[test]
fn test_part2() {
    let sum = sum_power(&get_input("input.txt"));
    assert_eq!(sum, 72970);
}

fn main() {
    check_games(&get_input("prelim.txt"));
    check_games(&get_input("input.txt"));
    sum_power(&get_input("prelim.txt"));
    sum_power(&get_input("input.txt"));
}

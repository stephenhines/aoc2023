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
struct ElfMap {
    dest_start: u64,
    src_start: u64,
    range_len: u64,
}

trait Contains {
    fn contains(&self, val: u64) -> bool;
    fn last(&self) -> u64;
}

impl Contains for ElfMap {
    fn contains(&self, val: u64) -> bool {
        val >= self.src_start && val < self.src_start + self.range_len
    }

    fn last(&self) -> u64 {
        self.src_start + self.range_len
    }
}

fn map_elf_value(ranges: &Vec<ElfMap>, loc: u64) -> u64 {
    for range in ranges {
        if range.contains(loc) {
            return loc - range.src_start + range.dest_start;
        }
    }
    loc
}

fn get_lowest_location(lines: &[String], use_seed_ranges: bool) -> u64 {
    let mut seeds = Vec::new();

    let mut maps = Vec::new();
    for line in lines {
        let toks: Vec<&str> = line.split(":").collect();
        match toks[0] {
            "seeds" => {
                // Read the seed numbers
                let numbers: Vec<&str> = toks[1].split_whitespace().collect();
                for n in numbers {
                    seeds.push(n.parse::<u64>().unwrap());
                }
            }

            "seed-to-soil map"
            | "soil-to-fertilizer map"
            | "fertilizer-to-water map"
            | "water-to-light map"
            | "light-to-temperature map"
            | "temperature-to-humidity map"
            | "humidity-to-location map" => {
                maps.push(Vec::new());
            }

            "" => {}

            _ => {
                // Read the digits in
                let num_toks: Vec<&str> = line.split_whitespace().collect();
                assert_eq!(num_toks.len(), 3);
                let dest_start = num_toks[0].parse::<u64>().unwrap();
                let src_start = num_toks[1].parse::<u64>().unwrap();
                let range_len = num_toks[2].parse::<u64>().unwrap();
                let elf_map = ElfMap {
                    dest_start,
                    src_start,
                    range_len,
                };
                maps.last_mut().unwrap().push(elf_map);
            }
        }
    }

    //println! {"seeds: {:?}", seeds};

    // Sort each ElfMap by src_start.
    maps.iter_mut()
        .for_each(|m| m.sort_by(|a, b| a.src_start.cmp(&b.src_start)));

    let mut min_seed_loc = u64::MAX;

    if use_seed_ranges {
        assert_eq!(seeds.len() % 2, 0);
        for pairs in seeds.chunks(2) {
            let base_loc = pairs[0];
            let range = pairs[1];
            //println!{"Trying {} {}", base_loc, range};
            for i in 0..range {
                let mut loc = base_loc + i;
                for map in &maps {
                    loc = map_elf_value(map, loc);
                }

                min_seed_loc = std::cmp::min(min_seed_loc, loc);
            }
            //println! {"min_seed_loc so far: {}", min_seed_loc};
        }
    } else {
        for mut loc in seeds {
            for map in &maps {
                loc = map_elf_value(map, loc);
            }

            min_seed_loc = std::cmp::min(min_seed_loc, loc);
        }
    }

    println! {"Location: {}", min_seed_loc};
    min_seed_loc
}

#[test]
fn test_prelim() {
    let loc = get_lowest_location(&get_input("prelim.txt"), false);
    assert_eq!(loc, 35);
}

#[test]
fn test_part1() {
    let loc = get_lowest_location(&get_input("input.txt"), false);
    assert_eq!(loc, 910845529);
}

#[test]
fn test_prelim2() {
    let loc = get_lowest_location(&get_input("prelim.txt"), true);
    assert_eq!(loc, 46);
}

// Too slow to be part of automated testing
/*#[test]
fn test_part2() {
    let loc = get_lowest_location(&get_input("input.txt"), true);
    assert_eq!(loc, 77435348);
}*/

fn main() {
    get_lowest_location(&get_input("prelim.txt"), false);
    get_lowest_location(&get_input("input.txt"), false);
    get_lowest_location(&get_input("prelim.txt"), true);
    // Too slow to be run normally
    //get_lowest_location(&get_input("input.txt"), true);
}

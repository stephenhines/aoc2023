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

#[derive(Debug, PartialEq)]
enum ParseState {
    ReadSeeds,
    ReadSeedToSoilMap,
    ReadSoilToFertilizerMap,
    ReadFertilizerToWaterMap,
    ReadWaterToLightMap,
    ReadLightToTemperatureMap,
    ReadTemperatureToHumidityMap,
    ReadHumidityToLocationMap,
}

fn map_elf_value(ranges: &Vec<ElfMap>, loc: u64) -> u64 {
    for range in ranges {
        let range_start = range.src_start;
        let range_end = range.src_start + range.range_len;
        if loc >= range_start && loc < range_end {
            return loc - range_start + range.dest_start;
        }
    }
    loc
}

fn get_lowest_location(lines: &[String]) -> u64 {
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil_map: Vec<ElfMap> = Vec::new();
    let mut soil_to_fertilizer_map: Vec<ElfMap> = Vec::new();
    let mut fertilizer_to_water_map: Vec<ElfMap> = Vec::new();
    let mut water_to_light_map: Vec<ElfMap> = Vec::new();
    let mut light_to_temperature_map: Vec<ElfMap> = Vec::new();
    let mut temperature_to_humidity_map: Vec<ElfMap> = Vec::new();
    let mut humidity_to_location_map: Vec<ElfMap> = Vec::new();

    let mut parse_state = ParseState::ReadSeeds;
    for line in lines {
        let toks: Vec<&str> = line.split(":").collect();
        match toks[0] {
            "seeds" => {
                assert_eq!(parse_state, ParseState::ReadSeeds);
                // Read the seed numbers
                let numbers: Vec<&str> = toks[1].split_whitespace().collect();
                for n in numbers {
                    seeds.push(n.parse::<u64>().unwrap());
                }
            }
            "seed-to-soil map" => {
                assert_eq!(parse_state, ParseState::ReadSeeds);
                parse_state = ParseState::ReadSeedToSoilMap;
            }
            "soil-to-fertilizer map" => {
                assert_eq!(parse_state, ParseState::ReadSeedToSoilMap);
                parse_state = ParseState::ReadSoilToFertilizerMap;
            }
            "fertilizer-to-water map" => {
                assert_eq!(parse_state, ParseState::ReadSoilToFertilizerMap);
                parse_state = ParseState::ReadFertilizerToWaterMap;
            }
            "water-to-light map" => {
                assert_eq!(parse_state, ParseState::ReadFertilizerToWaterMap);
                parse_state = ParseState::ReadWaterToLightMap;
            }
            "light-to-temperature map" => {
                assert_eq!(parse_state, ParseState::ReadWaterToLightMap);
                parse_state = ParseState::ReadLightToTemperatureMap;
            }
            "temperature-to-humidity map" => {
                assert_eq!(parse_state, ParseState::ReadLightToTemperatureMap);
                parse_state = ParseState::ReadTemperatureToHumidityMap;
            }
            "humidity-to-location map" => {
                assert_eq!(parse_state, ParseState::ReadTemperatureToHumidityMap);
                parse_state = ParseState::ReadHumidityToLocationMap;
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
                match parse_state {
                    ParseState::ReadSeedToSoilMap => {
                        seed_to_soil_map.push(elf_map);
                    }
                    ParseState::ReadSoilToFertilizerMap => {
                        soil_to_fertilizer_map.push(elf_map);
                    }
                    ParseState::ReadFertilizerToWaterMap => {
                        fertilizer_to_water_map.push(elf_map);
                    }
                    ParseState::ReadWaterToLightMap => {
                        water_to_light_map.push(elf_map);
                    }
                    ParseState::ReadLightToTemperatureMap => {
                        light_to_temperature_map.push(elf_map);
                    }
                    ParseState::ReadTemperatureToHumidityMap => {
                        temperature_to_humidity_map.push(elf_map);
                    }
                    ParseState::ReadHumidityToLocationMap => {
                        humidity_to_location_map.push(elf_map);
                    }
                    ParseState::ReadSeeds => {
                        panic!("Invalid state: {:?}", parse_state);
                    }
                }
            }
        }
    }

    //println! {"seeds: {:?}", seeds};

    let mut min_seed_loc = seeds[0];
    let mut maps = Vec::new();
    maps.push(&seed_to_soil_map);
    maps.push(&soil_to_fertilizer_map);
    maps.push(&fertilizer_to_water_map);
    maps.push(&water_to_light_map);
    maps.push(&light_to_temperature_map);
    maps.push(&temperature_to_humidity_map);
    maps.push(&humidity_to_location_map);

    for mut loc in seeds {
        for map in &maps {
            loc = map_elf_value(map, loc);
        }

        min_seed_loc = std::cmp::min(min_seed_loc, loc);
    }

    println! {"Location: {}", min_seed_loc};
    min_seed_loc
}

#[test]
fn test_prelim() {
    let loc = get_lowest_location(&get_input("prelim.txt"));
    assert_eq!(loc, 35);
}

#[test]
fn test_part1() {
    let loc = get_lowest_location(&get_input("input.txt"));
    assert_eq!(loc, 910845529);
}

fn main() {
    get_lowest_location(&get_input("prelim.txt"));
    get_lowest_location(&get_input("input.txt"));
}

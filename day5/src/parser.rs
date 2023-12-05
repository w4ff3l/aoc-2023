use std::{collections::HashMap, u32};

pub struct SeedsAndMaps {
    pub seeds: Vec<u32>,
    pub maps: Vec<HashMap<u32, u32>>,
}

pub fn parse_input(input: &str) -> SeedsAndMaps {
    let mut maps: Vec<HashMap<u32, u32>> = Vec::new();

    let split_by_empty_line = input.split("\n\n").collect::<Vec<&str>>();

    let seeds_str = split_by_empty_line[0].split(": ").collect::<Vec<&str>>()[1];
    let seeds = parse_seeds(seeds_str);

    println!("Parsing maps");
    for map_string in split_by_empty_line.iter().skip(1) {
        maps.push(parse_map(map_string));
        println!("Parsed map");
    }

    SeedsAndMaps { seeds, maps }
}

fn parse_map(seed_to_soil_str: &str) -> HashMap<u32, u32> {
    let mut seed_to_soil_map: HashMap<u32, u32> = HashMap::new();

    for line in seed_to_soil_str.lines().skip(1) {
        let seed_to_soil = line
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let destination_range_start = seed_to_soil[0];
        let source_range_start = seed_to_soil[1];
        let range = seed_to_soil[2];

        let destination_range = destination_range_start..destination_range_start + range;
        let source_range = source_range_start..source_range_start + range;

        for (d, s) in destination_range.zip(source_range) {
            seed_to_soil_map.insert(s, d);
        }
    }

    seed_to_soil_map
}

fn parse_seeds(seeds_str: &str) -> Vec<u32> {
    seeds_str
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests_parser {
    use crate::parser::parse_map;

    use super::parse_input;

    #[test]
    fn parses_seeds_and_maps() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 30\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     \n\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     \n\
                     water-to-light map:\n\
                     88 18 7\n\
                     \n\
                     light-to-temperature map:\n\
                     45 77 23\n\
                     \n\
                     temperature-to-humidity map:\n\
                     0 69 1\n\
                     \n\
                     humidity-to-location map:\n\
                     60 56 37";

        let result = parse_input(input);

        assert_eq!(result.seeds.len(), 4);
        assert_eq!(result.maps.len(), 7);
        assert_eq!(*result.maps[0].get(&79).unwrap(), 81);
    }

    #[test]
    fn parses_seed_to_soil_map() {
        let input = "seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 3";

        let result = parse_map(input);

        assert_eq!(result.get(&98).unwrap(), &50);
        assert_eq!(result.get(&99).unwrap(), &51);
        assert_eq!(result.get(&50).unwrap(), &52);
        assert_eq!(result.get(&51).unwrap(), &53);
    }
}

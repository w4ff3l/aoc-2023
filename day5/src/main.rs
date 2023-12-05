mod parser;

use std::{fs::read_to_string, i64};

use crate::parser::parse_input;

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let result_challenge_1 = calculate_result_challenge_1(&input);
    println!("Result for challenge 1: {}", result_challenge_1);
}

fn calculate_result_challenge_1(input: &str) -> i64 {
    let seeds_and_maps = parse_input(input);

    let mut locations: Vec<i64> = Vec::new();

    let seeds = seeds_and_maps.seeds;
    let maps = &seeds_and_maps.maps;

    for seed in seeds {
        let mut current = seed;

        for map in maps.iter() {
            'outer: for map_entry in map.map_entries.iter() {
                if is_in_source(current, map_entry) {
                    current = map_entry.destination_start + current - map_entry.source_start;
                    break 'outer;
                }
            }

        }
        locations.push(current);
    }

    *locations.iter().min().unwrap()
}

fn is_in_source(current: i64, map_entry: &parser::MapEntry) -> bool {
    map_entry.source_start <= current && map_entry.source_start + map_entry.range >= current
}

#[cfg(test)]
mod tests_c1 {
    use crate::calculate_result_challenge_1;

    #[test]
    fn challenge_1_example_test() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15\n\
                     \n\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     0 11 42\n\
                     42 0 7\n\
                     57 7 4\n\
                     \n\
                     water-to-light map:\n\
                     88 18 7\n\
                     18 25 70\n\
                     \n\
                     light-to-temperature map:\n\
                     45 77 23\n\
                     81 45 19\n\
                     68 64 13\n\
                     \n\
                     temperature-to-humidity map:\n\
                     0 69 1\n\
                     1 0 69\n\
                     \n\
                     humidity-to-location map:\n\
                     60 56 37\n\
                     56 93 4";

        let result = calculate_result_challenge_1(&input);

        assert_eq!(result, 35);
    }
}

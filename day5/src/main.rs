mod parser;

use std::{fs::read_to_string, i64};

use parser::{parse_maps, parse_seeds_c1, parse_seeds_c2, Map};

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let result_challenge_1 = calculate_result_challenge_1(&input);
    println!("Result for challenge 1: {}", result_challenge_1);
    let result_challenge_2 = calculate_result_challenge_2(&input);
    println!("Result for challenge 2: {}", result_challenge_2);
}

fn calculate_result_challenge_2(input: &str) -> i64 {
    let split_by_empty_line = input.split("\n\n").collect::<Vec<&str>>();
    let seeds_str = split_by_empty_line[0].split(": ").collect::<Vec<&str>>()[1];

    let seed_ranges = parse_seeds_c2(seeds_str);
    let seed_ranges_merged = merge_ranges(seed_ranges);

    let maps = parse_maps(input);

    let mut locations: Vec<i64> = Vec::new();
    let mut counter: i64 = 0;

    let amount = calculate_amount_of_seeds(seed_ranges_merged.clone());
    println!("{:?}", amount);

    for (seed_start, seed_end) in seed_ranges_merged {
        for seed in seed_start..seed_end {
            if counter % 100000 == 0 {
                println!("Progress: {:?}", counter);
            }

            let mut current = seed;

            for map in maps.iter() {
                'outer: for map_entry in map.map_entries.iter() {
                    if is_in_source(current, map_entry) {
                        current = map_entry.destination_start + current - map_entry.source_start;
                        break 'outer;
                    }
                }
            }

            counter += 1;

            locations.push(current);
        }
    }

    *locations.iter().min().unwrap()
}

fn merge_ranges(mut seed_ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // Sort
    seed_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // Push first element into stack
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    merged_ranges.push(*seed_ranges.first().unwrap());

    for seed_range in seed_ranges.iter().skip(1) {
        // Update range on top of stack if ranges overlap
        // Simply push range if ranges do not overlap
        if merged_ranges.last().unwrap().1 > seed_range.0 {
            let mut range_to_update = merged_ranges.pop().unwrap();
            range_to_update.1 = seed_range.1;
            merged_ranges.push(range_to_update);
        } else {
            merged_ranges.push(*seed_range);
        }
    }

    merged_ranges
}

fn calculate_amount_of_seeds(seeds: Vec<(i64, i64)>) -> i64 {
    let mut amount: i64 = 0;

    for (s, end) in seeds {
        amount += end - s;
    }

    amount
}

fn calculate_result_challenge_1(input: &str) -> i64 {
    let split_by_empty_line = input.split("\n\n").collect::<Vec<&str>>();
    let seeds_str = split_by_empty_line[0].split(": ").collect::<Vec<&str>>()[1];

    let seeds = parse_seeds_c1(seeds_str);
    let maps = parse_maps(input);

    find_nearest_locations(seeds, maps)
}

fn find_nearest_locations(seeds: Vec<i64>, maps: Vec<Map>) -> i64 {
    let mut locations: Vec<i64> = Vec::new();

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
    use crate::{calculate_result_challenge_1, merge_ranges};

    #[test]
    fn merges_ranges() {
        let input = vec![(1, 2), (3, 5), (4, 9)];

        let result = merge_ranges(input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[1], (3, 9));
    }

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

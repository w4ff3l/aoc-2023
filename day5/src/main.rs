mod parser;

use std::fs::read_to_string;

use crate::parser::parse_input;

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let result_challenge_1 = calculate_result_challenge_1(&input);
    println!("Result for challenge 1: {}", result_challenge_1);
}

fn calculate_result_challenge_1(input: &str) -> u32 {
    let seeds_and_maps = parse_input(input);
    println!("Done Parsing");

    let mut locations: Vec<u32> = Vec::new();

    let seeds = seeds_and_maps.seeds;
    let maps = &seeds_and_maps.maps;

    for seed in seeds {
        let mut current = match maps[0].get(&seed) {
            Some(value) => value,
            None => &seed,
        };

        for map in maps.iter().skip(1) {
            if let Some(value) = map.get(current) {
                current = value;
            }
        }

        locations.push(*current);
        println!("Pushed locations for seed");
    }

    *locations.iter().min().unwrap()
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

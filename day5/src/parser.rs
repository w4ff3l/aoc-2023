#[derive(Debug)]
pub struct Map {
    pub map_entries: Vec<MapEntry>,
}

#[derive(Debug)]
pub struct MapEntry {
    pub destination_start: i64,
    pub source_start: i64,
    pub range: i64,
}

pub struct SeedsAndMaps {
    pub seeds: Vec<i64>,
    pub maps: Vec<Map>,
}

pub fn parse_input(input: &str) -> SeedsAndMaps {
    let mut maps: Vec<Map> = Vec::new();

    let split_by_empty_line = input.split("\n\n").collect::<Vec<&str>>();

    let seeds_str = split_by_empty_line[0].split(": ").collect::<Vec<&str>>()[1];
    let seeds = parse_seeds(seeds_str);

    for map_string in split_by_empty_line.iter().skip(1) {
        maps.push(parse_map(map_string));
    }

    SeedsAndMaps { seeds, maps }
}

fn parse_map(seed_to_soil_str: &str) -> Map {
    let mut map_entries: Vec<MapEntry> = Vec::new();

    for line in seed_to_soil_str.lines().skip(1) {
        let seed_to_soil = line
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let destination_start = seed_to_soil[0];
        let source_start = seed_to_soil[1];
        let range = seed_to_soil[2];

        map_entries.push(MapEntry {
            destination_start,
            source_start,
            range,
        })
    }

    Map { map_entries }
}

fn parse_seeds(seeds_str: &str) -> Vec<i64> {
    seeds_str
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests_parser {
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
    }
}

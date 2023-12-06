use std::{fs::read_to_string, u64};

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let result_challenge_1 = calculate_result_challenge_1(&input);
    println!("Result for challenge 1: {}", result_challenge_1);
    let result_challenge_2 = calculate_result_challenge_2(&input);
    println!("Result for challenge 2: {}", result_challenge_2);
}

fn calculate_result_challenge_2(input: &str) -> u64 {
    let race = parse_race(input);

    println!("Race start");
    let max_time = race.time;
    let max_distance = race.distance;

    let mut c_ways_to_win = 0;

    for hold in 0..max_time {
        let distance = hold * (max_time - hold);
        if distance > max_distance {
            if hold % 10000 == 0 {
                println!("Progress: {:?}/{:?}", hold, max_time);
            }
            c_ways_to_win += 1;
        }
    }

    c_ways_to_win
}

fn calculate_result_challenge_1(input: &str) -> u64 {
    let races = parse_races(input);
    let mut ways_to_win_per_race: Vec<u64> = Vec::new();

    for race in races {
        println!("New Race");
        let max_time = race.time;
        let max_distance = race.distance;

        let mut c_ways_to_win = 0;

        for hold in 0..max_time {
            let distance = hold * (max_time - hold);
            if distance > max_distance {
                println!("Hold/Distance: {:?}/{:?}", hold, distance);
                c_ways_to_win += 1;
            }
        }

        println!("pushing {:?}", c_ways_to_win);
        println!();
        ways_to_win_per_race.push(c_ways_to_win);
    }

    ways_to_win_per_race.iter().product()
}

struct Race {
    time: u64,
    distance: u64,
}

fn parse_race(input: &str) -> Race {
    let split_time_distance = input.split("\n").collect::<Vec<&str>>();

    let time = split_time_distance[0].split("Time:").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = split_time_distance[1]
        .split("Distance:")
        .collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    Race { time, distance }
}

fn parse_races(input: &str) -> Vec<Race> {
    let split_time_distance = input.split("\n").collect::<Vec<&str>>();

    let times = split_time_distance[0].split("Time:").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distances = split_time_distance[1]
        .split("Distance:")
        .collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut races: Vec<Race> = Vec::new();
    for (time, distance) in times.iter().zip(distances) {
        races.push(Race {
            time: *time,
            distance,
        })
    }

    races
}

#[cfg(test)]
mod tests_c1 {
    use crate::{calculate_result_challenge_1, parse_races};

    #[test]
    fn challenge_1_example_test() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";

        let result = calculate_result_challenge_1(input);

        assert_eq!(result, 288);
    }

    #[test]
    fn parses_input() {
        let input = "Time:     2\n\
                     Distance: 10";

        let result = parse_races(input);

        assert_eq!(result[0].time, 2);
        assert_eq!(result[0].distance, 10);
    }
}

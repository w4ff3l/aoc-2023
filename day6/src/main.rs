use std::{fs::read_to_string, i32};

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let result_challenge_1 = calculate_result_challenge_1(&input);
    println!("Result for challenge 1: {}", result_challenge_1);
}

fn calculate_result_challenge_1(input: &str) -> i32 {
    let races = parse_races(input);
    let mut ways_to_win_per_race: Vec<i32> = Vec::new();

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
    time: i32,
    distance: i32,
}
fn parse_races(input: &str) -> Vec<Race> {
    let split_time_distance = input.split("\n").collect::<Vec<&str>>();

    let times = split_time_distance[0].split("Time:").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distances = split_time_distance[1]
        .split("Distance:")
        .collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

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
    use crate::{parse_races, calculate_result_challenge_1};

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

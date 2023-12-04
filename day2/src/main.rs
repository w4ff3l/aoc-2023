mod model;
mod parser;

use std::fs::read_to_string;

use model::Color;
use model::Cubes;
use model::Pull;
use parser::parse_games;

const MAX_CUBES_RED: Cubes = Cubes {
    quantity: 12,
    color: Color::Red,
};
const MAX_CUBES_GREEN: Cubes = Cubes {
    quantity: 13,
    color: Color::Green,
};
const MAX_CUBES_BLUE: Cubes = Cubes {
    quantity: 14,
    color: Color::Blue,
};

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let accumulated_games = accumulate_possible_games_c1(&input);
    println!("Accumulated possible games: {}", accumulated_games);

    let power_of_minimum_cubes = accumulate_power_minimum_cubes(&input);
    println!(
        "Accumulated power of minimal cubes: {}",
        power_of_minimum_cubes
    );
}

fn accumulate_power_minimum_cubes(input: &str) -> i32 {
    let games = parse_games(input);

    let mut accumulated_power = 0;

    // Determine power of minimal needed bag configuration such that a game is possible.
    for game in games {
        let max_of_red_cubes = game.max_of_cubes(Color::Red);
        let max_of_green_cubes = game.max_of_cubes(Color::Green);
        let max_of_blue_cubes = game.max_of_cubes(Color::Blue);

        accumulated_power += max_of_red_cubes * max_of_green_cubes * max_of_blue_cubes;
    }

    accumulated_power
}

fn accumulate_possible_games_c1(input: &str) -> i32 {
    let games = parse_games(input);

    // Determine which games are possible evaluated by the given pulls.
    // The bag has an initial configuration:
    // -> 12 Red, 13 Green, 14 Blue
    //
    // Constraints:
    // - Number of cubes of one color exceeds max. amount given in the bags initial configuration.

    let mut adder = 0;

    for game in games {
        if game
            .set_of_pulls
            .iter()
            .all(|pull| !is_too_many_cubes(pull.clone()))
        {
            adder += game.index;
        }
    }

    adder
}

fn is_too_many_cubes(pull: Pull) -> bool {
    let red_cubes = pull.get_num_red_cubes();
    let green_cubes = pull.get_num_green_cubes();
    let blue_cubes = pull.get_num_blue_cubes();

    if red_cubes > MAX_CUBES_RED.quantity
        || green_cubes > MAX_CUBES_GREEN.quantity
        || blue_cubes > MAX_CUBES_BLUE.quantity
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{accumulate_possible_games_c1, accumulate_power_minimum_cubes};

    #[test]
    fn aoc_example_test_c1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = accumulate_possible_games_c1(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn aoc_example_test_c2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = accumulate_power_minimum_cubes(input);

        assert_eq!(result, 2286);
    }
}

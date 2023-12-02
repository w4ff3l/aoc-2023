mod model;
mod parser;

use std::fs::read_to_string;

use parser::parse_games;

fn main() {
    let input = read_to_string("puzzle-input").unwrap();
    let games = parse_games(&input);

    // Determine which games are possible evaluated by the given pulls.
    // The bag has an initial configuration:
    // -> 12 Red, 13 Green, 14 Blue
    //
    // Constraints:
    // - Number of cubes of one color exceeds max. amount given in the bags initial configuration.
}


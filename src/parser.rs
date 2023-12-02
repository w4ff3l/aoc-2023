use std::str::FromStr;

use crate::model::{Color, Cubes, Game, Pull};

pub fn parse_games(games: &str) -> Vec<Game> {
    games.lines().map(parse_game).collect::<Vec<Game>>()
}

fn parse_game(game: &str) -> Game {
    let split_game_pulls = game.split(": ").collect::<Vec<&str>>();

    // extract index
    let index_str = split_game_pulls[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1];
    let index = index_str.parse::<i32>().unwrap();

    // extract pulls
    let pulls_strs = split_game_pulls[1].split("; ").collect::<Vec<&str>>();
    let set_of_pulls = pulls_strs.iter().map(parse_pull).collect::<Vec<Pull>>();

    Game {
        index,
        set_of_pulls,
    }
}

fn parse_pull(pull: &&str) -> Pull {
    let split_cubes = pull
        .split(", ")
        .map(|cubes| cubes.trim())
        .collect::<Vec<&str>>();
    let set_of_cubes = split_cubes
        .iter()
        .map(|cubes| parse_cubes(cubes))
        .collect::<Vec<Cubes>>();

    Pull { set_of_cubes }
}

fn parse_cubes(cubes: &str) -> Cubes {
    let split = cubes.split_whitespace().collect::<Vec<&str>>();
    let quantity = split[0].parse::<i32>().unwrap();
    let color = Color::from_str(split[1]).unwrap();

    Cubes { quantity, color }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        model::{Color, Cubes, Game, Pull},
        parser::{parse_cubes, parse_game, parse_games, parse_pull},
    };

    #[test]
    fn parses_cubes_correctly() {
        let raw: &str = "3 blue";
        let expected = Cubes {
            quantity: 3,
            color: Color::Blue,
        };

        let result = parse_cubes(raw);

        assert_eq!(result, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn parses_pull_correctly() {
        let raw: &str = "3 blue, 4 red";
        let cubes1 = Cubes { quantity: 3, color: Color::Blue, };
        let cubes2 = Cubes { quantity: 4, color: Color::Red, };
        let set_of_cubes = vec![cubes1, cubes2];
        let expected = Pull { set_of_cubes };

        let result = parse_pull(&raw);

        assert_eq!(result, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn parses_game_correctly() {
        // given
        let raw: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let cubes_pull11 = Cubes { quantity: 3, color: Color::Blue, };
        let cubes_pull12 = Cubes { quantity: 4, color: Color::Red, };
        let set_of_cubes1 = vec![cubes_pull11, cubes_pull12];
        let pull1 = Pull{ set_of_cubes: set_of_cubes1 };

        let cubes_pull21 = Cubes { quantity: 1, color: Color::Red, };
        let cubes_pull22 = Cubes { quantity: 2, color: Color::Green, };
        let cubes_pull23 = Cubes { quantity: 6, color: Color::Blue, };
        let set_of_cubes2 = vec![cubes_pull21, cubes_pull22, cubes_pull23];
        let pull2 = Pull{ set_of_cubes: set_of_cubes2 };

        let cubes_pull31 = Cubes { quantity: 2, color: Color::Green, };
        let set_of_cubes3 = vec![cubes_pull31];
        let pull3 = Pull{ set_of_cubes: set_of_cubes3 };
        
        let index = 1;
        let set_of_pulls = vec![pull1, pull2, pull3];

        let expected = Game { index, set_of_pulls };
        
        // when
        let result = parse_game(raw);

        // then
        assert_eq!(result, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn parses_games_correctly() {
        // given
        let raw: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                         Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        // game 1
        let cubes_pull11 = Cubes { quantity: 3, color: Color::Blue, };
        let cubes_pull12 = Cubes { quantity: 4, color: Color::Red, };
        let set_of_cubes1 = vec![cubes_pull11, cubes_pull12];
        let pull1 = Pull{ set_of_cubes: set_of_cubes1 };

        let cubes_pull21 = Cubes { quantity: 1, color: Color::Red, };
        let cubes_pull22 = Cubes { quantity: 2, color: Color::Green, };
        let cubes_pull23 = Cubes { quantity: 6, color: Color::Blue, };
        let set_of_cubes2 = vec![cubes_pull21, cubes_pull22, cubes_pull23];
        let pull2 = Pull{ set_of_cubes: set_of_cubes2 };

        let cubes_pull31 = Cubes { quantity: 2, color: Color::Green, };
        let set_of_cubes3 = vec![cubes_pull31];
        let pull3 = Pull{ set_of_cubes: set_of_cubes3 };
        
        let index = 1;
        let set_of_pulls = vec![pull1, pull2, pull3];

        let game1 = Game { index, set_of_pulls: set_of_pulls.to_vec() };
        let game2 = Game { index, set_of_pulls: set_of_pulls.to_vec() };

        let expected = vec![game1, game2];
        
        // when
        let result = parse_games(raw);

        // then
        assert_eq!(result, expected);
    }
}

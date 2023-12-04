use std::{i32, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Color, Self::Err> {
        match str {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Unknown Color."),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cubes {
    pub quantity: i32,
    pub color: Color,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pull {
    pub set_of_cubes: Vec<Cubes>,
}

impl Pull {
    pub fn get_num_red_cubes(&self) -> i32 {
        self.get_num_cubes(Color::Red)
    }

    pub fn get_num_green_cubes(&self) -> i32 {
        self.get_num_cubes(Color::Green)
    }

    pub fn get_num_blue_cubes(&self) -> i32 {
        self.get_num_cubes(Color::Blue)
    }

    fn get_num_cubes(&self, color: Color) -> i32 {
        let set = &self.set_of_cubes;
        for cubes in set {
            if cubes.color == color {
                return cubes.quantity;
            }
        }
        0
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub index: i32,
    pub set_of_pulls: Vec<Pull>,
}

impl Game {
    pub fn max_of_cubes(&self, color: Color) -> i32 {
        let mut max_cubes = 0;

        for pull in &self.set_of_pulls {
            let cubes = match color {
                Color::Red => pull.get_num_red_cubes(),
                Color::Green => pull.get_num_green_cubes(),
                Color::Blue => pull.get_num_blue_cubes(),
            };

            if cubes > max_cubes {
                max_cubes = cubes;
            }
        }

        max_cubes
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Cubes, Pull, Game};

    #[test]
    #[rustfmt::skip]
    fn returns_correct_color_of_cubes_from_pull() {
        let given_red_cubes = Cubes { quantity: 1, color: Color::Red, };
        let given_green_cubes = Cubes { quantity: 2, color: Color::Green, };
        let given_blue_cubes = Cubes { quantity: 3, color: Color::Blue, };
        let set_of_cubes = vec![ given_red_cubes.clone(), given_green_cubes.clone(), given_blue_cubes.clone(), ];
        let pull = Pull { set_of_cubes };

        let red_cubes = pull.get_num_red_cubes();
        let green_cubes = pull.get_num_green_cubes();
        let blue_cubes = pull.get_num_blue_cubes();

        assert_eq!(red_cubes, given_red_cubes.quantity);
        assert_eq!(green_cubes, given_green_cubes.quantity);
        assert_eq!(blue_cubes, given_blue_cubes.quantity);
    }

    #[test]
    #[rustfmt::skip]
    fn calculates_maximum_cubes_in_game_correctly() {
        // given
        let expected_red = 5;
        let expected_green = 10;
        let expected_blue = 20;

        let given_red_cubes1 = Cubes { quantity: expected_red, color: Color::Red, };
        let given_green_cubes1 = Cubes { quantity: 2, color: Color::Green, };
        let given_blue_cubes1 = Cubes { quantity: 3, color: Color::Blue, };
        let set_of_cubes1 = vec![ given_red_cubes1.clone(), given_green_cubes1.clone(), given_blue_cubes1.clone(), ];
        let pull1 = Pull { set_of_cubes: set_of_cubes1 };

        let given_red_cubes2 = Cubes { quantity: 1, color: Color::Red, };
        let given_green_cubes2 = Cubes { quantity: expected_green, color: Color::Green, };
        let given_blue_cubes2 = Cubes { quantity: expected_blue, color: Color::Blue, };
        let set_of_cubes2 = vec![ given_red_cubes2.clone(), given_green_cubes2.clone(), given_blue_cubes2.clone(), ];
        let pull2 = Pull { set_of_cubes: set_of_cubes2 };

        let game = Game { index: 1, set_of_pulls: vec![pull1, pull2]  };
        
        // when
        let result_red = game.max_of_cubes(Color::Red);
        let result_green = game.max_of_cubes(Color::Green);
        let result_blue = game.max_of_cubes(Color::Blue);

        // then
        assert_eq!(result_red, expected_red);
        assert_eq!(result_green, expected_green);
        assert_eq!(result_blue, expected_blue);
    }
}

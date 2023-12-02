use std::{str::FromStr, i32};

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


#[cfg(test)]
mod tests {
    use super::{Pull, Cubes, Color};

    #[test]
    fn returns_correct_color_of_cubes_from_pull() {
        let given_red_cubes = Cubes { quantity: 1, color: Color::Red };
        let given_green_cubes = Cubes { quantity: 2, color: Color::Green };
        let given_blue_cubes = Cubes { quantity: 3, color: Color::Blue };
        let set_of_cubes = vec![given_red_cubes.clone(), given_green_cubes.clone(), given_blue_cubes.clone()];
        let pull = Pull { set_of_cubes };

        let red_cubes = pull.get_num_red_cubes();
        let green_cubes = pull.get_num_green_cubes();
        let blue_cubes = pull.get_num_blue_cubes();

        assert_eq!(red_cubes, given_red_cubes.quantity);
        assert_eq!(green_cubes, given_green_cubes.quantity);
        assert_eq!(blue_cubes, given_blue_cubes.quantity);
    }
}

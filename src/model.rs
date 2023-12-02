use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
pub struct Game {
    pub index: i32,
    pub set_of_pulls: Vec<Pull>,
}


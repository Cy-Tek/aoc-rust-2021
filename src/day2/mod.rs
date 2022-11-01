use std::str::FromStr;
use crate::utils::read_input_from_string;

#[derive(Default)]
pub struct Position {
    pub distance: i64,
    pub depth: i64,
    pub aim: i64,
}

pub struct Instruction {
    distance: i64,
    direction: Direction,
}

pub enum Direction {
    Down,
    Forward,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err("Could not convert string to Direction".to_string()),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let direction = match split.next() {
            Some(text) => text.parse::<Direction>()?,
            None => return Err("Invalid arguments: No direction found".to_string()),
        };
        let distance = match split.next() {
            Some(text) => text.parse::<i64>().map_err(|e| e.to_string())?,
            None => return Err("Invalid arguments: Found a direction, but no distance".to_string()),
        };

        Ok(Instruction {
            direction,
            distance,
        })
    }
}

pub fn part_1(input: &str) -> i64 {
    let mut pos = Position::default();
    let inputs = read_input_from_string::<Instruction>(input);
    for Instruction { distance, direction } in inputs {
        match direction {
            Direction::Forward => pos.distance += distance,
            Direction::Up => pos.depth -= distance,
            Direction::Down => pos.depth += distance,
        }
    }

    pos.depth * pos.distance
}

pub fn part_2(input: &str) -> i64 {
    let mut pos = Position::default();
    let inputs = read_input_from_string::<Instruction>(input);
    for Instruction { distance, direction } in inputs {
        match direction {
            Direction::Forward => {
                pos.distance += distance;
                pos.depth += distance * pos.aim;
            },
            Direction::Up => pos.aim -= distance,
            Direction::Down => pos.aim += distance,
        }
    }

    pos.depth * pos.distance
}
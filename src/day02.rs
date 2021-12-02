use std::{
    io::{self, BufRead},
    str::FromStr,
};

use color_eyre::eyre::{eyre, Result};

pub fn parse_input() -> Vec<Direction> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

#[derive(Debug, Default, PartialEq)]
pub struct Location {
    horiz: i64,
    depth: i64,
}

pub enum Direction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Direction {
    type Err = color_eyre::eyre::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        match parts.next() {
            Some("forward") => Ok(Direction::Forward(parts.next()?.parse()?)),
            Some("down") => Ok(Direction::Down(parts.next()?.parse()?)),
            Some("up") => Ok(Direction::Up(parts.next()?.parse()?)),
            _ => Err(eyre!("invalid direction: {}", s)),
        }
    }
}

use std::fs;
use std::path;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}
impl FromStr for Command {
    type Err = ();
    fn from_str(string: &str) -> Result<Command, Self::Err> {
        let capture = COMMAND_REGEX.captures(&string).unwrap();
        let key = capture.get(1).unwrap().as_str();
        let number = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        match key {
            "forward" => Ok(Command::Forward(number)),
            "up" => Ok(Command::Up(number)),
            "down" => Ok(Command::Down(number)),
            _ => Err(()),
        }
    }
}

const COMMAND_REGEX_STRING: &str = "(\\w+) (\\d+)";
lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(COMMAND_REGEX_STRING).unwrap();
    static ref INPUT: Vec<Command> = {
        let input = fs::read_to_string(path::PathBuf::from(
            vec![env!("CARGO_MANIFEST_DIR"), "resources", "day_02.txt"].join("/"),
        ))
        .unwrap();
        let commands: Vec<Command> = input
            .split('\n')
            .map(|s| Command::from_str(s).unwrap())
            .collect();
        commands
    };
}

pub fn part_1() -> i32 {
    let mut depth: i32 = 0;
    let mut displacement: i32 = 0;
    for command in INPUT.iter() {
        match command {
            Command::Forward(i) => displacement += i,
            Command::Up(i) => depth -= i,
            Command::Down(i) => depth += i,
        }
    }
    depth * displacement
}

pub fn part_2() -> i32 {
    let mut depth: i32 = 0;
    let mut displacement: i32 = 0;
    let mut aim: i32 = 0;
    for command in INPUT.iter() {
        match command {
            Command::Forward(i) => {
                displacement += i;
                depth += aim * i;
            }
            Command::Up(i) => aim -= i,
            Command::Down(i) => aim += i,
        }
    }
    depth * displacement
}

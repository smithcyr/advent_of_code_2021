use std::ops::Range;

use crate::input::load_lines;

use lazy_static::lazy_static;
use regex::Regex;

const LINE_REGEX_STRING: &str = "(?P<x1>\\d+),(?P<y1>\\d+) -> (?P<x2>\\d+),(?P<y2>\\d+)";
lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(LINE_REGEX_STRING).unwrap();
    static ref INPUT: Vec<String> = load_lines("day_05.txt");
}

#[derive(Debug, Eq, PartialEq)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
impl Line {
    fn from(s: &String) -> Line {
        let cap = &LINE_REGEX.captures(s).unwrap();
        Line {
            x1: cap.name("x1").unwrap().as_str().parse::<usize>().unwrap(),
            y1: cap.name("y1").unwrap().as_str().parse::<usize>().unwrap(),
            x2: cap.name("x2").unwrap().as_str().parse::<usize>().unwrap(),
            y2: cap.name("y2").unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}

fn get_range(a: usize, b: usize) -> Range<usize> {
    if a < b {
        a..b + 1
    } else {
        b..a + 1
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Fog {
    state: Vec<Vec<u8>>,
}

impl Fog {
    fn new(n: usize) -> Fog {
        Fog {
            state: vec![vec![0; n]; n],
        }
    }
    fn add_line(&mut self, line: &Line, ignore_diagonal: bool) {
        if line.x1 != line.x2 && line.y1 != line.y2 {
            if ignore_diagonal {
                return;
            }
            let mut x = line.x1;
            let mut y = line.y1;
            self.state[y][x] += 1;
            while x != line.x2 && y != line.y2 {
                if x < line.x2 {
                    x += 1
                } else {
                    x -= 1
                }
                if y < line.y2 {
                    y += 1
                } else {
                    y -= 1
                }
                self.state[y][x] += 1;
            }
        } else {
            for x in get_range(line.x1, line.x2) {
                for y in get_range(line.y1, line.y2) {
                    self.state[y][x] += 1;
                }
            }
        }
    }
    fn add_lines(&mut self, lines: &Vec<Line>, ignore_diagonal: bool) {
        for line in lines {
            self.add_line(line, ignore_diagonal);
        }
    }
    fn total(&self) -> u64 {
        let mut count: u64 = 0;
        for row in self.state.iter() {
            for num in row.iter() {
                if *num > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Line> {
    input.iter().map(|s| Line::from(s)).collect()
}

pub fn part_1() -> u64 {
    let input = parse_input(&INPUT);
    let mut state = Fog::new(1000);
    state.add_lines(&input, true);
    state.total()
}
pub fn part_2() -> u64 {
    let input = parse_input(&INPUT);
    let mut state = Fog::new(1000);
    state.add_lines(&input, false);
    state.total()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let raw_input = vec![
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
        ];
        let input = parse_input(&raw_input);
        assert_eq!(
            input,
            vec![
                Line {
                    x1: 0,
                    y1: 9,
                    x2: 5,
                    y2: 9
                },
                Line {
                    x1: 8,
                    y1: 0,
                    x2: 0,
                    y2: 8
                },
                Line {
                    x1: 9,
                    y1: 4,
                    x2: 3,
                    y2: 4
                }
            ]
        );
    }
    #[test]
    fn test_add_lines() {
        let raw_input = vec![
            String::from("0,9 -> 5,9"),
            // String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            // String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            // String::from("0,0 -> 8,8"),
            // String::from("5,5 -> 8,2"),
        ];
        let input = parse_input(&raw_input);
        let mut state = Fog::new(10);
        state.add_lines(&input, true);
        assert_eq!(state.total(), 5);
    }
    #[test]
    fn test_add_lines_2() {
        let raw_input = vec![
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2"),
        ];
        let input = parse_input(&raw_input);
        let mut state = Fog::new(10);
        state.add_lines(&input, false);
        assert_eq!(state.total(), 12);
    }
}

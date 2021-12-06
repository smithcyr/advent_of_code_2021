use crate::input::load_resource;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = load_resource("day_06.txt");
}

#[derive(Debug, Eq, PartialEq)]
struct LanternFishState {
    state: Vec<u64>,
}

impl LanternFishState {
    fn advance_day(&mut self) {
        let spawning_fish = self.state.drain(0..1).as_slice()[0];
        self.state[6] += spawning_fish;
        self.state.push(spawning_fish);
    }
    fn advance_days(&mut self, days: u64) {
        for _ in 0..days {
            self.advance_day();
        }
    }
    fn from(numbers: Vec<usize>) -> LanternFishState {
        let mut initial_state: Vec<u64> = vec![0; 9];
        for num in numbers.iter() {
            initial_state[*num] += 1;
        }

        LanternFishState {
            state: initial_state,
        }
    }
    fn total(&self) -> u64 {
        self.state.iter().fold(0, |acc, n| acc + n)
    }
}

fn parse_input(input: &String) -> Vec<usize> {
    input
        .split(',')
        .map(|s| String::from(s.to_string().trim()).parse::<usize>().unwrap())
        .collect()
}

pub fn part_1() -> u64 {
    let input = parse_input(&INPUT);
    let mut state = LanternFishState::from(input);
    state.advance_days(80);
    state.total()
}
pub fn part_2() -> u64 {
    let input = parse_input(&INPUT);
    let mut state = LanternFishState::from(input);
    state.advance_days(256);
    state.total()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = String::from("3,4,3,1,2");
        assert_eq!(parse_input(&input), vec![3, 4, 3, 1, 2]);
    }
    #[test]
    fn test_advance_days() {
        let input = String::from("3,4,3,1,2");
        let input = parse_input(&input);
        let mut state = LanternFishState::from(input);
        state.advance_days(80);
        assert_eq!(state.total(), 5934);
    }
    #[test]
    fn test_advance_days_2() {
        let input = String::from("3,4,3,1,2");
        let input = parse_input(&input);
        let mut state = LanternFishState::from(input);
        state.advance_days(256);
        assert_eq!(state.total(), 26984457539);
    }
}

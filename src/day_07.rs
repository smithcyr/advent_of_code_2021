use crate::input::load_resource;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = load_resource("day_07.txt");
}

fn parse_input(input: &String) -> Vec<u64> {
    input
        .split(',')
        .map(|s| String::from(s.to_string().trim()).parse::<u64>().unwrap())
        .collect()
}

fn linear_error(input: &Vec<u64>, pivot: u64) -> u64 {
    input.iter().fold(0_u64, |acc, n| {
        if *n > pivot {
            acc + *n - pivot
        } else {
            acc + pivot - *n
        }
    })
}

fn find_linear_pivot(input: &Vec<u64>) -> u64 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    println!("{} {}", min, max);
    (*min..*max).map(|n| linear_error(&input, n)).min().unwrap()
}

pub fn part_1() -> u64 {
    let input = parse_input(&INPUT);
    find_linear_pivot(&input)
}

fn sum_error(n: u64) -> u64 {
    if n == 1 {
        1
    } else if n > 1 {
        n * (n + 1) / 2
    } else {
        0
    }
}
fn progressive_error(input: &Vec<u64>, pivot: u64) -> u64 {
    input.iter().fold(0_u64, |acc, n| {
        if *n > pivot {
            acc + sum_error(*n - pivot)
        } else {
            acc + sum_error(pivot - *n)
        }
    })
}

fn find_progressive_pivot(input: &Vec<u64>) -> u64 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    println!("{} {}", min, max);
    (*min..*max)
        .map(|n| progressive_error(&input, n))
        .min()
        .unwrap()
}
pub fn part_2() -> u64 {
    let input = parse_input(&INPUT);
    find_progressive_pivot(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(parse_input(&input), vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }
    #[test]
    fn test_linear_error() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        let input = parse_input(&input);

        assert_eq!(linear_error(&input, 2), 37);
    }
    #[test]
    fn test_find_linear_pivot() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        let input = parse_input(&input);

        assert_eq!(find_linear_pivot(&input), 37);
    }
    #[test]
    fn test_find_progressive_pivot() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        let input = parse_input(&input);

        assert_eq!(find_progressive_pivot(&input), 168);
    }
    // #[test]
    // fn test_advance_days_2() {
    //     let input = String::from("3,4,3,1,2");
    //     let input = parse_input(&input);
    //     let mut state = LanternFishState::from(input);
    //     state.advance_days(256);
    //     assert_eq!(state.total(), 26984457539);
    // }
}

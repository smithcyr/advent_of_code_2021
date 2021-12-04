use crate::input::load_lines;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<String> = load_lines("day_03.txt");
}

fn calculate_gamma_epsilon(counts: &Vec<i32>, half: i32) -> (i64, i64) {
    let mut gamma: i64 = 0;
    let mut epsilon: i64 = 0;
    for (index, count) in counts.iter().rev().enumerate() {
        if *count >= half {
            // 1 is the most occuring for this index
            gamma |= 1 << index;
        } else {
            epsilon |= 1 << index;
        }
    }
    (gamma, epsilon)
}

fn count_characters(input: &Vec<String>) -> Vec<i32> {
    let mut counter: Vec<i32> = vec![0; input[0].len()];
    for line in input.iter() {
        for (index, character) in line.chars().enumerate() {
            if character == '1' {
                counter[index] += 1
            }
        }
    }
    counter
}

fn get_oxygen_rating(input: &Vec<i64>, bit_depth: usize) -> i64 {
    let mut rating: i64 = 0;
    for index in 0..bit_depth {
        let bit_shift = bit_depth - 1 - index;
        let mut total: usize = 0;
        let mut count: usize = 0;
        for line in input
            .iter()
            .filter(|line| **line >> (bit_shift + 1) == rating >> (bit_shift + 1))
        {
            total += 1;
            // check bit from right
            if line & 1 << bit_shift > 0 {
                count += 1;
            }
        }
        if total == 1 {
            if count == 1 {
                rating |= 1 << bit_shift;
            }
        } else if count as f32 >= (total as f32 / 2.0) {
            rating |= 1 << bit_shift;
        }
    }
    rating
}

fn get_scrubber_rating(input: &Vec<i64>, bit_depth: usize) -> i64 {
    let mut rating: i64 = 0;
    for index in 0..bit_depth {
        let bit_shift = bit_depth - 1 - index;
        let mut total: usize = 0;
        let mut count: usize = 0;

        for line in input.iter().filter(|line| {
            let letfmost_bits = bit_shift + 1;
            **line >> letfmost_bits == rating >> letfmost_bits
        }) {
            total += 1;
            // check index bit from right
            if line & (1 << bit_shift) > 0 {
                count += 1;
            }
        }

        if total == 1 {
            if count == 1 {
                rating |= 1 << bit_shift;
            }
        } else if (count as f32) < (total as f32 / 2.0) {
            rating |= 1 << bit_shift;
        }
    }
    rating
}

pub fn part_1() -> i64 {
    let counter = count_characters(&INPUT);
    let (gamma, epsilon) = calculate_gamma_epsilon(&counter, INPUT.len() as i32 / 2);
    gamma * epsilon
}

pub fn part_2() -> i64 {
    let length = INPUT[0].len();
    let numbers = INPUT
        .iter()
        .map(|n| i64::from_str_radix(n, 2).unwrap())
        .collect();
    get_oxygen_rating(&numbers, length) * get_scrubber_rating(&numbers, length)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculate_gamma_epsilon() {
        let half: i32 = 6;
        let counter: Vec<i32> = vec![7, 5, 8, 7, 5];
        assert_eq!(calculate_gamma_epsilon(&counter, half), (22, 9))
    }

    #[test]
    fn test_calculate_counter() {
        let test_data: Vec<String> = "\
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"
            .split('\n')
            .map(|s| String::from(s.trim()))
            .collect();
        assert_eq!(count_characters(&test_data), vec![7, 5, 8, 7, 5])
    }
    #[test]
    fn test_oxygen_rating_counter() {
        let test_data: Vec<i64> = "\
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"
            .split('\n')
            .map(|n| i64::from_str_radix(n.trim(), 2).unwrap())
            .collect();
        assert_eq!(get_oxygen_rating(&test_data, 5), 23)
    }

    #[test]
    fn test_scrubber_rating_counter() {
        let test_data: Vec<i64> = "\
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"
            .split('\n')
            .map(|n| i64::from_str_radix(n.trim(), 2).unwrap())
            .collect();
        assert_eq!(get_scrubber_rating(&test_data, 5), 10)
    }
}

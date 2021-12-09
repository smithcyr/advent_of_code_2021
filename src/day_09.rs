use std::collections::HashSet;

use crate::input::load_lines;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<Vec<u32>> = parse_input(load_lines("day_09.txt"));
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u32>> {
    let mut start: Vec<Vec<u32>> = vec![vec![9; input[0].len()]];
    let middle: Vec<Vec<u32>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let end: Vec<Vec<u32>> = vec![vec![9; input[0].len()]];

    start.extend(middle);
    start.extend(end);
    start
}

fn is_low_point(num: u32, top: u32, right: u32, bottom: u32, left: u32) -> bool {
    top > num && right > num && bottom > num && left > num
}

fn compute_danger(three_rows: (&Vec<u32>, &Vec<u32>, &Vec<u32>)) -> u32 {
    let first = three_rows.0;
    let second = three_rows.1;
    let third = three_rows.2;
    second.iter().enumerate().fold(0, |acc, (index, num)| {
        let top = first[index];
        let right = if index + 1 == second.len() {
            10
        } else {
            second[index + 1]
        };
        let bottom = third[index];
        let left = if index == 0 { 10 } else { second[index - 1] };
        if is_low_point(*num, top, right, bottom, left) {
            acc + num + 1
        } else {
            acc
        }
    })
}
fn compute_input_danger(input: &Vec<Vec<u32>>) -> u32 {
    input.windows(3).fold(0, |acc, window| {
        acc + compute_danger((&window[0], &window[1], &window[2]))
    })
}
pub fn part_1() -> u32 {
    compute_input_danger(&INPUT)
}

fn mark_basin(
    coordinate: (usize, usize),
    input: &Vec<Vec<u32>>,
    walked: &mut HashSet<(usize, usize)>,
) -> u32 {
    let (x, y) = coordinate;
    let mut result: u32 = 0;
    if walked.get(&coordinate) == None && input[y][x] != 9 {
        walked.insert(coordinate);
        result += 1;
        if x < input[0].len() - 1 {
            // right 1
            result += mark_basin((x + 1, y), input, walked);
        }
        if x != 0 {
            // left 1
            result += mark_basin((x - 1, y), input, walked);
        }
        if y < input.len() - 1 {
            // down 1
            result += mark_basin((x, y + 1), input, walked);
        }
        if y != 0 {
            // up 1
            result += mark_basin((x, y - 1), input, walked);
        }
    }
    result
}

fn extract_basins(input: &Vec<Vec<u32>>) -> Vec<(u32, (usize, usize))> {
    let mut walked: HashSet<(usize, usize)> = HashSet::new();
    let mut basins: Vec<(u32, (usize, usize))> = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for x in 0..row.len() {
            let basin_size = mark_basin((x, y), input, &mut walked);
            if basin_size != 0 {
                basins.push((basin_size, (x, y)));
            }
        }
    }
    for (y, row) in input.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num != 9 {
                let coord = (x, y);
                assert!(walked.get(&coord) != None);
            }
        }
    }
    basins
}

pub fn part_2() -> u32 {
    let mut basins = extract_basins(&INPUT);
    basins.sort_by(|a, b| b.cmp(a));
    basins[0].0 * basins[1].0 * basins[2].0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_input_danger() {
        let input = vec![
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678"),
        ];
        let numbers = parse_input(input);
        assert_eq!(compute_input_danger(&numbers), 15);
    }

    #[test]
    fn test_compute_danger() {
        let f1 = vec![9; 10];
        let f2 = vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0];
        let f3 = vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1];
        assert_eq!(compute_danger((&f1, &f2, &f3)), 3);
    }

    #[test]
    fn test_basins() {
        let input = vec![
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678"),
        ];
        let numbers = parse_input(input);
        let mut basins = extract_basins(&numbers);
        basins.sort_by(|a, b| b.cmp(a));

        let result = basins[0].0 * basins[1].0 * basins[2].0;
        assert_eq!(result, 1134);
    }
}

use lazy_static::lazy_static;

use crate::input::load_lines;

lazy_static! {
    static ref INPUT: Vec<String> = load_lines("day_15.txt");
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn wrap(v: u32) -> u32 {
    if v > 9 {
        v % 10 + 1
    } else {
        v
    }
}

fn parse_input_part_2(input: &Vec<String>) -> Vec<Vec<u32>> {
    let mut base = parse_input(input);
    for line in base.iter_mut() {
        let base_line = line.clone();
        for increment in 1..5 {
            line.extend(base_line.iter().map(|v| wrap(v + increment)))
        }
    }
    let len = base.len();
    for increment in 1..5 {
        for y in 0..len {
            base.push(
                base[y]
                    .clone()
                    .iter()
                    .map(|v| wrap(v + increment))
                    .collect(),
            );
        }
    }
    base
}

fn lowest_path(input: &mut Vec<Vec<u32>>) -> u32 {
    let mut path_length = input.clone();
    let y_max = input.len() as usize;
    let x_max = input[0].len() as usize;
    let diagonal = x_max + y_max;
    for y_start in 0..diagonal {
        let mut y: usize = y_start;
        for x in 0..x_max {
            if y == 0 && x == 0 {
                path_length[y][x] = 0;
                break;
            }
            if y < y_max {
                let paths = vec![
                    // left
                    if y != 0 {
                        path_length[y - 1][x]
                    } else {
                        u32::MAX
                    },
                    // top
                    if x != 0 {
                        path_length[y][x - 1]
                    } else {
                        u32::MAX
                    },
                    // top right top
                    if y > 1 && x < x_max - 1 {
                        path_length[y - 2][x + 1] + input[y - 1][x + 1] + input[y - 1][x]
                    } else {
                        u32::MAX
                    },
                    // left bottom left
                    if y < y_max - 1 && x > 1 {
                        path_length[y + 1][x - 2] + input[y + 1][x - 1] + input[y][x - 1]
                    } else {
                        u32::MAX
                    },
                ];
                path_length[y][x] = paths.iter().min().unwrap() + input[y][x];
            }
            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }
    }
    path_length[y_max - 1][x_max - 1]
}

pub fn part_1() -> u32 {
    let mut input = parse_input(&INPUT);
    lowest_path(&mut input)
}

pub fn part_2() -> u32 {
    let mut input = parse_input_part_2(&INPUT);
    lowest_path(&mut input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::parse_lines;

    #[test]
    fn test_part_1() {
        let raw_input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let mut input = parse_input(&parse_lines(String::from(raw_input)));
        assert_eq!(lowest_path(&mut input), 40);
    }
    #[test]
    fn test_part_2() {
        let raw_input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let mut input = parse_input_part_2(&parse_lines(String::from(raw_input)));
        for line in input.iter() {
            println!(
                "{}",
                line.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
        assert_eq!(lowest_path(&mut input), 315);
    }
}

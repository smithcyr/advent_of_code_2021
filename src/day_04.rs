use crate::input::load_lines;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<String> = load_lines("day_04.txt");
}

#[derive(Debug, Eq, PartialEq)]
struct BingoBoard {
    found: Vec<u8>,
    board: Vec<Vec<u8>>,
}

impl BingoBoard {
    fn pick_number(&mut self, chosen_number: u8) {
        for (row_index, row) in self.board.iter().enumerate() {
            for (index, num) in row.iter().rev().enumerate() {
                if *num == chosen_number {
                    self.found[row_index] |= 1 << index;
                }
            }
        }
    }
    fn get_score(&self, last_number: u32) -> u32 {
        let mut score: u32 = 0;
        for (row_index, row) in self.board.iter().enumerate() {
            for (index, num) in row.iter().rev().enumerate() {
                if self.found[row_index] & (1 << index) == 0 {
                    score += *num as u32;
                }
            }
        }
        score * last_number
    }
    fn has_bingo(&self) -> bool {
        self.found.contains(&0b11111) || self.found.iter().fold(0b11111, |acc, next| acc & next) > 0
    }
    fn from(numbers: Vec<u8>) -> BingoBoard {
        BingoBoard {
            found: vec![0, 0, 0, 0, 0],
            board: numbers[0..25].chunks(5).map(|x| x.to_vec()).collect(),
        }
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<u8>, Vec<BingoBoard>) {
    let numbers = input[0]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    for board in input[2..].split(|v| v.is_empty()) {
        boards.push(BingoBoard::from(
            board
                .join(" ")
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect(),
        ))
    }
    (numbers, boards)
}

fn play_bingo(numbers: Vec<u8>, mut boards: Vec<BingoBoard>) -> u32 {
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            board.pick_number(*number);
            if board.has_bingo() {
                return board.get_score(*number as u32);
            }
        }
    }
    0
}

fn play_bingo_last_winner(numbers: Vec<u8>, mut boards: Vec<BingoBoard>) -> u32 {
    let total_boards = boards.len();
    let mut counter: u32 = 0;
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            let before = board.has_bingo();
            board.pick_number(*number);
            if board.has_bingo() != before {
                counter += 1;
                if counter == total_boards as u32 {
                    return board.get_score(*number as u32);
                }
            }
        }
    }
    0
}

pub fn part_1() -> u32 {
    let (numbers, boards) = parse_input(&INPUT);
    play_bingo(numbers, boards)
}
pub fn part_2() -> u32 {
    let (numbers, boards) = parse_input(&INPUT);
    play_bingo_last_winner(numbers, boards)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let test_input: Vec<String> = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(|s| String::from(s.trim()))
        .collect();
        let (numbers, boards) = parse_input(&test_input);
        assert_eq!(
            numbers,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            boards,
            vec![
                BingoBoard {
                    found: vec![0, 0, 0, 0, 0],
                    board: vec![
                        vec![22, 13, 17, 11, 0],
                        vec![8, 2, 23, 4, 24],
                        vec![21, 9, 14, 16, 7],
                        vec![6, 10, 3, 18, 5],
                        vec![1, 12, 20, 15, 19],
                    ]
                },
                BingoBoard {
                    found: vec![0, 0, 0, 0, 0],
                    board: vec![
                        vec![3, 15, 0, 2, 22],
                        vec![9, 18, 13, 17, 5],
                        vec![19, 8, 7, 25, 23],
                        vec![20, 11, 10, 24, 4],
                        vec![14, 21, 16, 12, 6],
                    ]
                },
                BingoBoard {
                    found: vec![0, 0, 0, 0, 0],
                    board: vec![
                        vec![14, 21, 17, 24, 4],
                        vec![10, 16, 15, 9, 19],
                        vec![18, 8, 23, 26, 20],
                        vec![22, 11, 13, 6, 5],
                        vec![2, 0, 12, 3, 7],
                    ]
                }
            ]
        );
    }
    #[test]
    fn test_has_bingo() {
        assert!(BingoBoard {
            found: vec![0b11111, 0, 0, 0, 0],
            board: vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
        }
        .has_bingo());
        assert!(BingoBoard {
            found: vec![1, 1, 1, 1, 1],
            board: vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
        }
        .has_bingo());
    }
    #[test]
    fn test_get_score() {
        let test_input: Vec<String> = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16", //,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(|s| String::from(s.trim()))
        .collect();
        let (numbers, boards) = parse_input(&test_input);
        assert_eq!(play_bingo(numbers, boards), 4512);
    }
}

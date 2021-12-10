use crate::input::load_lines;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<String> = load_lines("day_10.txt");
    static ref START: Vec<char> = vec!['(', '[', '{', '<'];
    static ref END: Vec<char> = vec![')', ']', '}', '>'];
}

fn get_char_error(character: char) -> u32 {
    match character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("tried to get error for {}", character),
    }
}
fn get_part_1_line_error(line: &String) -> u32 {
    let mut queue: Vec<char> = Vec::new();
    for symb in line.chars() {
        if let Some(index) = START.iter().position(|v| *v == symb) {
            // push end char for start character
            queue.push(END[index]);
        } else if Some(&symb) == queue.last() {
            queue.pop();
        } else if queue.pop() == None {
            // ignore
            return 0;
        } else {
            return get_char_error(symb);
        }
    }
    0
}

fn get_part_1_error(input: &Vec<String>) -> u32 {
    input
        .iter()
        .fold(0, |acc, line| acc + get_part_1_line_error(line))
}

pub fn part_1() -> u32 {
    get_part_1_error(&INPUT)
}

fn get_part_2_error(input: &Vec<String>) -> u128 {
    let mut errors: Vec<u128> = input
        .iter()
        .map(|line| get_part_2_line_error(line))
        .filter(|e| *e > 0)
        .collect::<Vec<u128>>();
    errors.sort();
    errors[errors.len() / 2]
}
fn get_part_2_char_error(character: char) -> u128 {
    match character {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("tried to get error for {}", character),
    }
}
fn get_part_2_line_error(line: &String) -> u128 {
    let mut queue: Vec<char> = Vec::new();
    for symb in line.chars() {
        if let Some(index) = START.iter().position(|v| *v == symb) {
            // push end char for start character
            queue.push(END[index]);
        } else if Some(&symb) == queue.last() {
            queue.pop();
        } else if queue.pop() == None {
            // ignore
            return 0;
        } else {
            // ignore corrupted line
            return 0;
        }
    }
    queue
        .iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + get_part_2_char_error(*c))
}
pub fn part_2() -> u128 {
    get_part_2_error(&INPUT)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];
        assert_eq!(get_part_1_error(&input), 26397);
    }
    #[test]
    fn test_part_1_line() {
        assert_eq!(
            get_part_1_line_error(&String::from("{([(<{}[<>[]}>{[]{[(<()>")),
            1197
        );
        assert_eq!(
            get_part_1_line_error(&String::from("[[<[([]))<([[{}[[()]]]")),
            3
        );
        assert_eq!(
            get_part_1_line_error(&String::from("[{[{({}]{}}([{[{{{}}([]")),
            57
        );
        assert_eq!(
            get_part_1_line_error(&String::from("[<(<(<(<{}))><([]([]()")),
            3
        );
        assert_eq!(
            get_part_1_line_error(&String::from("<{([([[(<>()){}]>(<<{{")),
            25137
        );
    }
    #[test]
    fn test_part_2() {
        let input = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"), //
            String::from("[(()[<>])]({[<{<<[]>>("),   //
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"), //
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"), //
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"), //
        ];
        assert_eq!(get_part_2_error(&input), 288957);
    }
    #[test]
    fn test_part_2_line() {
        assert_eq!(
            get_part_2_line_error(&String::from("[({(<(())[]>[[{[]{<()<>>")),
            288957
        );
        assert_eq!(
            get_part_2_line_error(&String::from("[(()[<>])]({[<{<<[]>>(")),
            5566
        );
        assert_eq!(
            get_part_2_line_error(&String::from("(((({<>}<{<{<>}{[]{[]{}")),
            1480781
        );
        assert_eq!(
            get_part_2_line_error(&String::from("{<[[]]>}<{[{[{[]{()[[[]")),
            995444
        );
        assert_eq!(
            get_part_2_line_error(&String::from("<{([{{}}[<[[[<>{}]]]>[]]")),
            294
        );
    }
}

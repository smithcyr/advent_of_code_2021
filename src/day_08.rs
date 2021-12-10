use std::collections::HashMap;

use crate::input::load_lines;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<String> = load_lines("day_08.txt");
    static ref CHAR_VALUE: HashMap<char, u32> = [
        ('a', 0b1),
        ('b', 0b10),
        ('c', 0b100),
        ('d', 0b1000),
        ('e', 0b10000),
        ('f', 0b100000),
        ('g', 0b1000000),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<char, u32>>();
}

pub fn part_1() -> u64 {
    let mut counter: u64 = 0;
    for line in INPUT.iter() {
        for character in line.split(" | ").collect::<Vec<&str>>()[1].split_whitespace() {
            let length = character.len();
            if length == 2 || length == 3 || length == 4 || length == 7 {
                counter += 1;
            }
        }
    }
    counter
}

fn get_item_hash(segments: &str) -> u32 {
    segments
        .chars()
        .fold(0, |acc, c| acc | CHAR_VALUE.get(&c).unwrap())
}

fn decode_sample(sample: &Vec<&str>) -> HashMap<u32, u32> {
    let mut decoded: HashMap<u32, u32> = HashMap::new();
    let mut letter_counts: HashMap<char, u32> = HashMap::new();
    for c in sample.join("").chars() {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    let mut segment_5: char = ' ';
    let mut segment_6: char = ' ';
    for (letter, count) in &letter_counts {
        if *count == 4 {
            segment_5 = *letter;
        }
        if *count == 6 {
            segment_6 = *letter;
        }
    }
    if segment_5 == ' ' || segment_6 == ' ' {
        panic!(
            "failed to find segment 5 or 6: \n{:?} \n{:#?}",
            sample, letter_counts
        );
    }
    let mut char_1_segments: String = String::from("");
    for entry in sample {
        if entry.len() == 2 {
            decoded.insert(get_item_hash(entry), 1);
            char_1_segments = entry.to_string();
        } else if entry.len() == 5 && entry.contains(segment_5) {
            decoded.insert(get_item_hash(entry), 2);
        } else if entry.len() == 5 && !entry.contains(segment_5) && !entry.contains(segment_6) {
            decoded.insert(get_item_hash(entry), 3);
        } else if entry.len() == 4 {
            decoded.insert(get_item_hash(entry), 4);
        } else if entry.len() == 5 && entry.contains(segment_6) {
            decoded.insert(get_item_hash(entry), 5);
        } else if entry.len() == 3 {
            decoded.insert(get_item_hash(entry), 7);
        } else if entry.len() == 7 {
            decoded.insert(get_item_hash(entry), 8);
        } else if entry.len() == 6 && !entry.contains(segment_5) {
            decoded.insert(get_item_hash(entry), 9);
        }
    }
    fn missing_chars(source: String, query: &String) -> usize {
        query.chars().filter(|c| !source.contains(*c)).count()
    }
    for entry in sample {
        if entry.len() == 6 && missing_chars(entry.to_string(), &char_1_segments) == 1_usize {
            &mut decoded.insert(get_item_hash(entry), 6);
        }
    }
    assert!(decoded.len() == 9);
    for entry in sample {
        if let None = decoded.get(&get_item_hash(entry)) {
            decoded.insert(get_item_hash(entry), 0);
        }
    }

    assert!(decoded.len() == 10);

    decoded
}

fn decode_number(decode_hash: &HashMap<u32, u32>, segments: &Vec<&str>) -> u32 {
    let mut line_number: u32 = 0;
    for (index, character) in segments.iter().rev().enumerate() {
        let number = decode_hash.get(&get_item_hash(character)).unwrap();
        line_number += number * 10_u32.pow(index as u32);
    }
    line_number
}

fn decode_input(input: &Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for line in input.iter() {
        let line_parts = line.split("|").collect::<Vec<&str>>();
        let sample_characters = line_parts[0]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let mapping = decode_sample(&sample_characters);
        let encoded_num = line_parts[1]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        total += decode_number(&mapping, &encoded_num);
    }
    total
}

pub fn part_2() -> u32 {
    decode_input(&INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_sample() {
        let input = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        let decode_hash = decode_sample(&input);
        assert_eq!(
            decode_hash,
            [
                (get_item_hash("acedgfb"), 8),
                (get_item_hash("cdfbe"), 5),
                (get_item_hash("gcdfa"), 2),
                (get_item_hash("fbcad"), 3),
                (get_item_hash("dab"), 7),
                (get_item_hash("cefabd"), 9),
                (get_item_hash("cdfgeb"), 6),
                (get_item_hash("eafb"), 4),
                (get_item_hash("cagedb"), 0),
                (get_item_hash("ab"), 1)
            ]
            .iter()
            .cloned()
            .collect::<HashMap<u32, u32>>()
        );
        let encoded_num = vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"];
        assert_eq!(
            *decode_hash.get(&get_item_hash(&encoded_num[0])).unwrap(),
            5
        );
        assert_eq!(
            *decode_hash.get(&get_item_hash(&encoded_num[1])).unwrap(),
            3
        );
        assert_eq!(
            *decode_hash.get(&get_item_hash(&encoded_num[2])).unwrap(),
            5
        );
        assert_eq!(
            *decode_hash.get(&get_item_hash(&encoded_num[3])).unwrap(),
            3
        );
        assert_eq!(decode_number(&decode_hash, &encoded_num), 5353);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            String::from(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            ),
            String::from(
                "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            ),
            String::from(
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            ),
            String::from(
                "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            ),
            String::from(
                "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            ),
            String::from(
                "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            ),
            String::from(
                "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            ),
            String::from(
                "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            ),
            String::from(
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            ),
            String::from(
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
            ),
        ];
        assert_eq!(decode_input(&input), 61229);
    }
}

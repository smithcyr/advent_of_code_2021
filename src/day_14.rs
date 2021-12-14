use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::input::load_resource;

lazy_static! {
    static ref INPUT: String = load_resource("day_14.txt");
}
const SEED: &str = "OOFNFCBHCKBBVNHBNVCP";

fn parse_seed(input: &String) -> HashMap<String, u128> {
    let mut map: HashMap<String, u128> = HashMap::new();
    for c in input.chars() {
        let s = String::from(c);
        if let Some(count) = map.get_mut(&s) {
            *count += 1;
        } else {
            map.insert(s, 1);
        }
    }
    for window in input.chars().collect::<Vec<char>>().windows(2) {
        let pair = window.iter().collect::<String>();
        if let Some(count) = map.get_mut(&pair) {
            *count += 1;
        } else {
            map.insert(pair, 1);
        }
    }
    map
}

fn parse_steps(input: &String) -> HashMap<String, (String, String, String)> {
    input
        .split("\n")
        .map(|line| line.trim().split(" -> ").collect::<Vec<&str>>())
        .map(|parts| {
            (
                String::from(parts[0]),
                (
                    format!(
                        "{}{}",
                        parts[0].chars().nth(0).unwrap(),
                        parts[1].chars().nth(0).unwrap(),
                    ),
                    format!(
                        "{}{}",
                        parts[1].chars().nth(0).unwrap(),
                        parts[0].chars().nth(1).unwrap()
                    ),
                    parts[1].to_string(),
                ),
            )
        })
        .collect()
}

fn process_step(
    mut input: HashMap<String, u128>,
    steps: &HashMap<String, (String, String, String)>,
) -> HashMap<String, u128> {
    let mut changes: Vec<((String, String, String), u128)> = Vec::new();
    for (key, value) in steps {
        if let Some(count) = input.get(key) {
            changes.push((value.clone(), *count));
            input.remove_entry(key);
        }
    }
    for ((first, second, c), num) in changes {
        if let Some(count) = input.get_mut(&first) {
            *count += num;
        } else {
            input.insert(first, num);
        }

        if let Some(count) = input.get_mut(&second) {
            *count += num;
        } else {
            input.insert(second, num);
        }

        if let Some(count) = input.get_mut(&c) {
            *count += num;
        } else {
            input.insert(c, num);
        }
    }
    input
}

pub fn part_1() -> u128 {
    let steps = parse_steps(&INPUT);
    let initial = parse_seed(&String::from(SEED));
    let result = (0..10).fold(initial, |prev, _| process_step(prev, &steps));
    let max = &result
        .iter()
        .filter(|(key, _)| key.len() == 1)
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    let min = &result
        .iter()
        .filter(|(key, _)| key.len() == 1)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    *max.1 - *min.1
}

pub fn part_2() -> u128 {
    let steps = parse_steps(&INPUT);
    let initial = parse_seed(&String::from(SEED));
    let result = (0..40).fold(initial, |prev, _| process_step(prev, &steps));
    let max = &result
        .iter()
        .filter(|(key, _)| key.len() == 1)
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    let min = &result
        .iter()
        .filter(|(key, _)| key.len() == 1)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    *max.1 - *min.1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_seed() {
        let seed = String::from("NNCB");
        assert_eq!(
            parse_seed(&seed),
            [
                (String::from("N"), 2),
                (String::from("C"), 1),
                (String::from("B"), 1),
                (String::from("NN"), 1),
                (String::from("NC"), 1),
                (String::from("NB"), 1),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<String, u128>>()
        )
    }
    #[test]
    fn test_part_1() {
        let initial = parse_seed(&String::from("NNCB"));
        let raw_input = String::from(
            "CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C",
        );
        let input = parse_steps(&raw_input);
        println!("{:?}", initial);
        let result = (0..10).fold(initial, |prev, _| process_step(prev, &input));
        println!("{:?}", result);
        let max = &result
            .iter()
            .filter(|(key, _)| key.len() == 1)
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        let min = &result
            .iter()
            .filter(|(key, _)| key.len() == 1)
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        assert_eq!(*max.1 - *min.1, 1588);
    }
}

use std::fs;
use std::path;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<f32> = {
        let input = fs::read_to_string(path::PathBuf::from(
            vec![env!("CARGO_MANIFEST_DIR"), "resources", "day_01.txt"].join("/"),
        ))
        .unwrap();
        let numbers: Vec<f32> = input
            .split('\n')
            .map(|s| s.to_string().trim().parse::<f32>().unwrap())
            .collect();
        numbers
    };
}

pub fn part_1() -> i32 {
    let mut part_1_counter: i32 = 0;
    &INPUT.iter().fold(f32::INFINITY, |prev, x| {
        if prev < *x {
            part_1_counter += 1;
        }
        *x
    });
    part_1_counter
}

pub fn part_2() -> i32 {
    let mut part_2_counter: i32 = 0;
    let sum_tuple = |(x, y, z): (f32, f32, f32)| x + y + z;
    &INPUT
        .iter()
        .fold((f32::INFINITY, f32::INFINITY, f32::INFINITY), |prev, x| {
            let next = (prev.1, prev.2, *x);
            if sum_tuple(prev) < sum_tuple(next) {
                part_2_counter += 1;
            }
            next
        });
    part_2_counter
}

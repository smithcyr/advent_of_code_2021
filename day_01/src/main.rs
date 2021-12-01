use std::fs;
use std::path;

fn main() {
    let input = fs::read_to_string(path::PathBuf::from(
        vec![env!("CARGO_MANIFEST_DIR"), "resource", "input.txt"].join("/"),
    ))
    .unwrap();
    let numbers: Vec<f32> = input
        .split('\n')
        .map(|s| s.to_string().trim().parse::<f32>().unwrap())
        .collect();

    let mut part_1_counter: i32 = 0;
    numbers.iter().fold(f32::INFINITY, |prev, x| {
        if prev < *x {
            part_1_counter += 1;
        }
        *x
    });
    println!("part 1: {:#?}", part_1_counter);

    let mut part_2_counter: i32 = 0;
    numbers
        .iter()
        .fold((f32::INFINITY, f32::INFINITY, f32::INFINITY), |prev, x| {
            let next = (prev.1, prev.2, *x);
            if prev.0 + prev.1 + prev.2 < next.0 + next.1 + next.2 {
                part_2_counter += 1;
            }
            next
        });
    println!("part 2: {:#?}", part_2_counter);
}

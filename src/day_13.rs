use std::collections::HashSet;

use crate::input::load_resource;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = load_resource("day_13.txt");
    static ref FOLDS: String = load_resource("day_13_folds.txt");
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone)]
enum Fold {
    Horizontal(u32),
    Vertical(u32),
}
fn parse_folds(input: &String) -> Vec<Fold> {
    let mut result: Vec<Fold> = Vec::new();
    for line in input.split('\n').map(|l| l.trim()) {
        let parts = line.split('=').collect::<Vec<&str>>();
        let aspect = parts[0].split_whitespace().collect::<Vec<&str>>()[2];
        let coord = match aspect {
            "y" => Fold::Vertical(parts[1].parse::<u32>().unwrap()),
            "x" => Fold::Horizontal(parts[1].parse::<u32>().unwrap()),
            _ => panic!("wrong slope {}", aspect),
        };
        result.push(coord);
    }
    result
}

fn parse_coordinates(input: &String) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for line in input.split('\n').map(|l| l.trim()) {
        let parts = line.split(',').collect::<Vec<&str>>();
        let coord = Point {
            x: parts[0].parse::<u32>().unwrap(),
            y: parts[1].parse::<u32>().unwrap(),
        };
        result.push(coord);
    }
    result
}

fn fold_on_crease(points: &Vec<Point>, crease: Fold) -> Vec<Point> {
    let mut set: HashSet<Point> = HashSet::new();
    for point in points {
        match crease {
            Fold::Horizontal(foldx) => {
                if foldx < point.x {
                    let difference = point.x - foldx;
                    set.insert(Point {
                        x: foldx - difference,
                        y: point.y,
                    });
                } else {
                    set.insert(*point);
                }
            }
            Fold::Vertical(foldy) => {
                if foldy < point.y {
                    let difference = point.y - foldy;
                    set.insert(Point {
                        x: point.x,
                        y: foldy - difference,
                    });
                } else {
                    set.insert(*point);
                }
            }
        };
    }
    set.into_iter().collect()
}

pub fn part_1() -> usize {
    let points = parse_coordinates(&INPUT);
    let folds = parse_folds(&FOLDS);

    fold_on_crease(&points, folds[0]).len()
}

pub fn part_2() -> String {
    let mut points = parse_coordinates(&INPUT);
    for fold in parse_folds(&FOLDS) {
        points = fold_on_crease(&points, fold);
    }
    let mut display = vec![vec![' '; 40]; 6];
    for point in points.iter() {
        display[point.y as usize][point.x as usize] = 'X';
    }
    let mut result: String = String::new();
    for line in display {
        result.push_str("\n");
        result.push_str(line.iter().collect::<String>().as_str());
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let raw_points = String::from(
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0",
        );
        let points = parse_coordinates(&raw_points);
        let raw_folds = String::from(
            "fold along y=7
fold along x=5",
        );
        let folds = parse_folds(&raw_folds);
        let first_fold = fold_on_crease(&points, folds[0]);
        assert_eq!(first_fold.len(), 17);
        let second_fold = fold_on_crease(&first_fold, folds[1]);
        assert_eq!(second_fold.len(), 16);
    }
}

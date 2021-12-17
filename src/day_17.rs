use std::collections::HashSet;

const MIN_X: u32 = 150;
const MAX_X: u32 = 193;
const MIN_Y: i32 = -136;
const MAX_Y: i32 = -86;

fn sum_x(steps: u32, initial: u32) -> u32 {
    (0..steps).fold(0, |acc, offset| {
        acc + if initial > offset {
            initial - offset
        } else {
            0
        }
    })
}
fn sum_y(steps: u32, initial: i32) -> i32 {
    (0..steps).fold(0, |acc, offset| acc + initial - offset as i32)
}

pub fn part_1() -> i32 {
    let velocity_and_steps = (MIN_Y).abs() - 1;
    sum_y(velocity_and_steps as u32, velocity_and_steps)
}

fn get_total_velocities(minx: u32, maxx: u32, miny: i32, maxy: i32) -> u32 {
    // why not brute force ??

    let mut unique: HashSet<(u32, i32)> = HashSet::new();
    for steps in 0..300 {
        for y in 0..300 {
            let disp_y = sum_y(steps, y - 150);
            if disp_y >= miny && disp_y <= maxy {
                for x in 0..200 {
                    let disp_x = sum_x(steps, x);
                    if disp_x >= minx && disp_x <= maxx {
                        unique.insert((x, y - 150));
                    }
                }
            }
        }
    }
    unique.len() as u32
}

pub fn part_2() -> u32 {
    get_total_velocities(MIN_X, MAX_X, MIN_Y, MAX_Y)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(sum_y(4, 4), 10);
        assert_eq!(sum_y(135, 135), 9180);
        assert_eq!(sum_y(272, 135), -136);
        assert_eq!(sum_x(4, 4), 10);
        assert_eq!(sum_x(135, 135), 9180);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(get_total_velocities(20, 30, -10, -5), 112);
    }
}

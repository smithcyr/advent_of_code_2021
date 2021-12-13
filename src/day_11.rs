use std::collections::HashSet;

fn get_input() -> Vec<Vec<u32>> {
    vec![
        vec![7, 2, 3, 2, 3, 7, 4, 3, 1, 4],
        vec![8, 5, 3, 1, 1, 1, 3, 7, 8, 6],
        vec![3, 4, 1, 1, 7, 8, 7, 8, 2, 8],
        vec![5, 4, 8, 2, 2, 4, 1, 3, 4, 4],
        vec![5, 8, 5, 6, 8, 2, 7, 7, 4, 2],
        vec![7, 6, 1, 4, 5, 3, 2, 7, 6, 4],
        vec![5, 3, 1, 1, 3, 2, 1, 7, 5, 8],
        vec![1, 2, 5, 5, 1, 1, 6, 1, 8, 7],
        vec![5, 8, 2, 1, 2, 7, 7, 7, 1, 4],
        vec![2, 6, 2, 3, 8, 3, 4, 7, 8, 8],
    ]
}

fn advance_step(state: &mut Vec<Vec<u32>>) -> u32 {
    let size = state.len();
    // add 1 to all elements
    for y in 0..size {
        for x in 0..size {
            state[y][x] += 1;
        }
    }
    let mut flashing: HashSet<(usize, usize)> = HashSet::new();

    // iterate all
    loop {
        let current_flashing: Vec<(usize, usize)> = (0..size)
            .flat_map(|y| (0..size).map(move |x| (x, y)))
            .filter(|(x, y)| state[*y][*x] > 9 && flashing.get(&(*x, *y)) == None)
            .collect::<Vec<(usize, usize)>>();

        if current_flashing.len() == 0 {
            break;
        }
        for (x, y) in current_flashing {
            &mut flashing.insert((x, y));
            if y > 0 {
                if x > 0 {
                    state[y - 1][x - 1] += 1;
                }
                state[y - 1][x] += 1;
                if x < size - 1 {
                    state[y - 1][x + 1] += 1;
                }
            }
            if x > 0 {
                state[y][x - 1] += 1;
            }
            if x < size - 1 {
                state[y][x + 1] += 1;
            }
            if y < size - 1 {
                if x > 0 {
                    state[y + 1][x - 1] += 1;
                }
                state[y + 1][x] += 1;
                if x < size - 1 {
                    state[y + 1][x + 1] += 1;
                }
            }
        }
    }

    // set all flashing coordinates to 0
    for (x, y) in flashing.iter() {
        state[*y][*x] = 0;
    }

    // return all flashing coordinates
    flashing.len() as u32
}

pub fn part_1() -> u32 {
    let mut state = get_input();
    (0..100).fold(0, |acc, _| acc + advance_step(&mut state))
}

pub fn part_2() -> u32 {
    let mut state = get_input();
    let mut counter: u32 = 0;
    loop {
        counter += 1;
        if advance_step(&mut state) == 100 {
            return counter;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let mut initial: Vec<Vec<u32>> = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];
        fn for_test(state: &Vec<Vec<u32>>) -> String {
            state
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n")
        }
        assert_eq!(
            for_test(&initial),
            "11111
19991
19191
19991
11111"
        );
        advance_step(&mut initial);

        assert_eq!(
            for_test(&initial),
            "34543
40004
50005
40004
34543"
        );
        advance_step(&mut initial);

        assert_eq!(
            for_test(&initial),
            "45654
51115
61116
51115
45654"
        );
    }
}

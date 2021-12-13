use std::collections::{HashMap, HashSet};

struct Path {
    path: Vec<String>,
}

impl Path {
    fn has_double_visit(&self) -> bool {
        let mut small_caves: HashSet<&String> = HashSet::new();
        for small_cave in self
            .path
            .iter()
            .filter(|p| p.chars().all(|c| c.is_lowercase()))
        {
            if small_caves.contains(small_cave) {
                return true;
            }
            small_caves.insert(small_cave);
        }

        return false;
    }
}

static INPUT: &str = "start-co
ip-WE
end-WE
le-ls
wt-zi
end-sz
wt-RI
wt-sz
zi-start
wt-ip
YT-sz
RI-start
le-end
ip-sz
WE-sz
le-WE
le-wt
zi-ip
RI-zi
co-zi
co-le
WB-zi
wt-WE
co-RI
RI-ip";

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.split("\n").map(|s| s.to_string()) {
        let parts = line.split('-').collect::<Vec<&str>>();
        if let Some(connections) = map.get_mut(parts[0]) {
            connections.push(String::from(parts[1]));
        } else {
            map.insert(String::from(parts[0]), vec![String::from(parts[1])]);
        }
        if let Some(connections) = map.get_mut(parts[1]) {
            connections.push(String::from(parts[0]));
        } else {
            map.insert(String::from(parts[1]), vec![String::from(parts[0])]);
        }
    }
    map
}

fn calculate_paths(input: &HashMap<String, Vec<String>>) -> Vec<Path> {
    let mut total_paths: Vec<Path> = Vec::new();
    let mut walkers: Vec<Path> = vec![Path {
        path: vec![String::from("start")],
    }];
    while walkers.len() > 0 {
        let mut next_walkers: Vec<Path> = Vec::new();
        for walker in walkers {
            if let Some(paths) = input.get(walker.path.last().unwrap()) {
                for next in paths.iter().filter(|p| {
                    !walker.path.contains(p) || p.chars().all(|c| char::is_uppercase(c))
                }) {
                    let mut prev_path = walker.path.clone();
                    prev_path.push(String::from(next));
                    let next_walker = Path { path: prev_path };
                    if next == "end" {
                        total_paths.push(next_walker);
                    } else {
                        next_walkers.push(next_walker);
                    }
                }
            }
        }

        walkers = next_walkers;
    }
    total_paths
}

pub fn part_1() -> usize {
    let input = parse_input(INPUT);
    let total_paths = calculate_paths(&input);
    total_paths.len()
}

fn calculate_paths_2(input: &HashMap<String, Vec<String>>) -> u32 {
    let mut total_paths: u32 = 0;
    let mut walkers: Vec<Path> = vec![Path {
        path: vec![String::from("start")],
    }];
    while walkers.len() > 0 {
        let mut next_walkers: Vec<Path> = Vec::new();
        for walker in walkers {
            if let Some(paths) = input.get(walker.path.last().unwrap()) {
                for next in paths.iter().filter(|p| {
                    p.chars().all(|c| char::is_uppercase(c))
                        || (*p != "start"
                            && walker.path.iter().filter(|prev| p == prev).count()
                                < if walker.has_double_visit() { 1 } else { 2 })
                }) {
                    let mut prev_path = walker.path.clone();
                    prev_path.push(String::from(next));
                    let next_walker = Path { path: prev_path };
                    if next == "end" {
                        total_paths += 1;
                    } else {
                        next_walkers.push(next_walker);
                    }
                }
            }
        }

        walkers = next_walkers;
    }
    total_paths
}

pub fn part_2() -> u32 {
    let input = parse_input(INPUT);
    let total_paths = calculate_paths_2(&input);
    total_paths
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = parse_input(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        );
        assert_eq!(calculate_paths(&input).len(), 19);
        let input = parse_input(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        );
        assert_eq!(calculate_paths(&input).len(), 226);
    }
    #[test]
    fn test_part_2() {
        let input = parse_input(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );
        assert_eq!(calculate_paths_2(&input), 36);
        let input = parse_input(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        );
        assert_eq!(calculate_paths_2(&input), 103);
        let input = parse_input(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        );
        assert_eq!(calculate_paths_2(&input), 3509);
    }
}

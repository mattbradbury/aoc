use std::{collections::HashMap, fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input: String = load_input(2021, 12).clone();

    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(&input)));
    // drop(input);
}

fn bench<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    T: Display,
{
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

type CaveMap<'a> = HashMap<&'a str, Vec<Cave<'a>>>;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Copy)]
struct Cave<'a> {
    label: &'a str,
    big: bool,
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(input: &'a str) -> Self {
        let big = input.to_ascii_uppercase() == input;
        Cave { label: input, big }
    }
}

impl<'a> Display for Cave<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[allow(dead_code)]
fn parse_input<'a>(input: &'a str) -> CaveMap {
    let mut map = CaveMap::new();
    for line in input.trim().lines() {
        let (left, right) = line.split_once('-').unwrap();
        let left: Cave = left.into();
        let right: Cave = right.into();
        assert!(!(left.big == true && right.big == true));
        map.entry(left.label)
            .or_insert(Vec::new())
            .push(right.clone());
        map.entry(right.label).or_insert(Vec::new()).push(left);
    }

    map
}

fn visit<'a>(cave: &Cave<'a>, map: &CaveMap, mut visited: Vec<Cave<'a>>) -> usize {
    visited.push(cave.clone());
    if cave.label == "end" {
        return 1;
    };
    map.get(&cave.label)
        .unwrap()
        .iter()
        .map(|edge| {
            if edge.big || !visited.contains(edge) {
                visit(edge, map, visited.clone())
            } else {
                0
            }
        })
        .sum()
}

fn _has_two_smalls(visited: &Vec<Cave>) -> bool {
    for cave in visited {
        if cave.big {
            continue;
        };
        if visited.iter().filter(|c| *c == cave).count() > 1 {
            return true;
        }
    }
    false
}

fn visit_twice<'a>(
    cave: &'a Cave<'a>,
    map: &CaveMap,
    mut visited: Vec<&'a Cave<'a>>,
    mut twice: bool,
) -> usize {
    if !cave.big && visited.contains(&&cave) {
        twice = true
    }
    visited.push(cave);

    if cave.label == "end" {
        return 1;
    }
    map.get(&cave.label)
        .unwrap()
        .iter()
        .map(|edge| {
            if edge.label != "start" && (edge.big || !twice || !visited.contains(&edge)) {
                visit_twice(edge, map, visited.clone(), twice)
            } else {
                0
            }
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let start = Cave {
        label: "start",
        big: false,
    };
    visit(&start, &map, Vec::new())
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let start = Cave {
        label: "start",
        big: false,
    };
    visit_twice(&start, &map, Vec::new(), false)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 10)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 36)
    }
}

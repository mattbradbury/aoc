use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, time, vec};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 6);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(&input)));
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

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn part1(input: &str) -> String {
    let input = parse_input(input);
    let width = input[0].len();
    let mut vecmap = vec![HashMap::new(); width];
    for line in input {
        for i in 0..width {
            *vecmap[i].entry(line[i]).or_insert(0) += 1;
        }
    }

    vecmap
        .iter()
        .map(|map| *map.iter().max_by_key(|(_, &v)| v).unwrap().0)
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    let width = input[0].len();
    let mut vecmap = vec![HashMap::new(); width];
    for line in input {
        for i in 0..width {
            *vecmap[i].entry(line[i]).or_insert(0) += 1;
        }
    }

    vecmap
        .iter()
        .map(|map| *map.iter().min_by_key(|(_, &v)| v).unwrap().0)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), "easter")
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), "advent")
    }
}

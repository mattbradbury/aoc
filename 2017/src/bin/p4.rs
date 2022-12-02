use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 4);
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
fn parse_input(input: &str) -> Vec<Vec<String>> {
    input.trim()
        .lines()
        .map(|line| 
            line.split_whitespace()
                .map(|word| word.into())
                .collect_vec()
        )
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.into_iter().map(|mut row| {
        row.sort();
        let a = row.len();
        row.dedup();
        if a == row.len() { 1 } else { 0 }
    })
    .sum()
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    input.into_iter().map(|mut row| {
        row = row.into_iter().map(|word| {
            let mut word = word.chars().collect_vec();
            word.sort();
            let word = word.iter().collect::<String>();
            word
        }).collect_vec();
        row.sort();
        let a = row.len();
        row.dedup();
        if a == row.len() { 1 } else { 0 }
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa
"#;

    const EXAMPLE2: &str =
r#"abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 2)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE2), 3)
    }
}

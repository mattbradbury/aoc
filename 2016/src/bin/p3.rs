use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 3);
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
fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| scan_fmt!(line, "{d} {d} {d}", usize, usize, usize).unwrap())
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse_input(&input);
    input
        .iter()
        .filter(|(a, b, c)| a + b > *c && a + c > *b && b + c > *a)
        .count()
}

fn part2(input: &str) -> usize {
    let input = parse_input(&input);
    input
        .into_iter()
        .tuple_windows()
        .enumerate()
        .fold(0, |mut acc, (i, (a, b, c))| {
            if i % 3 != 0 {
                return acc;
            };
            if a.0 + b.0 > c.0 && a.0 + c.0 > b.0 && b.0 + c.0 > a.0 {
                acc += 1
            };
            if a.1 + b.1 > c.1 && a.1 + c.1 > b.1 && b.1 + c.1 > a.1 {
                acc += 1
            };
            if a.2 + b.2 > c.2 && a.2 + c.2 > b.2 && b.2 + c.2 > a.2 {
                acc += 1
            };

            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 0)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

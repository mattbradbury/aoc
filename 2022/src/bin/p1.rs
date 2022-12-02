use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2022, 1);
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
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.split("\n\n").map(|group| {
        group.lines()
            .map(|line| { 
                line.parse::<usize>().unwrap() 
            }).collect_vec()
    }).collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.into_iter().map(|group| {
        group.into_iter().sum()
    }).max().unwrap()
    
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);

    input.into_iter().map(|group| -> usize  {
        group.into_iter().sum()
    }).sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 24000)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 45000)
    }
}

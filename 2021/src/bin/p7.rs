use std::time;

use aoc_helper::load_input;
use itertools::Itertools;

fn main() {
    let input = load_input(2021, 7);
    let input = input.trim();
    let input = parse_input(input);

    bench(|| part1(&input));
    bench(|| part2(&input));
}

fn bench<F>(f: F)
where
    F: Fn(),
{
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn part1(input: &[usize]) {
    let res = find_optimal(input, false);
    println!("Part1: {}", res)
}

fn part2(input: &[usize]) {
    // let input = parse_input(input);
    let res = find_optimal(input, true);
    println!("Part2: {}", res);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|v| {
            // println!("v= {}", v);
            v.parse().unwrap()
        })
        .collect()
}

fn find_optimal(input: &[usize], part2: bool) -> usize {
    let (min, max) = if let itertools::MinMaxResult::MinMax(min, max) = input.iter().minmax() {
        (*min, *max)
    } else {
        panic!()
    };
    (min..=max)
        .map(|pos| match part2 {
            false => get_cost(&input, pos),
            true => get_sluggish_cost(&input, pos),
        })
        .min()
        .unwrap()
}

fn get_cost(input: &[usize], pos: usize) -> usize {
    input
        .iter()
        .map(|v| (*v as isize - pos as isize).abs() as usize)
        .sum()
}

fn get_sluggish_cost(input: &[usize], pos: usize) -> usize {
    input
        .iter()
        .map(|v| {
            let dist = (*v as isize - pos as isize).abs() as usize;
            (dist * dist + 1) / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{find_optimal, parse_input};

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_example1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(find_optimal(&input, false), 37);
    }

    #[test]
    fn test_example2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(find_optimal(&input, true), 168);
    }
}

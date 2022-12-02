use itertools::Itertools;
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 18);
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

fn count_tiles(input: &str, num_rows: usize) -> usize {
    let row = input.trim().chars().map(|c| {
        match c {
            '^' => true,
            '.' => false,
            _ => panic!()
        }
    }).collect_vec();

    let mut rows = vec![row];

    let row_builder = |prev: &Vec<bool>| -> Vec<bool> {
        let mut prev = prev.clone();
        prev.insert(0, false);
        prev.push(false);
        prev.iter().tuple_windows().map(|(a, b, c)| {
            match (a,b,c) {
                (true, true, true) => false,
                (true, true, false) => true,
                (true, false, true) => false,
                (true, false, false) => true,
                (false, true, true) => true,
                (false, true, false) => false,
                (false, false, true) => true,
                (false, false, false) => false,
            }
        }).collect_vec()
    };
    
    while rows.len() < num_rows {
        rows.push(row_builder(rows.last().unwrap()))
    }
    rows.into_iter().flatten().filter(|c| !*c).count()
}

#[allow(dead_code)]
fn parse_input(_input: &str) {}

fn part1(input: &str) -> usize {
    count_tiles(input,40)
}

fn part2(input: &str) -> usize {
    count_tiles(input,400000)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, count_tiles};

    const EXAMPLE: &str = ".^^.^.^^^^";


    #[test]
    fn test_example1_p1() {
        assert_eq!(count_tiles(EXAMPLE, 10), 38)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

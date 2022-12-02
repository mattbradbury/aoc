use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 1);
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
fn parse_input(input: &str) -> Vec<u32> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
}

fn part1(input: &str) -> u32 {
    let nums = parse_input(input);
    let mut sum = 0;
    for i in 0..nums.len() {
        if nums[i] == nums[(i+1)%nums.len()] {
            sum += nums[i]
        }
    };
    sum
}

fn part2(input: &str) -> u32 {
    let nums = parse_input(input);
    let mut sum = 0;
    for i in 0..nums.len() {
        if nums[i] == nums[(i + (nums.len() / 2)) % nums.len()] {
            sum += nums[i]
        }
    };
    sum
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"1122"#;
    const EXAMPLE2: &str = r#"12131415"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 3)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE2), 4)
    }
}

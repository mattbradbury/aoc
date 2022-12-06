use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, collections::VecDeque};

fn main() {
    let input = load_input(2022, 6);
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
fn parse_input(_input: &str) {}

fn part1(input: &str) -> usize {
    let mut chars = input.chars();
    let mut queue =VecDeque::new();
    for _ in 0..3 {
        queue.push_back(chars.next().unwrap())
    };
    let mut counter = 3;
    while let Some(c) = chars.next() {
        counter += 1;
        queue.push_back(c);
        // dbg!(&queue, c, counter);

        if queue.iter().all_unique() {
            return counter
            
        };
        queue.pop_front().unwrap();

    }
    0
}

fn part2(input: &str) -> usize {
    let mut chars = input.chars();
    let mut queue =VecDeque::new();
    for _ in 0..13 {
        queue.push_back(chars.next().unwrap())
    };
    let mut counter = 13;
    while let Some(c) = chars.next() {
        counter += 1;
        queue.push_back(c);
        // dbg!(&queue, c, counter);

        if queue.iter().all_unique() {
            return counter
            
        };
        queue.pop_front().unwrap();

    }
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 7)
    }

    #[test]
    fn test_example2_p1() {
        assert_eq!(part1(EXAMPLE2), 11)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 19)
    }

    #[test]
    fn test_example2_p2() {
        assert_eq!(part2(EXAMPLE2), 26)
    }
}

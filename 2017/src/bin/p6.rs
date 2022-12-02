use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 6);
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
fn parse_input(input: &str) -> Vec<usize> {
    input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect_vec()
}

fn part1(input: &str) -> usize {
    let mut counter = 0;
    let mut history = Vec::new();
    let mut memory = parse_input(input);
    history.push(memory.clone());

    let size = memory.len();
    let loops = loop {
        let mut max = *memory.iter().max().unwrap();
        let mut pos = memory.iter().position(|val| *val == max).unwrap();
        
        memory[pos] = 0;
        while max > 0 {
            pos += 1;
            if pos >= size { pos = 0 };
            memory[pos] += 1;
            max -= 1;
        }
        counter += 1;
        if history.contains(&memory) { break counter };
        history.push(memory.clone());

    };
    // dbg!(history);
    loops
}

fn part2(input: &str) -> usize {
    let mut counter = 0;
    let mut history = Vec::new();
    let mut memory = parse_input(input);
    history.push(memory.clone());

    let size = memory.len();
    let loops = loop {
        let mut max = *memory.iter().max().unwrap();
        let mut pos = memory.iter().position(|val| *val == max).unwrap();
        memory[pos] = 0;
        while max > 0 {
            pos += 1;
            if pos >= size { pos = 0 };
            memory[pos] += 1;
            max -= 1;
        }
        counter += 1;
        if history.contains(&memory) { break counter };
        history.push(memory.clone());

    };
    // dbg!(history);
    let first = history.iter().position(|val| val == &memory).unwrap();
    loops - first
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"0 2 7 0"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 5)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 4)
    }
}

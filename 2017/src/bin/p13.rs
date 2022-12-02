use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 13);
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
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input.trim().lines().map(|line| {
        let mut parts = line.split(": ").map(|n| n.parse().unwrap());
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect_vec()
}


fn part1(input: &str) -> usize {
    let mut scanners = parse_input(input).into_iter();
    let mut severity = 0;
    while let Some((idx, size)) = scanners.next() {
        let period = (size - 1) * 2;
        if idx % period == 0 { severity += idx * size }
    }
    severity
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let mut severity = 0;
    let mut delay = 0;
    loop {
        let mut scanners = input.iter();

        while let Some((idx, size)) = scanners.next() {
            let period = (size - 1) * 2;
            if (delay + idx) % period == 0 { severity += idx * size + 1}
        }
        if severity == 0 { break }
        delay += 1;
        severity = 0;
    }
    delay
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"0: 3
1: 2
4: 4
6: 4
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 24)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 10)
    }
}

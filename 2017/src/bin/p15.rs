use aoc_helper::load_input;
use std::{fmt::Display, time};

fn main() {
    // let input = load_input(2017, 15);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(679, 771)));
    println!("Part2: {}", bench(|| part2(679, 771)));
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

#[derive(Debug)]
struct Generator {
    value: usize,
    factor: usize,
    criteria: usize,
}

impl Generator {
    fn new(seed: usize, factor: usize, criteria: usize) -> Self {
        Self { value: seed, factor, criteria }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // dbg!(&self);
            self.value = (self.value * self.factor) % 2147483647;
            // dbg!(&self);
            // if self.value == 0 { panic!() };
            if self.value % self.criteria == 0 { return Some(self.value)}
        }
    }
}

#[allow(dead_code)]
fn parse_input(_input: &str) {}

fn part1(g1: usize, g2: usize) -> usize {
    let (f1, f2) = (16807, 48271);
    let ts = 2_usize.pow(16);
    dbg!(ts);
    let mut g1 = Generator::new(g1, f1, 1);
    let mut g2 = Generator::new(g2, f2, 1);
    let mut pairs = 0;
    for i in 0..40_000_000 {
        let r1 = g1.next().unwrap();
        let r2 = g2.next().unwrap();
        if i < 5 { println!("{r1:10}  {r2:10}")}
        if r1 % ts == r2 % ts { pairs += 1 }
    }
    pairs
}

fn part2(g1: usize, g2: usize) -> usize {
    let (f1, f2) = (16807, 48271);
    let ts = 2_usize.pow(16);
    dbg!(ts);
    let mut g1 = Generator::new(g1, f1, 4);
    let mut g2 = Generator::new(g2, f2, 8);
    let mut pairs = 0;
    for i in 0..5_000_000 {
        let r1 = g1.next().unwrap();
        let r2 = g2.next().unwrap();
        if i < 5 { println!("{r1:10}  {r2:10}")}
        if r1 % ts == r2 % ts { pairs += 1 }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(65, 8921), 588)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(65, 8921), 309)
    }
}

use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, collections::HashMap};

fn main() {
    let input = load_input(2017, 8);
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
fn parse_input(input: &str) -> Vec<Instr> {
    input.trim().lines().map(|line| {
        let mut parts = line.split_whitespace().collect_vec();
        assert_eq!(parts.len(), 7);
        let reg = parts[0].into();
        let mut amount = parts[2].parse().unwrap();
        if parts[1] == "dec" { amount *= -1 };
        let cond_lhs = parts[4].into();
        let cond = parts[5].try_into().unwrap();
        let cond_rhs = parts[6].parse().unwrap();
        Instr { reg, amount, cond, cond_lhs, cond_rhs }
    }).collect_vec()
}

fn part1(input: &str) -> isize {
    let code = parse_input(input);
    let mut mem = HashMap::new();
    for instr in code {
        let lhs = *mem.entry(instr.cond_lhs).or_default();
        if instr.cond.eval(lhs, instr.cond_rhs) {
            *mem.entry(instr.reg).or_default() += instr.amount;
        }
    }
    *mem.values().max().unwrap()
}

fn part2(input: &str) -> isize {
    let code = parse_input(input);
    let mut mem = HashMap::new();
    let mut highest = 0;
    for instr in code {
        let lhs = *mem.entry(instr.cond_lhs).or_default();
        if instr.cond.eval(lhs, instr.cond_rhs) {
            *mem.entry(instr.reg).or_default() += instr.amount;
        }
        let max = *mem.values().max().unwrap();
        if max > highest { highest = max }

    }
    highest
}

struct Instr {
    reg: String,
    amount: isize,
    cond: Conditional,
    cond_lhs: String,
    cond_rhs: isize,
    
}

#[derive(Debug, Clone, Copy)]
enum Conditional {
    LT,
    LE,
    EQ,
    NE,
    GT,
    GE,
}

impl Conditional {
    fn eval(&self, lhs: isize, rhs: isize) -> bool {
        match self {
            Conditional::LT => lhs <  rhs,
            Conditional::LE => lhs <= rhs,
            Conditional::EQ => lhs == rhs,
            Conditional::NE => lhs != rhs,
            Conditional::GT => lhs >  rhs,
            Conditional::GE => lhs >= rhs,
        }
    }
}

impl TryFrom<&str> for Conditional {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Conditional::*;
        let res = match value {
            "<"  => { LT }
            "<=" => { LE }
            "==" => { EQ }
            "!=" => { NE }
            ">"  => { GT }
            ">=" => { GE }
            _ => { return Err(()) }
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 1)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 10)
    }
}

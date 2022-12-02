use itertools::Itertools;
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::{fmt::Display, str::FromStr, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 12);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegInt {
    Reg(Reg),
    Int(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Reg(usize);
impl Reg {
    fn char(c: char) -> Reg {
        Reg(c as usize - 'a' as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instr {
    Cpy(RegInt, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(RegInt, isize),
}
impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_ascii_whitespace().collect_vec();
        let res = match parts[0] {
            "cpy" => {
                let arg1 = match parts[1].parse::<isize>() {
                    Ok(v) => RegInt::Int(v),
                    Err(_) => RegInt::Reg(Reg::char(parts[1].chars().next().unwrap())),
                };
                let arg2 = Reg::char(parts[2].chars().next().unwrap());
                Instr::Cpy(arg1, arg2)
            }
            "inc" => {
                let arg1 = Reg::char(parts[1].chars().next().unwrap());
                Instr::Inc(arg1)
            }
            "dec" => {
                let arg1 = Reg::char(parts[1].chars().next().unwrap());
                Instr::Dec(arg1)
            }
            "jnz" => {
                let arg1 = match parts[1].parse::<isize>() {
                    Ok(v) => RegInt::Int(v),
                    Err(_) => RegInt::Reg(Reg::char(parts[1].chars().next().unwrap())),
                };
                let arg2 = parts[2].parse::<isize>().unwrap();
                Instr::Jnz(arg1, arg2)
            }
            _ => panic!(),
        };
        Ok(res)
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(core::str::FromStr::from_str)
        .map(Result::unwrap)
        .collect_vec()
}

fn run(code: Vec<Instr>, mut reg: [isize; 4]) -> [isize; 4] {
    let mut ptr = 0;
    while let Some(instr) = code.get(ptr) {
        // dbg!(reg);
        match instr {
            Instr::Cpy(ri, r) => {
                let arg1 = match ri {
                    RegInt::Reg(r) => reg[r.0],
                    RegInt::Int(i) => *i,
                };
                reg[r.0] = arg1;
            }
            Instr::Inc(r) => reg[r.0] += 1,
            Instr::Dec(r) => reg[r.0] -= 1,
            Instr::Jnz(ri, dist) => {
                let arg1 = match ri {
                    RegInt::Reg(r) => reg[r.0],
                    RegInt::Int(i) => *i,
                } as usize;
                if arg1 != 0 {
                    ptr = (ptr as isize + dist) as usize;
                    continue;
                }
            }
        }

        ptr += 1;
    }

    reg
}

fn part1(input: &str) -> isize {
    let input = parse_input(input);
    let reg: [isize; 4] = [0, 0, 0, 0];

    let output = run(input, reg);
    output[0]
}

fn part2(input: &str) -> isize {
    let input = parse_input(input);
    let reg: [isize; 4] = [0, 0, 1, 0];

    let output = run(input, reg);
    output[0]
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 42)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

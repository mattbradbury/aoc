use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, str::FromStr, collections::HashMap};

fn main() {
    let input = load_input(2017, 18);
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

#[derive(Debug, Clone, Copy)]
enum RegNum {
    Reg(char),
    Num(isize),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Set(char, RegNum),
    Add(char, RegNum),
    Mul(char, RegNum),
    Mod(char, RegNum),
    Snd(char),
    Rcv(char),
    Jgz(char, RegNum),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instr::*;
        let parts = s.trim().split_ascii_whitespace().collect_vec();
        let reg = parts[1].chars().next().unwrap();
        let reg_or_num = |s: &str| {
            match s.parse() {
                Ok(n) => RegNum::Num(n),
                Err(_) => { RegNum::Reg(s.chars().next().unwrap()) },
            }
        };
        let res = match parts[0] {
            "set" => { Set(reg, reg_or_num(parts[2])) },
            "add" => { Add(reg, reg_or_num(parts[2])) },
            "mul" => { Mul(reg, reg_or_num(parts[2])) },
            "jgz" => { Jgz(reg, reg_or_num(parts[2])) },
            "mod" => { Mod(reg, reg_or_num(parts[2])) },
            "snd" => { Snd(reg) },
            "rcv" => { Rcv(reg) }
            _ => { return Err(()) }
            
        };
        Ok(res)
    }
}

enum Status {
    Finished,
    Rcv(isize),
}
struct Cpu {
    pc: usize,
    reg:  HashMap<char, isize>,
    code: Vec<Instr>,
    buf: Vec<isize>,
    sound: isize,
}

impl Cpu {
    fn new(code: Vec<Instr>) -> Self {
        Self { 
            pc: 0,
            reg: HashMap::new(),
            buf: Vec::new(),
            sound: 0,
            code
        }
    }

    fn r_n(&self, rn: RegNum) -> isize {
        match rn {
            RegNum::Reg(r) => *(self.reg.get(&r)).unwrap_or(&0),
            RegNum::Num(n) => n,
        }
    }

    fn setr(&mut self, r:char, v: isize) {
        self.reg.insert(r, v);
    }

    fn getr(&mut self, r: char) -> isize {
        *self.reg.entry(r).or_insert(0)
    }

    fn run(&mut self) -> Status {
        let mut counter = 0;
        while self.pc < self.code.len() {
            // dbg!(&self.pc, &self.reg);
            // dbg!(self.code[self.pc]);
            // dbg!(counter);
            counter += 1;
            match self.code[self.pc] {
                Instr::Set(r, rn) => {
                    self.setr(r, self.r_n(rn));
                    self.pc += 1;
                },
                Instr::Add(r, rn) => {
                    let lhs = self.getr(r);
                    let rhs = self.r_n(rn);
                    self.setr(r, lhs + rhs);
                    self.pc += 1;

                },
                Instr::Mul(r, rn) => {
                    let lhs = self.getr(r);
                    let rhs = self.r_n(rn);
                    self.setr(r, lhs * rhs);
                    self.pc += 1;

                },
                Instr::Mod(r, rn) => {
                    let lhs = self.getr(r);
                    let rhs = self.r_n(rn);
                    self.setr(r, lhs % rhs);
                    self.pc += 1;

                },

                Instr::Snd(r) => {
                    let val = self.getr(r);
                    self.sound = val;
                    self.pc += 1;

                },

                Instr::Rcv(r) => {
                    let val = self.getr(r);
                    if val > 0 {return Status::Rcv(self.sound) }
                    // if val > 0 { self.buf.push(self.sound) }
                    self.pc += 1;
                },
                Instr::Jgz(r, rn) => {
                    let lhs = self.getr(r);
                    let rhs = self.r_n(rn);
                    if lhs > 0 {
                        let pc = self.pc as isize + rhs;
                        if pc < 0 || pc > self.code.len() as isize { break };
                        self.pc = pc as usize;
                    } else {             
                        self.pc += 1;
                    }
                },

            }
        }
        Status::Finished
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Instr> {
    input.trim().lines().map(str::parse).map(Result::unwrap).collect_vec()
}

fn part1(input: &str) -> isize {
    let code = parse_input(input);
    // dbg!(code);
    let mut cpu = Cpu::new(code);
    match cpu.run() {
        Status::Finished => todo!(),
        Status::Rcv(val) => val,
    }

    // *cpu.buf.first().unwrap()
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 4)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

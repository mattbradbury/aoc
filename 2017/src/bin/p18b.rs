use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, str::FromStr, collections::{HashMap, VecDeque}};

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
    Jgz(RegNum, RegNum),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instr::*;
        let parts = s.trim().split_ascii_whitespace().collect_vec();
        let reg = parts[1].chars().next().unwrap();
        if !reg.is_alphabetic() { dbg!(&parts, &reg); };
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
            "jgz" => { Jgz(reg_or_num(&reg.to_string()), reg_or_num(parts[2])) },
            "mod" => { Mod(reg, reg_or_num(parts[2])) },
            "snd" => { Snd(reg) },
            "rcv" => { Rcv(reg) }
            _ => { return Err(()) }
            
        };
        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Stopped,
    MsgWait,
    Finished,
    Msg(isize),
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Start(isize),
    Resume,
    ResumeMsg,
}
struct Cpu {
    pc: usize,
    reg:  HashMap<char, isize>,
    code: Vec<Instr>,
    buf: VecDeque<isize>,
}

impl Cpu {
    fn new(code: Vec<Instr>) -> Self {
        Self { 
            pc: 0,
            reg: HashMap::new(),
            buf: VecDeque::new(),
            code
        }
    }

    fn r_n(&self, rn: RegNum) -> isize {
        match rn {
            // don't use self.getr so we don't need mutable access
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

    fn run(&mut self, action: Action) -> Status {
        match action {
            Action::Start(id) => { 
                self.pc = 0; 
                self.reg.clear(); 
                self.reg.insert('p', id);
            },
            Action::Resume => {},
            Action::ResumeMsg => { }, // just resume on the same instruction
        }
        let mut counter = 0;
        while self.pc < self.code.len() 
            // && counter < 100
        {
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
                    let value = self.getr(r);
                    // self.buf.push(value);
                    self.pc += 1;
                    return Status::Msg(value);
                    // self.pc += 1;

                },

                Instr::Rcv(r) => {
                
                    if let Some(msg) = self.buf.pop_front() {
                        self.setr(r, msg);
                        self.pc += 1;
                    } else {
                        return Status::MsgWait 
                    }
                
                    // if val > 0 { self.buf.push(self.sound) }
                    self.pc += 1;
                },
                Instr::Jgz(r, rn) => {
                    let lhs = self.r_n(r);
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

    let mut cpu = Vec::new(); 
    cpu.push(Cpu::new(code.clone()));
    cpu.push(Cpu::new(code));

    let mut action = Vec::new();
    action.push(Action::Start(0));
    action.push(Action::Start(1));

    let mut status = vec![Status::Stopped; 2];

    let mut p1sends = 0;
    let mut cur_id = 0;
    loop {
        if status[cur_id] == Status::Finished { continue; }
        status[cur_id] = cpu[cur_id].run(action[cur_id]);
        println!("Context switch");
        dbg!(cur_id, &status);
        match status[cur_id] {
            Status::Stopped => { unreachable!() },
            Status::Finished => { 
                if status[cur_id ^ 1] == Status::Finished { break };
                if status[cur_id ^ 1] == Status::MsgWait { break };

                status[cur_id] = Status::Finished;
                cur_id ^= 1;
             },
            Status::MsgWait => { 
                if status[cur_id ^ 1] == Status::Finished { break };
                if status[cur_id ^ 1] == Status::MsgWait 
                    && cpu[0].buf.is_empty() && cpu[1].buf.is_empty()
                { break };
                action[cur_id] = Action::ResumeMsg;
                cur_id ^= 1;
            },
            Status::Msg(msg) => {  
                cpu[cur_id ^ 1].buf.push_back(msg);
                action[cur_id] = Action::Resume;
                if cur_id == 1 { 
                    p1sends += 1;
                    if p1sends % 100_000 == 0 { println!("{p1sends}") };
                }
            },
        }
    
         
    };
    p1sends
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

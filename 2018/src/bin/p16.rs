use std::{time, collections::HashMap};

use aoc_helper::load_input;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: usize,
    args: [usize; 3],
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        
        let input = value
            .trim()
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>,_>>()?;
        Ok(Instruction {
            opcode: input[0],
            args:input[1..4].try_into()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Sample {
    before: [usize; 4],
    instr: Instruction,
    after: [usize; 4],
}

impl TryFrom<&str> for Sample {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        Ok(Sample {
            before: lines.next().unwrap()
                .split('[')
                .nth(1).unwrap()
                .trim_end_matches(']')
                .split(',')
                .map(str::trim)
                .map(str::parse)
                .collect::<Result<Vec<_>,_>>()?
                .as_slice()
                .try_into()?,
            instr: lines.next().unwrap()
                .try_into()?,
            after: lines.next().unwrap()
                .split('[')
                .nth(1).unwrap()
                .trim_end_matches(']')
                .split(',')
                .map(str::trim)
                .map(str::parse)
                .collect::<Result<Vec<_>,_>>()?
                .as_slice()
                .try_into()?,
        })
    }
}

type Operation = fn([usize; 4], [usize; 3]) -> [usize;4];


fn main() {
    let input = load_input(2018, 16);
    let input = parse_input(&input);
    bench(|| part1(&input.0));
    bench(|| part2(input));
}

fn bench<F>(f: F) where F: FnOnce() {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

#[allow(dead_code)]
fn parse_input(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let (part1, part2) = input.split_once("\n\n\n\n").unwrap();
    let samples = part1.split("\n\n")
        .map(|v| {
            println!("{}", v);
            v.try_into()
        })
        .collect::<Result<_,_>>()
        .unwrap();

    let instrs = part2
        .trim()
        .lines()
        .map(Instruction::try_from)
        .map(Result::unwrap)
        .collect::<Vec<_>>();



    (samples, instrs)
}

fn build_func_vec() -> Vec<(String, Operation)> {
    let mut funcs = Vec::new();
    let addr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] + i[args[1]]; i };
    let addi: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] + args[1]; i };
    let mulr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] * i[args[1]]; i };
    let muli: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] * args[1]; i };
    let banr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] & i[args[1]]; i };
    let bani: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] & args[1]; i };
    let borr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] | i[args[1]]; i };
    let bori: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]] | args[1]; i };
    let setr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = i[args[0]]; i };
    let seti: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = args[0]; i };
    let gtir: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = if args[0] > i[args[1]] {1} else {0}; i };
    let gtri: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = if i[args[0]] > args[1] {1} else {0}; i };
    let gtrr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = if i[args[0]] > i[args[1]] {1} else {0}; i };
    let eqir: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = if args[0] == i[args[1]] {1} else {0}; i };
    let eqri: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = if i[args[0]] == args[1] {1} else {0}; i };
    let eqrr: Operation = 
        |mut i:[usize; 4], args:[usize; 3]| { i[args[2]] = if i[args[0]] == i[args[1]] {1} else {0}; i };


    funcs.push(("addr".to_owned(),addr));
    funcs.push(("addi".to_owned(),addi));
    funcs.push(("mulr".to_owned(),mulr));
    funcs.push(("muli".to_owned(),muli));
    funcs.push(("banr".to_owned(),banr));
    funcs.push(("bani".to_owned(),bani));
    funcs.push(("borr".to_owned(),borr));
    funcs.push(("bori".to_owned(),bori));
    funcs.push(("setr".to_owned(),setr));
    funcs.push(("seti".to_owned(),seti));
    funcs.push(("gtir".to_owned(),gtir));
    funcs.push(("gtri".to_owned(),gtri));
    funcs.push(("gtrr".to_owned(),gtrr));
    funcs.push(("eqir".to_owned(),eqir));
    funcs.push(("eqri".to_owned(),eqri));
    funcs.push(("eqrr".to_owned(),eqrr));
    
    funcs
}

fn part1(input: &[Sample]) {
    let funcs = build_func_vec();
    let count = input
        .iter()
        .filter(|s| oper_match(funcs.clone(), *s).len() >= 3)
        .count();
    println!("Part1: {}", count);
}

fn oper_match(mut opers: Vec<(String, Operation)>, sample: &Sample) -> Vec<(String, Operation)> {
    opers.retain(|(_, o)| {
        o(sample.before, sample.instr.args) == sample.after
    });
    opers
}

fn part2((mut samples, instrs) : (Vec<Sample>, Vec<Instruction>)) {
    let mut opers = build_func_vec();
    let mut mapping = HashMap::new();
    while opers.len() > 0 {
        println!("Mapping: {:#?}", mapping);

        let (matches, sample) = samples
            .iter()
            .find_map(|sample| { 
                let ops = opers.clone();
                let matches = oper_match(ops, sample);
                if matches.len() == 1 {
                    Some((matches, sample))
                } else { None }
            }).unwrap();
        let sample = *sample;
        mapping.insert(sample.instr.opcode, matches[0].1);
        opers.retain(|(name, _)| *name != matches[0].0);
        samples.retain(|s| s.instr.opcode != sample.instr.opcode);
    }

    // let mut regs = [0,0,0,0];
    let regs = instrs
        .iter()
        .fold([0,0,0,0], |r, i| {
            mapping[&i.opcode](r, i.args)
        });
    println!("Part2: {:?}", regs)
}

#[cfg(test)]
mod tests {
    const _EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1() {
        // assert_eq!(x,0);
    }
}

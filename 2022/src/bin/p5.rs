use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, str::FromStr};

fn main() {
    let input = load_input(2022, 5);
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
struct Rule {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Rule {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split_ascii_whitespace();
        let amount = s.nth(1).unwrap().parse().unwrap();
        let from = s.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = s.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        Ok(Rule { amount, from, to })
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Rule>) {
    let (boxes, rules) = input.split_once("\n\n").unwrap();
    let mut boxes = boxes.lines().rev();
    let nums = boxes.next().unwrap().trim().split_ascii_whitespace();
    let num_stacks = nums.last().unwrap().parse::<usize>().unwrap();

    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    while let Some(line) = boxes.next() {
        let  line = line.chars().collect_vec();
        
        for i in 0..num_stacks {
            let c = line[i * 4 + 1];
            if c != ' ' { stacks[i].push(c) }
        }

    }
    let rules = rules.lines()
        .map(|line| { line.parse().unwrap() })
        .collect_vec();
    (stacks, rules)
}

fn part1(input: &str) -> String {
    let (mut stack, rules) = parse_input(input);
    for rule in rules {
        for _ in 0..rule.amount {
            let item = stack[rule.from].pop().unwrap();
            stack[rule.to].push(item);
        }
    }
    let mut ret = String::new();
    for column in stack {
        ret.push(*column.last().unwrap());
    };
    ret
}

fn part2(input: &str) -> String {
    let (mut stack, rules) = parse_input(input);
    for rule in rules {
        let mut temp = Vec::new();
        for _ in 0..rule.amount {
            // let item = stack[rule.from].pop().unwrap();
            temp.push(stack[rule.from].pop().unwrap());
            // stack[rule.to].push(item);
        };
        for _ in 0..rule.amount {
            stack[rule.to].push(temp.pop().unwrap());
        };
    }
    let mut ret = String::new();
    for column in stack {
        ret.push(*column.last().unwrap());
    };
    ret
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), "CMZ".to_owned())
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), "MCD".to_owned())
    }
}

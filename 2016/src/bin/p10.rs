use core::panic;
use itertools::Itertools;
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::{collections::HashMap, fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 10);
    let _input2 = r#"value 5 goes to bot 2
    bot 2 gives low to bot 1 and high to bot 0
    value 3 goes to bot 1
    bot 1 gives low to output 1 and high to bot 0
    bot 0 gives low to output 2 and high to output 0
    value 2 goes to bot 2"#;
    // let input = parse_input(&input);
    // println!("Part1: {}", bench(|| part1(input2)));
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

#[derive(Debug)]

enum Bot {
    Empty,
    One(usize),
    Two { low: usize, high: usize },
}

impl Bot {
    fn give(&mut self, val: usize) {
        let newbot = match self {
            Bot::Empty => Bot::One(val),
            Bot::One(held) => {
                if *held < val {
                    Bot::Two {
                        low: *held,
                        high: val,
                    }
                } else {
                    Bot::Two {
                        low: val,
                        high: *held,
                    }
                }
            }
            Bot::Two { low: _, high: _ } => panic!(),
        };
        // std::mem::replace(self, newbot);
        *self = newbot;
    }

    fn take(&mut self) -> Option<(usize, usize)> {
        let ret = match &self {
            Bot::Empty => return None,
            Bot::One(_) => return None,
            Bot::Two { low, high } => Some((*low, *high)),
        };
        *self = Bot::Empty;
        ret
    }
}

#[derive(Debug)]
enum Dest {
    Bot(usize),
    Bin(usize),
}

impl Dest {
    fn new(name: &str, val: usize) -> Self {
        match name {
            "output" => Dest::Bin(val),
            "bot" => Dest::Bot(val),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Value { val: usize, bot: usize },
    Gives { bot: usize, low: Dest, high: Dest },
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Rule> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (first, rest) = line.trim().split_once(' ').unwrap();
            match first {
                "value" => {
                    let (val, bot) = scan_fmt!(rest, "{d} goes to bot {d}", usize, usize).unwrap();
                    Rule::Value { val, bot }
                }
                "bot" => {
                    let (bot, low, low_val, high, high_val) = scan_fmt!(
                        rest,
                        "{d} gives low to {} {d} and high to {} {d}",
                        usize,
                        String,
                        usize,
                        String,
                        usize
                    )
                    .unwrap();
                    let low = Dest::new(&low, low_val);
                    let high = Dest::new(&high, high_val);
                    Rule::Gives { bot, low, high }
                }
                _ => panic!(),
            }
        })
        .collect_vec()
}

fn run(mut rules: Vec<Rule>) -> usize {
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
    let mut rules_len = rules.len();
    dbg!(rules.len());
    // let mut count = 0;

    loop {
        rules.retain(|rule| {
            match rule {
                Rule::Value { val, bot } => {
                    bots.entry(*bot).or_insert(Bot::Empty).give(*val);
                    return false;
                }
                Rule::Gives { bot, low, high } => {
                    let taken = bots.entry(*bot).or_insert(Bot::Empty).take();
                    match taken {
                        Some((low_val, high_val)) => {
                            dbg!(rule);
                            // if low_val == 17 && high_val == 61 {
                            //     return outputs[&0] as usize  * outputs[&1] as usize  * outputs[&2] as usize
                            //     // return *bot
                            // }
                            for (chip, val) in [(low, low_val), (high, high_val)] {
                                dbg!(chip);
                                match chip {
                                    Dest::Bot(bot) => {
                                        bots.entry(*bot).or_insert(Bot::Empty).give(val);
                                    }
                                    Dest::Bin(bin) => {
                                        dbg!(bin);
                                        outputs.insert(*bin, val);
                                    }
                                }
                            }
                            return false;
                        }
                        None => {}
                    }
                }
            }

            true
        });
        // rules.retain(|rule| match rule {
        //     Rule::Value { val, bot } => false,
        //     Rule::Gives { bot, low, high } => true,
        // });
        dbg!(&outputs);
        // dbg!(&bots);
        // dbg!(&rules.len());
        if rules.len() == rules_len {
            break;
        }
        rules_len = rules.len();
        // count += 1;
        // if count > 10 { break }
    }
    return outputs[&0] as usize * outputs[&1] as usize * outputs[&2] as usize;
}

fn part1(input: &str) -> usize {
    let rules = parse_input(input);
    // dbg!(rules);
    run(rules)
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 0)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

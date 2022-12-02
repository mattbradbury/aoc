use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, str::FromStr};


fn main() {
    let input = load_input(2022, 2);
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

enum Action {
    Lose,
    Tie,
    Win
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "X" => Ok(Action::Lose),
            "Y" => Ok(Action::Tie),
            "Z" => Ok(Action::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2, 
    Scissor = 3
}

impl FromStr for Hand {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(())
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => std::cmp::Ordering::Equal,
            (Hand::Rock, Hand::Paper) => std::cmp::Ordering::Less,
            (Hand::Rock, Hand::Scissor) => std::cmp::Ordering::Greater,
            (Hand::Paper, Hand::Rock) => std::cmp::Ordering::Greater,
            (Hand::Paper, Hand::Paper) => std::cmp::Ordering::Equal,
            (Hand::Paper, Hand::Scissor) => std::cmp::Ordering::Less,
            (Hand::Scissor, Hand::Rock) => std::cmp::Ordering::Less,
            (Hand::Scissor, Hand::Paper) => std::cmp::Ordering::Greater,
            (Hand::Scissor, Hand::Scissor) =>std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
   
    fn find_hand_by_action(&self, action: &Action) -> Hand {
        use Hand::*;
        match (self, action) {
            (Hand::Rock, Action::Lose) => Scissor,
            (Hand::Rock, Action::Tie) => Rock,
            (Hand::Rock, Action::Win) => Paper,
            (Hand::Paper, Action::Lose) => Rock,
            (Hand::Paper, Action::Tie) => Paper,
            (Hand::Paper, Action::Win) => Scissor,
            (Hand::Scissor, Action::Lose) => Paper,
            (Hand::Scissor, Action::Tie) => Scissor,
            (Hand::Scissor, Action::Win) => Rock,
        }
    }
}


#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<(Hand, Hand)> {
    input.lines().map(|l| {
        let mut parts = l.split_ascii_whitespace();
        (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
    }).collect_vec()
}

fn parse_input2(input: &str) -> Vec<(Hand, Action)> {
    input.lines().map(|l| {
        let mut parts = l.split_ascii_whitespace();
        (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
    }).collect_vec()
}


fn part1(input: &str) -> usize {
    let tourney = parse_input(input);
    tourney.into_iter().map(|(them, me)| {
        match me.cmp(&them) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        }
    } + me as usize).sum()
    
}

fn part2(input: &str) -> usize {
    let tourney = parse_input2(input);
    let mut me = Hand::Rock;
    tourney.into_iter().map(|(them, action)| {
        me = them.find_hand_by_action(&action);
        match me.cmp(&them) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        }
    } + me as usize).sum()
    
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"A Y
B X
C Z
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 15)
    }

    #[test] 
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 12)
    }
}

use std::{time, fmt::Display};

use aoc_helper::load_input;
use itertools::Itertools;

enum ChunkError {
    Incomplete(Vec<char>),
    Corrupt(char),
}

fn main() {
    let input = load_input(2021, 10);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(&input)));
}

fn bench<F, T>(f: F) -> T where F: FnOnce() -> T , T:Display {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    
    parse_input(&input).filter_map(|ce| {
        match ce {
            ChunkError::Incomplete(_) => None,
            ChunkError::Corrupt(c) => {
                Some(match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => {
                        println!("c = '{:?}'", c);
                        panic!()
                    },
                })
            }
        }
    })
    .sum()
}


fn part2(input: &str) -> usize {
    let mut results = parse_input(input)
        .filter_map(|ce| {
            match ce {
                ChunkError::Corrupt(_) => None,
                ChunkError::Incomplete(mut stack) => {
                    stack.reverse();
                    let ret = stack.into_iter().fold(0, |acc, c| {
                        acc * 5 + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4, 
                            _ => panic!(),
                        }
                    });
                    Some(ret)
                }
            }
                
        })
        .collect_vec();
    results.sort();
    results[results.len()/2]
}

fn pair(a: char) -> char {
    let flip = "{<([".chars().collect_vec();
    flip["}>)]".chars().position(|i| i==a).unwrap()]
    
}

fn parse_input<'a>(input: &'a str) -> impl 'a +  Iterator<Item=ChunkError>
{
    input.trim()
        .lines()
        .map(|l| {
            let mut stack = Vec::new();
            for c in l.trim().chars() {
                match c {
                    '[' | '(' | '<' | '{' => { stack.push(c) },
                    _ => { 
                        // let top = stack.pop();
                        match stack.pop() {
                            Some(top) => {
                                if top != pair(c) { 
                                    return ChunkError::Corrupt(c) }
                                },
                            None => return ChunkError::Corrupt(c),
                        }
                    }
                }
            };
            return ChunkError::Incomplete(stack);
            
        })
        
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]
    "#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 26397)
    }

    #[test]
    fn test_example2_p1() {
        let a = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(part1(a), 1197);
        let a = "[[<[([]))<([[{}[[()]]]";
        assert_eq!(part1(a), 3);
        let a = "[{[{({}]{}}([{[{{{}}([]";
        assert_eq!(part1(a), 57);
        let a = "[<(<(<(<{}))><([]([]()";
        assert_eq!(part1(a), 3);
        let a = "<{([([[(<>()){}]>(<<{{";
        assert_eq!(part1(a), 25137);
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 288957)
    }
}

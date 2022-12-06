use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, io::Read, collections::HashSet};

fn main() {
    let input = load_input(2022, 3);
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
fn parse_input(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    input.lines().map(|line| {
        let bytes = line.trim().as_bytes().bytes().map(|b| {
            let b = b.unwrap();
            match b {
                65..=90 => { b - 64  + 26 },
                97..=122 => { b - 96 },
                
                _ => panic!()
            } 
            
        }).collect_vec();
            
       

        let (first, second) = bytes.split_at(bytes.len()/2);
        (first.bytes().map(Result::unwrap).collect(), second.bytes().map(Result::unwrap).collect())
    }).collect_vec()
}

fn parse_input2(input: &str) -> Vec<HashSet<u8>> {
    input.lines().map(|line| {
        line.trim().as_bytes().bytes().map(|b| {
            let b = b.unwrap();
            match b {
                65..=90 => { b - 64  + 26 },
                97..=122 => { b - 96 },
                
                _ => panic!()
            } 
            
        }).collect()
    }).collect_vec()
}

fn part1(input: &str) -> usize {
    let sacks = parse_input(input);
    sacks.into_iter().map(|(a,b)| {
        let m = a.intersection(&b)
            .into_iter()
            .max()
            .unwrap()
            .clone();
        dbg!(m) as usize
            
    }).sum::<usize>()
    
}

fn part2(input: &str) -> usize {
    let sacks = parse_input2(input);
    sacks.into_iter().tuples().map(|(a,b,c)| {
        let m = a.intersection(&b).map(|y| y.to_owned()).collect::<HashSet<_>>()
            .intersection(&c).map(|y| y.to_owned()).collect::<HashSet<_>>()
   
            .into_iter()
            .max()
            .unwrap()
            .clone();
        dbg!(m) as usize
            
    }).sum::<usize>()
    
}



#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 157)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 70)
    }
}

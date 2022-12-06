use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, ops::RangeInclusive};

fn main() {
    let input = load_input(2022, 4);
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
fn parse_input(input: &str) -> Vec<Vec<RangeInclusive<usize>>> {
    input.lines().map(|line| {
        line.trim()
            .split(',')
            .map(|part| {
                let nums = part.split('-')
                    .map(|n| n.parse::<usize>().unwrap()).collect_vec();
                RangeInclusive::new(nums[0], nums[1])
            }).collect_vec()
        
        
            
    }).collect_vec()
}

fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs.into_iter().filter(|pair| {
        for i in 0..=1 {
            if pair[i].contains(pair[1-i].start()) 
                & pair[i].contains(pair[1-i].end()) { return true }
        }
        false
    }).count()
    
}

fn part2(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs.into_iter().filter(|pair| {
        for i in 0..=1 {
            if pair[i].contains(pair[1-i].start()) { return true };
            if pair[i].contains(pair[1-i].end()) { return true };
        }
        false
    }).count()
    
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 2)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 4)
    }
}

use aoc_helper::load_input;
use itertools::{Itertools, MinMaxResult};
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 2);
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
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.trim().lines().map(|line| line.split_whitespace().map(|chunk| chunk.parse().unwrap()).collect_vec()).collect_vec()
}

fn part1(input: &str) -> usize {
    let rows = parse_input(input);
    rows.iter()
        .map(|row| row.iter().minmax())
        .map(|lh| { 
            if let MinMaxResult::MinMax(l,h) = lh {
                h - l
            }
            else { 0 }
        }).sum()
    
}

fn part2(input: &str) -> usize {
    let rows = parse_input(input);
    rows.iter()
        .map(|row| row.iter().permutations(2).filter_map(|ab| {
            // println!("perm {} {}", ab[0], ab[1]);
            if ab[0] % ab[1] == 0 { Some(ab[0] / ab[1] )} else { Some(0) }
        }).sum::<usize>()
    ).sum()    
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"5 1 9 5
7 5 3
2 4 6 8
"#;    

    const EXAMPLE2: &str = 
r#"5 9 2 8
9 4 7 3
3 8 6 5
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 18)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE2), 9)
    }
}

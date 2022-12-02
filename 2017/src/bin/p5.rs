use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 5);
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
fn parse_input(input: &str) -> Vec<isize> {
    input.trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let mut maze = parse_input(input);
    let mut loc = 0;
    let mut counter = 0;
    loop {
        counter += 1;

        let next = loc as isize + maze[loc];
        if next < 0 || next as usize >= maze.len() { break counter };
        maze[loc] += 1;
        loc = next as usize;

    }
    
}

fn part2(input: &str) -> usize {
    let mut maze = parse_input(input);
    let mut loc = 0;
    let mut counter = 0;
    loop {
        counter += 1;
        let offset = if maze[loc] >= 3 { -1 } else { 1 };
        let next = loc as isize + maze[loc];
        if next < 0 || next as usize >= maze.len() { break counter };
        maze[loc] += offset;
        loc = next as usize;

    }
    
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"0
3
0
1
-3
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 5)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 10)
    }
}

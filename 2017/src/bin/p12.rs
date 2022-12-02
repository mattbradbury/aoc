use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 12);
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
    input.trim().lines().map(|line| {
        let mut parts = line.split(" <-> ");
        parts.next().unwrap();
        parts.next().unwrap().split(", ").map(|num| num.parse().unwrap()).collect_vec()
    }).collect_vec()
}

fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    let mut reachable = vec![false; graph.len()];
    let mut queue = graph[0].clone();
    reachable[0] = true;
    while let Some(program) = queue.pop() {
        reachable[program] = true;
        for n in &graph[program] {
            if !reachable[*n] { queue.push(*n); reachable[*n] = true }
        }
    };
    reachable.iter().filter(|v| **v).count()
    
}

fn part2(input: &str) -> usize {
    let graph = parse_input(input);
    let mut reachable = vec![false; graph.len()];
    let mut groups = Vec::new();

    while let Some((pos,_)) = reachable.iter().find_position(|u| !**u) {
        let mut queue = graph[pos].clone();
        reachable[pos] = true;
        groups.push(pos);
        while let Some(program) = queue.pop() {
            reachable[program] = true;
            for n in &graph[program] {
                if !reachable[*n] { queue.push(*n); reachable[*n] = true }
            }
        };
    
    }
    groups.len()
    
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 6)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

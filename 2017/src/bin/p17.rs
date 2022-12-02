use aoc_helper::load_input;
use std::{fmt::Display, time};

fn main() {
    // let input = load_input(2017, 0);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(386)));
    println!("Part2: {}", bench(|| part2(386)));
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
fn parse_input(_input: &str) {}

fn part1(skip: usize) -> usize {
    let mut lock = Vec::with_capacity(2018);
    lock.push(0);
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + skip) % i + 1;
        lock.insert(pos, i);
    }
    let pos = (pos + 1) % 2018;
    lock[pos]
}


fn part2(skip: usize) -> usize {
    let mut lock = Vec::with_capacity(2);
    lock.push(0);
    let mut pos1 = 0;
    let mut pos = 0;
    for i in 1..50_000_000 {
        pos = (pos + skip) % i + 1;
        // lock.insert(pos, i);
        if pos == 1 { pos1 = i; println!("Counter; {i}   Pos1:{pos1}")};
        if i % 1_000_000 == 0 { println!("Counter: {i}")}
    }
    let pos = (pos + 1) % 2018;
    pos1
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(3), 638)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(3), 0)
    }
}

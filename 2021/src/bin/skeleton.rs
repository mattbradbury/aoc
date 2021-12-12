use std::{time, fmt::Display};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2021, 0);
    // let input = parse_input(&input);
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
fn parse_input(_input: &str)  {

}

fn part1(_input: &str) -> usize {
    0
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
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

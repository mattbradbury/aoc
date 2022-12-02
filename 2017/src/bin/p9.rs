use aoc_helper::load_input;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 9);
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
fn parse_input(_input: &str) {

}

#[derive(Debug, Clone, Copy)]
enum State {
    Level(usize),
    Garbage,
    Bang,
}

fn part1(input: &str) -> usize {
    let mut score = 0;
    let mut stack = vec![State::Level(0)];
    
    input.chars().for_each(|c| {
        use State::*;
        match (*stack.last().unwrap(), c) {
            (Level(depth), '{') => { stack.push(Level(depth + 1)) }
            (Level(depth), '}') => { stack.pop(); score += depth }
            (Level(_), '<') =>            { stack.push(Garbage) }
            (Level(_), _) =>              {}
            (Garbage, '>') =>             { stack.pop(); }
            (Garbage, '!') =>             { stack.push(Bang) }
            (Garbage, _) =>               {}
            (Bang, _) =>                  { stack.pop(); }

            (_, _) => { dbg!(&stack, c); }
        }

    });
    
    score
}

fn part2(input: &str) -> usize {
    let mut score = 0;
    let mut garbage = 0;
    let mut stack = vec![State::Level(0)];
    
    input.chars().for_each(|c| {
        use State::*;
        match (*stack.last().unwrap(), c) {
            (Level(depth), '{') => { stack.push(Level(depth + 1)) }
            (Level(depth), '}') => { stack.pop(); score += depth }
            (Level(_), '<') =>            { stack.push(Garbage) }
            (Level(_), _) =>              {}
            (Garbage, '>') =>             { stack.pop(); }
            (Garbage, '!') =>             { stack.push(Bang) }
            (Garbage, _) =>               { garbage += 1 }
            (Bang, _) =>                  { stack.pop(); }

            (_, _) => { dbg!(&stack, c); }
        }

    });
    
    garbage
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3)
    }

    #[test]
    fn test_example2_p1() {
        assert_eq!(part1("{{{},{},{{}}}}"), 16)
    }

    #[test]
    fn test_example7_p2() {
        assert_eq!(part2(r#"<{o"i!a,<{i<a>"#), 10)
    }
}

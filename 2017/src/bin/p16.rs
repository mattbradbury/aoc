use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time, error::Error, str::FromStr};

fn main() {
    let input = load_input(2017, 16);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input,"abcdefghijklmnop")));
    println!("Part2: {}", bench(|| part2(&input,"abcdefghijklmnop")));
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


#[derive(Debug, Clone, Copy)]
enum Instr {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Debug)]
struct StupidError {}

impl Display for StupidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Stupid Error")
    }
}

impl Error for StupidError {
    
}

impl FromStr for Instr {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let op = chars.next().unwrap();
        let payload:String = chars.collect();
        let res = match op {
            's' => { Self::Spin(payload.parse()?)},
            'x' => {
                let mut parts = payload.split('/');
                let a = parts.next().ok_or("Bad")?.parse()?;
                let b = parts.next().ok_or("Bad")?.parse()?;
                Self::Exchange(a, b)       
            },
            'p' => {
                let chars = payload.chars().collect_vec();
                Self::Partner(chars[0], chars[2])
            },
            _ => { return Err(Box::new(StupidError{}))}
        };
        Ok(res)
    }
}
 


#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Instr> {
    input.trim().split(",").map(|inst| inst.parse().unwrap()).collect_vec()
}

fn part1(input: &str, dancers: &str) -> String {
    let mut dancers = dancers.chars().collect_vec();
    let instrs = parse_input(input);
    for instr in instrs {
        match instr {
            Instr::Spin(k) => dancers.rotate_right(k),
            Instr::Exchange(a, b) => dancers.swap(a, b),
            Instr::Partner(a, b) => {
                let a = dancers.iter().position(|d| d == &a).unwrap();
                let b = dancers.iter().position(|d| d == &b).unwrap();
                dancers.swap(a, b);
            },
        }
    }
    dancers.iter().collect()
}

fn part2(input: &str, dancers: &str) -> String  {
    let instrs = parse_input(input);
    let mut dancers = dancers.chars().collect_vec();
    let mut history = Vec::new();
    let mut counter = 0;
    while counter < 1_000_000_000 {
        
        for instr in &instrs {
            match instr {
                Instr::Spin(k) => dancers.rotate_right(*k),
                Instr::Exchange(a, b) => dancers.swap(*a, *b),
                Instr::Partner(a, b) => {
                    let a = dancers.iter().position(|d| d == a).unwrap();
                    let b = dancers.iter().position(|d| d == b).unwrap();
                    dancers.swap(a, b);
                },
            }
        }
        if counter % 100 == 0 { println!("{counter}")};

        if history.contains(&dancers) { 
            let first = history.iter().position(|k| k == &dancers).unwrap();
            let second = counter;
            dbg!(first, second);
            let loops = 1_000_000_000 / second;
            counter = second * loops + 1;
            history.clear()
             
        }
        else {
            counter += 1;
            history.push(dancers.clone());
            
        }
    
    }
    dancers.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"s1,x3/4,pe/b
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE,"abcde").as_str(), "baedc")
    }

    // #[test]
    // fn test_example1_p2() {
    //     assert_eq!(part2(EXAMPLE), 0)
    // }
}

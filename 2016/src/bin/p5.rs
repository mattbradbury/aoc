use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 5);
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
fn parse_input(_input: &str) {}

fn part1(input: &str) -> String {
    let mut ret = String::new();
    let mut count = 0;
    let input = input.trim();
    loop {
        let sum = md5::compute(format!("{}{}", input, count).as_str());
        let sum = format!("{:x}", sum);
        if sum.starts_with("00000") {
            println!("Found {}", sum);
            ret.push(sum.chars().nth(5).unwrap());
        };
        if ret.len() == 8 {
            break;
        }
        count += 1;
    }
    ret
}

fn part2(input: &str) -> String {
    let mut ret = vec![None; 8];
    let mut count = 0;
    let input = input.trim();
    loop {
        let sum = md5::compute(format!("{}{}", input, count).as_str());
        let sum = format!("{:x}", sum);
        if sum.starts_with("00000") {
            println!("Found {}", sum);
            let mut sum = sum.chars();
            match sum.nth(5).unwrap().to_digit(10) {
                Some(pos) if pos < 8 => {
                    let val = sum.next().unwrap();
                    if ret[pos as usize] == None {
                        ret[pos as usize] = Some(val)
                    }
                }
                _ => {}
            };
        };
        if !ret.contains(&None) {
            break;
        }
        count += 1;
    }
    ret.into_iter().map(|c| c.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1("abc"), "18f47a30")
    }

    #[test]
    fn test_input_p1() {
        assert_eq!(part1("ojvtpuvg"), "4543c154")
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2("ojvtpuvg"), "05ace8e3")
    }
}

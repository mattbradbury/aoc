use aoc_helper::load_input;
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 10);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input, 256)));
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
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect_vec()
}

fn parse_input2(input: &str) -> Vec<u32> {

    let mut res = input.trim()
        .bytes()
        .map(|n| n as u32)
        .collect_vec();
    let mut suffix = vec![17,31,73,47,23];
    res.append(&mut suffix);
    
    res
}

fn part1(input: &str, size: isize) -> isize {
    let mut list = (0..size).collect_vec();
    let mut lengths = parse_input(input);
    let mut pos = 0;
    let mut skip = 0;

    for mut length in lengths {
        for i in 0..(length / 2 ) {
            let low = (pos + i + size) % size;
            let high = (pos + length - i + size - 1) % size;
            list.swap(low as usize, high as usize);
        }
        pos = (pos + skip + length) % size;
        skip += 1;
        dbg!( pos, skip, &list);
    }

    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let size = 256;
    let mut list = (0..size).collect_vec();
    let lengths = parse_input2(input);
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for length in &lengths {
            for i in 0..(length / 2 ) {
                let low = (pos + i + size) % size;
                let high = (pos + length - i + size - 1) % size;
                list.swap(low as usize, high as usize);
            }
            pos = (pos + skip + length) % size;
            skip = (skip + 1) % size;
            // dbg!( pos, skip, &list);
        }
    
    }

    let mut hex = String::new();
    for i in 0..16 {
        let mut hash = 0;
        for j in 0..16 {
            hash ^= list[i*16 + j]; 
        }
        hex += format!("{:0>2x}",hash).as_str();
    }
    hex
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1("3,4,1,5", 5), 12)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2("").as_str(), "a2582a3a0e66e6e86e3812dcb672a272")
    }

    #[test]
    fn test_example2_p2() {
        assert_eq!(part2("AoC 2017").as_str(), "33efeb34ea91902bb2f59c9920caa6cd")
    }
}

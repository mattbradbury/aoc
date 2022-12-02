#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 9);
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
fn parse_input(input: &str) -> String {
    input.split_ascii_whitespace().collect()
}

#[derive(Debug)]
enum ChunkType {
    Plain(usize),
    Repeat(usize, Vec<ChunkType>),
}

impl ChunkType {
    fn len(&self) -> usize {
        match self {
            Plain(n) => *n,
            Repeat(count, chunks) => chunks.iter().map(|c| c.len()).sum::<usize>() * count,
        }
    }
}

use ChunkType::*;

fn parse_input2(input: &str) -> Vec<ChunkType> {
    let mut input_string = input.to_owned();
    let mut output = Vec::new();
    let mut input = input_string.as_str();
    loop {
        let (first, second) = match input.split_once('(') {
            Some(parts) => parts,
            None => {
                output.push(Plain(input.len()));
                break;
            }
        };
        output.push(Plain(first.len()));
        let (len, count, rest) = scan_fmt!(second, "{d}x{d}){}", usize, usize, String).unwrap();
        let chunk = &rest[0..len];
        output.push(Repeat(count, parse_input2(chunk)));
        // for _ in 0..count { output += chunk };
        input_string = rest;
        input = &input_string[len..];
        if input.len() == 0 {
            break;
        }
    }
    output
}

fn decompress(input: String) -> String {
    let mut input_string = input;
    let mut output = String::new();
    let mut input = input_string.as_str();
    loop {
        let (first, second) = match input.split_once('(') {
            Some(parts) => parts,
            None => {
                output += input;
                break;
            }
        };
        output += first;
        let (len, count, rest) = scan_fmt!(second, "{d}x{d}){}", usize, usize, String).unwrap();
        let chunk = &rest[0..len];
        for _ in 0..count {
            output += chunk
        }
        input_string = rest;
        input = &input_string[len..];
        if input.len() == 0 {
            break;
        }
    }
    output
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let output = decompress(input);
    output.len()
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let chunks = Repeat(1, parse_input2(&input));
    // println!("{:#?}", chunks);
    chunks.len()
}

#[cfg(test)]
mod tests {
    use crate::{decompress, part1, part2};

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1_decompress1() {
        assert_eq!(decompress("ADVENT".to_owned()), "ADVENT");
        assert_eq!(part1("ADVENT"), 6)
    }

    #[test]
    fn test_example1_decompress2() {
        assert_eq!(decompress("A(1x5)BC".to_owned()), "ABBBBBC")
    }

    #[test]
    fn test_example1_decompress3() {
        assert_eq!(decompress("(3x3)XYZ".to_owned()), "XYZXYZXYZ")
    }

    #[test]
    fn test_example1_decompress4() {
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG".to_owned()), "ABCBCDEFEFG")
    }

    #[test]
    fn test_example1_decompress5() {
        assert_eq!(decompress("(6x1)(1x3)A".to_owned()), "(1x3)A")
    }

    #[test]
    fn test_example1_decompress6() {
        assert_eq!(
            decompress("X(8x2)(3x3)ABCY".to_owned()),
            "X(3x3)ABC(3x3)ABCY"
        );
        assert_eq!(part1("X(8x2)(3x3)ABCY"), 18)
    }

    #[test]
    fn test_example2_decompress1() {
        // assert_eq!(decompress("ADVENT".to_owned()), "ADVENT");
        assert_eq!(part2("ADVENT"), 6)
    }

    #[test]
    fn test_example2_decompress2() {
        assert_eq!(part2("A(1x5)BC"), "ABBBBBC".len())
    }

    #[test]
    fn test_example2_decompress3() {
        assert_eq!(part2("(3x3)XYZ"), "XYZXYZXYZ".len())
    }

    #[test]
    fn test_example2_decompress4() {
        assert_eq!(
            part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        )
    }

    #[test]
    fn test_example2_decompress5() {
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920)
    }

    #[test]
    fn test_example2_decompress6() {
        assert_eq!(part2("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY".len());
        assert_eq!(part1("X(8x2)(3x3)ABCY"), 18)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

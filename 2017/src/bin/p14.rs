use aoc_helper::load_input;
use bitvec::{prelude::BitVec, order::Msb0};
use itertools::Itertools;
use std::{fmt::Display, time, collections::{HashSet, VecDeque}};

fn main() {
    let input = load_input(2017, 14);
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

fn parse_input2(input: &str) -> Vec<u32> {

    let mut res = input.trim()
        .bytes()
        .map(|n| n as u32)
        .collect_vec();
    let mut suffix = vec![17,31,73,47,23];
    res.append(&mut suffix);
    
    res
}


fn hash_knot(input: &str) -> String {
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
        let mut hash:u8 = 0;
        for j in 0..16 {
            hash ^= list[i*16 + j] as u8; 
        }
        hex += format!("{:0>2x}",hash).as_str();
    }
    hex
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                      .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}


fn part1(input: &str) -> u32 {
    (0..=127).map(|i| {
        let input = input.trim();
        let h = format!("{input}-{i}");
        let hk = hash_knot(&h);
        if i == 0 { println!("{h}  ---  {hk}") }
        hk.chars().map(|c| {
            c.to_digit(16).unwrap().count_ones()
        }).sum::<u32>()
        // let bits = usize::from_str_radix(&hk, 16).unwrap();
        // bits.count_ones()
    }).sum() 
}

fn part2(input: &str) -> u32 {

    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    (0..=127).for_each(|i| {
        let input = input.trim();
        let h = format!("{input}-{i}");
        let hk = hash_knot(&h);
        let bytes = hex_to_bytes(&hk).unwrap();
        let bits = BitVec::<_,Msb0>::from_vec(bytes);
        grid.extend(bits.into_iter().enumerate().filter(|v| v.1).map(|v| {
            (v.0 as i32, i as i32)
        }));
    });
    let mut regions = 0;
    let mut queue = VecDeque::new();
    loop {
        let start = { if let Some(k) = grid.iter().next() {*k} else { break }};
        grid.remove(&start);
        regions += 1;
        queue.push_back(start);

        while let Some(k) = queue.pop_front() {
            queue.extend(grid.take(&(k.0 + 1, k.1 + 0 )));
            queue.extend(grid.take(&(k.0 - 1, k.1 + 0 )));
            queue.extend(grid.take(&(k.0 + 0, k.1 + 1 )));
            queue.extend(grid.take(&(k.0 + 0, k.1 - 1 )));

        }
    }
    regions
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1("flqrgnkx"), 8108)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2("flqrgnkx"), 1242)
    }
}

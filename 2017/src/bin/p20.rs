use aoc_helper::{load_input, point::Point3};
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2017, 20);
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

#[derive(Debug, Clone, Copy)]
struct Particle {
    id: usize,
    pos: Point3<isize>,
    vel: Point3<isize>,
    acc: Point3<isize>,
}

impl Particle {
    fn distance(&self) -> usize {
        (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()) as usize
    }

    fn tick(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Particle> {
    input.lines().enumerate().map(|(id, l)| {
        let mut parts = l.trim().split(", ").map(|part| {
            // dbg!(&part);
            let s = part.trim().split("<").nth(1).unwrap().trim_end_matches(">");
            // dbg!(&s);
            s.parse::<Point3<isize>>().unwrap()
        });
        let pos = parts.next().unwrap();
        let vel = parts.next().unwrap();
        let acc = parts.next().unwrap();
        Particle {
            id,
            acc,
            vel,
            pos,
        }
    }).collect_vec()
}

fn part1(input: &str) -> usize {
    let mut parts = parse_input(input);
    // dbg!(parts);
    let mut acc = vec![0; parts.len()];
    let mut dis = parts.iter().map(|p| p.distance()).collect_vec();
    let mut counter = 0;
    loop {
        for part in &mut parts {
            part.tick()
        }

        let dis2 = parts.iter().map(|p| p.distance()).collect_vec();
        acc = dis.iter().zip(dis2.iter()).map(|(a,b)| *b as isize - *a as isize).collect_vec();
        let near = dis2.iter().position_min().unwrap();
        let slow = acc.iter().position_min().unwrap();
        // dbg!(counter, parts[near], parts[slow]);

        let positive = acc.iter().find(|a| **a < 0).is_none();
        // dbg!(positive);
        if counter == 100000 { break near }
        // if near == slow && positive { break near }

        dis = dis2;
        counter += 1;
    }
    
}

fn part2(input: &str) -> usize {
    let mut parts = parse_input(input);
    // dbg!(parts);
    // let mut acc = vec![0; parts.len()];
    // let mut dis = parts.iter().map(|p| p.distance()).collect_vec();
    let mut counter = 0;
    loop {
        for part in &mut parts {
            part.tick()
        }

        let mut i = 0;
        while i < parts.len() - 1 {
            let mut j = i + 1;
            let mut dirty = false;
            while j < parts.len() {
                if parts[i].pos == parts[j].pos {
                    parts.remove(j);
                    dirty = true;
                } else {
                    j += 1;
                }
            }
            if dirty { parts.remove(i); }
            else { i += 1 };
        }
        // let dis2 = parts.iter().map(|p| p.distance()).collect_vec();
        // acc = dis.iter().zip(dis2.iter()).map(|(a,b)| *b as isize - *a as isize).collect_vec();
        // let near = dis2.iter().position_min().unwrap();
        // let slow = acc.iter().position_min().unwrap();
        // // dbg!(counter, parts[near], parts[slow]);

        // let positive = acc.iter().find(|a| **a < 0).is_none();
        // dbg!(positive);
        if counter % 1 == 0 { dbg!(parts.len(), counter); }
        if counter == 100 { break parts.len() }
        // if near == slow && positive { break near }

        // dis = dis2;
        counter += 1;
    }
    
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

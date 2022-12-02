use std::{collections::HashMap, time};

use aoc_helper::load_input;

// type WaterMap = RefCell<HashMap<(usize,usize), MapEnum>>;
type WaterMap = HashMap<(usize, usize), MapEnum>;

fn main() {
    let input = load_input(2018, 17);
    let input = parse_input(&input);
    bench(|| part1(input));
    // bench(|| part2(&input));
}

fn bench<F>(f: F)
where
    F: FnOnce(),
{
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum MapEnum {
    // Empty,
    Clay,
    Still,
    Flowing,
    // Falling,
}
impl Default for MapEnum {
    fn default() -> Self {
        Self::Flowing
    }
}

use MapEnum::*;

#[allow(dead_code)]
fn parse_input(input: &str) -> WaterMap {
    let mut ground = HashMap::new();
    input.trim().lines().for_each(|l| {
        let (p1, p2) = l.split_once(", ").unwrap();
        let (axis, val) = p1.split_once('=').unwrap();
        let val: usize = val.parse().unwrap();
        let (_, range) = p2.split_once('=').unwrap();

        let mut r = range.split("..").map(|v| v.parse().unwrap());
        let (min, max) = (r.next().unwrap(), r.next().unwrap());
        for i in min..=max {
            let xy = if axis == "x" { (val, i) } else { (i, val) };
            ground.insert(xy, Clay);
        }
    });
    // RefCell::new(ground)
    ground
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    LookSide(bool), // left is bad (false), right is good(true)
    LookDown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ReturnType {
    // Wall(usize, usize),
    Wall,
    // Fall(usize, usize),
    // Falling,
    // Filled,
    Abyss,
}

use Mode::*;
use ReturnType::*;

fn part1(water_map: WaterMap) {
    // let mut falling = VecDeque::new();
    // falling.push_back(start);
    // println!("{:?}", flowing);

    // sim(&water_map, LookDown, start.0, start.1, max_y);

    let res = flow(water_map);
    println!("Part1: {}", res);
}

fn flow(mut water_map: WaterMap) -> usize {
    let (_, min_y) = *water_map.keys().min_by_key(|(_, y)| y).unwrap();
    let (_, max_y) = *water_map.keys().max_by_key(|(_, y)| y).unwrap();
    let start = (500, min_y - 1);
    let mut falls = Vec::new();
    falls.push(start);
    while let Some(fall) = falls.last() {
        let fall = *fall;
        println!("Fall: {:?}", fall);
        // let is_abyss = loop {
        //     let down = *water_map.entry(fall).or_default();
        //     if down == Clay || down == Still {
        //         break false;
        //     };
        //     if fall.1 > max_y { break true; };
        // };

        if is_abyss(&mut water_map, LookDown, fall, max_y) == Abyss {
            falls.pop().unwrap();
            continue;
        };
        let (mut x, y) = fall;

        let left_wall = loop {
            x -= 1;
            let down = *water_map.entry((x, y + 1)).or_default();
            if down == Flowing {
                falls.push((x, y));
                break None;
            };
            let left = *water_map.entry((x - 1, y + 1)).or_default();
            if left == Clay {
                break Some((x - 1, y));
            };
        };

        let (mut x, y) = fall;

        let right_wall = loop {
            x -= 1;
            let down = *water_map.entry((x, y + 1)).or_default();
            if down == Flowing {
                falls.push((x, y));
                break None;
            };
            let right = *water_map.entry((x + 1, y + 1)).or_default();
            if right == Clay {
                break Some((x + 1, y));
            };
        };

        match (right_wall, left_wall) {
            (Some((lx, _)), Some((rx, _))) => {
                for x in lx + 1..rx {
                    water_map.insert((x, y), Still);
                }
                // falls.push(fall);
            }
            _ => {}
        }
    }
    // water_map.

    water_map
        .values()
        .filter(|v| **v == Flowing || **v == Still)
        .count()
}

fn is_abyss(
    water_map: &mut WaterMap,
    mode: Mode,
    start: (usize, usize),
    max_y: usize,
) -> ReturnType {
    let (mut x, mut y) = start;
    println!("Is_abyss (x,y) {},{}", x, y);
    match mode {
        LookSide(look_right) => loop {
            let next_x = if look_right { x + 1 } else { x - 1 };
            let down = *water_map.entry((next_x, y + 1)).or_default();
            if down == Flowing {
                return is_abyss(water_map, LookDown, (next_x, y + 1), max_y);
            };
            let side = *water_map.entry((next_x, y)).or_default();
            if side == Clay {
                return Wall;
            }
            x = next_x;
        },
        LookDown => loop {
            let down = *water_map.entry((x, y + 1)).or_default();
            if down == Clay || down == Still {
                let left = is_abyss(water_map, LookSide(false), (x - 1, y), max_y);
                let right = is_abyss(water_map, LookSide(true), (x + 1, y), max_y);
                if left == Abyss && right == Abyss {
                    return Abyss;
                }
            };
            if y > max_y {
                return Abyss;
            };
            y += 1;
        },
    };
}

fn _part2(_input: &str) {}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1};

    const EXAMPLE: &str = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;

    #[test]
    fn test_example1() {
        let input = parse_input(EXAMPLE);
        part1(input);
        panic!()
    }
}

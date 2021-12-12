use std::{time, collections::{HashMap}, cell::RefCell};

use aoc_helper::load_input;

type WaterMap = RefCell<HashMap<(usize,usize), MapEnum>>;
// type WaterMap = HashMap<(usize,usize), MapEnum>;

fn main() {
    let input = load_input(2018, 17);
    let input = parse_input(&input);
    bench(|| part1(input));
    // bench(|| part2(&input));
}

fn bench<F>(f: F) where F: FnOnce() {
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

use MapEnum::*;

#[allow(dead_code)]
fn parse_input(input: &str) -> WaterMap {
    let mut ground = HashMap::new();
    input.trim()
        .lines()
        .for_each(|l| {
            let (p1, p2) = l.split_once(", ").unwrap();
            let (axis, val) = p1.split_once('=').unwrap();
            let val:usize = val.parse().unwrap();
            let (_, range) = p2.split_once('=').unwrap();
            
            let mut r = range.split("..")
                .map(|v| v.parse().unwrap());
            let (min, max) = (r.next().unwrap(), r.next().unwrap());
            for i in min..=max {
                let xy = if axis == "x" { (val,i) } else { (i, val) };
                ground.insert(xy, Clay);
            }
        });
    RefCell::new(ground)
    // ground
}

#[derive(Debug)]
enum Mode {
    LookSide(bool),  // left is bad (false), right is good(true)
    LookDown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ReturnType {
    Wall(usize, usize),
    // Fall(usize, usize),
    // Falling,
    Filled,
    Abyss
}

use Mode::*;
use ReturnType::*;

fn part1(water_map: WaterMap) {
    let (_,min_y) = *water_map.borrow_mut().keys().min_by_key(|(_,y)| y).unwrap();
    let (_,max_y) = *water_map.borrow_mut().keys().max_by_key(|(_,y)| y).unwrap();
    let start = (500,min_y - 1);
    // let mut falling = VecDeque::new();
    // falling.push_back(start);
    // println!("{:?}", flowing);

    sim(&water_map, LookDown, start.0, start.1, max_y);

    let res = water_map.borrow()
        .values()
        .filter(|v| **v == Flowing || **v == Still )
        .count();
    println!("Part1: {}", res);
}

fn sim(water_map: &WaterMap, mode: Mode, x: usize, y:usize, max_y:usize) -> ReturnType {
    println!("x,y = {},{} {:?}", x, y, mode);
    match mode {
        LookSide(right) => {
            let look_x = if right { x + 1 } else { x - 1 };
            // check for waterfall
            let down =  *water_map.borrow_mut().entry((x,y+1)).or_insert(Flowing);
            if down == Flowing { 
                let down_sim = sim(water_map, LookDown ,x, y+1, max_y);
                if down_sim != Filled { return down_sim }
            }
            let side =  *water_map.borrow_mut().entry((look_x,y)).or_insert(Flowing);
            if side == Clay { return Wall(look_x, y) };
            return sim(water_map, LookSide(right) ,look_x, y, max_y)
        },
        LookDown => {
            if y+1 > max_y { return Abyss }
            // loop {
                // let water_map = water_map.clone();
                let down = water_map.borrow_mut().entry((x,y+1)).or_insert(Flowing).clone();
                if down == Still || down == Clay {
                    let left = sim(water_map, LookSide(false), x, y, max_y);
                    let right = sim(water_map, LookSide(true), x, y, max_y);
                    match (left, right) {
                        (Wall(lx,ly), Wall(rx, _ry)) => {
                            for i in lx+1..rx {
                                water_map.borrow_mut().insert((i, ly), Still);
                            }
                            // y -= 1;
                            return Filled;
                        }
                        (_, Abyss)|(Abyss, _) => { return Abyss }
                        (Filled, Filled) | 
                        (Filled, Wall(_,_)) |
                        (Wall(_,_), Filled) => { 
                            sim(water_map, LookDown, x, y, max_y); 
                            // y -= 2;
                        }
                        // (Filled,_)|(_,Filled) => { return Filled }

                        // _ => { 
                        //     println!("{:?} {:?}", left, right);
                        //     panic!()
                        // }
                    }
                }
                else {
                    let down = sim(water_map, LookDown, x, y+1, max_y);
                    // return down;
                    if down == Filled {
                        let look_down = *water_map.borrow().get(&(x,y+1)).unwrap();
                        println!("Look_down: {:?}", look_down);
                        // y -= 1;
                        if look_down == Still {
                            return sim(water_map, LookDown, x, y - 2, max_y); 
                        } else {
                            return down
                        }
                        
                    }
                    else { return down };
                    // if down == Abyss {
                    //     return Abyss;
                    // };
                    
                }
    
            // } //loop
        },
    }
    Abyss
}



fn _part2(_input: &str) {
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1};

    const EXAMPLE: &str = 
r#"x=495, y=2..7
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

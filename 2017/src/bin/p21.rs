use aoc_helper::load_input;
use itertools::Itertools;
use simple_grid::Grid;
use std::{fmt::Display, time, collections::HashMap};

fn main() {
    let input = load_input(2017, 21);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input, 5)));
    println!("Part2: {}", bench(|| part1(&input, 18)));
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
fn parse_input(input: &str) -> HashMap<Grid<char>, Grid<char>> {
    input.trim().lines().map(|line| {
        // let mut parts = 
        let (a,b) = line.split(" => ").map(|part| {
            let part = part.chars().filter(|c| *c != '/').collect_vec();

            let size = (part.len() as f32).sqrt() as usize;
            let g = Grid::new(size, size, part);
            g

        }).collect_tuple().unwrap();
        (a, b)
        
        
    }).collect()
}

fn part1(input: &str, iterations: usize) -> usize {
    let mut grid = Grid::new(3, 3, ".#...####".chars().collect());

    let rules = parse_input(input);
    for i in 0..iterations {
        if grid.width() % 2 == 0 {
            let newsize = grid.width() * 3 / 2;
            let mut newgrid = Grid::<char>::new_default(newsize, newsize);
            for j in 0..grid.height()/2 {
                for i in 0..grid.width()/2 {
                    let mut subgrid = grid.clone().subgrid(i*2, j*2, 2, 2);
                    let mut k = 0;
                    let mut foundgrid =
                        loop {
                            match rules.get(&subgrid) {
                                Some(g) => { break g},
                                None => {},
                            }
                            k += 1;
                            subgrid.rotate_cw();
                            if k == 4 { subgrid.flip_horizontally() };
                            if k == 8 { panic!() };
                        };
                    for l in 0..3 {
                        for m in 0..3 {
                            newgrid.replace_cell((i * 3 + l, j*3 + m), *foundgrid.get((l,m)).unwrap());
                        }
                    }
                }
            }
            grid = newgrid;


        } else {
            let newsize = grid.width() * 4 / 3;
            let mut newgrid = Grid::<char>::new_default(newsize, newsize);
            for j in 0..grid.height()/3 {
                for i in 0..grid.width()/3 {
                    let mut subgrid = grid.clone().subgrid(i*3, j*3, 3, 3);
                    let mut k = 0;
                    let mut foundgrid =
                        loop {
                            match rules.get(&subgrid) {
                                Some(g) => { break g},
                                None => {},
                            }
                            k += 1;
                            subgrid.rotate_cw();
                            if k == 4 { subgrid.flip_horizontally() };
                            if k == 8 { panic!() };
                        };
                    for l in 0..4 {
                        for m in 0..4 {
                            newgrid.replace_cell((i * 4 + l, j*4 + m), *foundgrid.get((l,m)).unwrap());
                        }
                    }
                }
            }
            grid = newgrid;


        };
        println!("{}/n", grid.to_pretty_string());

    }
    // dbg!(rules);
    grid.cell_iter().filter(|c| **c == '#').count()
    // 0
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE, 2), 12)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

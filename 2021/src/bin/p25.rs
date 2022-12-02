use aoc_helper::{load_input, Grid};
use itertools::Itertools;
use std::{fmt::Display, time};

fn main() {
    let input = load_input(2021, 25);
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
fn parse_input(input: &str) -> Grid<char> {
    let grid = input.trim()
        .lines()
        .map(|line| { line.trim().chars().collect_vec() })
        .collect_vec();
    Grid::new(grid)
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut oldest_grid = grid.clone();
    let (max_x, max_y) = grid.dimensions_signed();
    
    let  mut count = 0;
    println!("{}", grid);
    println!();
    loop {
        let mut dirty = false;
        let old_grid = grid.clone();
        for y in (0..max_y).rev() {
            for x in (0..max_x).rev() {
                if *old_grid.wrapping_get(x, y) == '>' {
                    if *old_grid.wrapping_get(x + 1, y) == '.' {
                        grid.wrapping_set(x, y, '.');
                        grid.wrapping_set(x + 1, y, '>');
                        dirty = true;
                    }
                }
            }
        }

        let old_grid = grid.clone();

        for y in (0..max_y).rev() {
            for x in (0..max_x).rev() {
                if *old_grid.wrapping_get(x, y) == 'v' {
                    if *old_grid.wrapping_get(x, y + 1) == '.' {
                        grid.wrapping_set(x, y, '.');
                        grid.wrapping_set(x, y + 1, 'v');
                        dirty = true;
                    }
                }
            }
        }
        count += 1;
        println!("After step {}", count);
        println!("{}", grid);
        if !dirty { break; }
        // if count == 59 { break }
    }
    
    count
}

fn part2(_input: &str) -> usize {
    0
}

#[test]
fn test() {
    part1(EXAMPLE2);
}

const EXAMPLE2: &str =
r#"...>...
.......
......>
v.....>
......>
.......
..vvv.."#;

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"#;



    #[test]
    fn test_example1_p1() {
        
        assert_eq!(part1(EXAMPLE), 58)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

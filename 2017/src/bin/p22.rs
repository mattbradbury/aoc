use aoc_helper::{load_input, Turtle, turtle::{Dirs, Facing}};
use std::{fmt::Display, time, collections::HashMap};

fn main() {
    let input = load_input(2017, 22);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input, 10000)));
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

fn parse_input(input: &str) -> (HashMap<(isize, isize), bool>, isize) {

    let mut lines = input.trim().lines().peekable();
    let size = lines.peek().unwrap().len();
    
    let grid = lines.enumerate().flat_map(|(j,line)| {
        line.trim().char_indices().map(move |(i, c)| {
            ((i as isize, -(j as isize)), if c == '#' { true } else { false })
        })
    }).collect();
    (grid, size as isize)
}

fn print_grid(grid: &HashMap<(isize,isize), bool>, radius:isize , turtle: &Turtle) {
    for j in -radius..=radius {
        for i in -radius..=radius {
            let c = if *grid.get(&(i,-j)).unwrap_or(&false) { '#' } else { '.' };
            print!("{c}");
            if turtle.x == i && turtle.y == -j { print!("*") } else {print!(" ")};
        }
        println!()
    }
}

fn part1(input: &str, iterations: usize) -> usize {
    let (mut grid, size) = parse_input(input);
    dbg!(&grid);
    let mut turtle: Turtle = (size/2, -size/2).into();
    turtle.facing = Facing::North;

    let mut infections = 0;

    for i in 0..iterations {
        // println!("{}", turtle);
        // print_grid(&grid, 7, &turtle);
        let val = grid.entry(turtle.into()).or_insert(false);
        if *val { 
            turtle.turn(Dirs::Right) 
        } else { 
            infections += 1;
            turtle.turn(Dirs::Left)
        };
        *val = !*val;
        turtle.advance(1);
        // println!("{}", turtle);
        
    }
    infections
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r##"..#
#..
...
"##;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE, 70), 41);
        assert_eq!(part1(EXAMPLE, 10000), 5587);
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

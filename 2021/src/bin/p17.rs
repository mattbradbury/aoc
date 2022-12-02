use aoc_helper::{load_input, point::Point};
use scan_fmt::scan_fmt;
use std::{cmp::max, fmt::Display, time};

fn main() {
    let input = load_input(2021, 17);
    // let input = parse_input(&input);
    // let input = "target area: x=20..30, y=-10..-5";
    part1(&input);
    bench(|| part1(&input));
    // println!("Part1: {:?}", bench(|| part1(&input)));
    // println!("Part2: {}", bench(|| part2(&input)));
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
fn parse_input(input: &str) -> (Point<isize>, Point<isize>) {
    let (ux, lx, ly, uy) = scan_fmt!(
        input,
        "target area: x={d}..{d}, y={d}..{d}",
        isize,
        isize,
        isize,
        isize
    )
    .unwrap();
    let ul = Point { x: ux, y: uy };
    let lr = Point { x: lx, y: ly };
    (ul, lr)
}

fn sim(
    mut vel: Point<isize>,
    (ul, lr): (Point<isize>, Point<isize>),
) -> Result<isize, (Point<isize>, Point<isize>)> {
    let xr = ul.x..=lr.x;
    let yr = lr.y..=ul.y;
    let mut pos = Point { x: 0, y: 0 };
    let mut maxy = isize::MIN;
    loop {
        pos += vel;
        maxy = max(pos.y, maxy);

        vel.x += match vel.x.cmp(&0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        vel.y -= 1;

        if vel.x == 0 && !xr.contains(&pos.x) {
            return Err((pos, vel));
        };
        if pos.y < lr.y {
            return Err((pos, vel));
        };
        if xr.contains(&pos.x) && yr.contains(&pos.y) {
            return Ok(maxy);
        };
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum State {
    FindXMin,
    FindXMax(isize),
    FindYMax(isize, isize),
}

fn part1(input: &str) -> usize {
    let bounds = parse_input(input);
    println!("Bounds: {} {}", bounds.0, bounds.1);
    let _xr = bounds.0.x..=bounds.1.x;
    let _yr = bounds.0.y..=bounds.1.y;
    let _vel = Point { x: 1, y: 0 };
    let mut maxy = 0;
    let _state = State::FindXMin;
    let mut count = 0;
    let _prev_my = isize::MIN;

    for x in 1..=bounds.1.x {
        for y in bounds.1.y..-bounds.1.y {
            match sim(Point { x, y }, bounds) {
                Ok(y) => {
                    maxy = max(y, maxy);
                    count += 1
                }
                Err(_) => {}
            }
        }
    }
    println!("{} {}", maxy, count);

    0 // maxy
}

#[allow(dead_code)]
fn part2(_input: &str) -> usize {
    0
}

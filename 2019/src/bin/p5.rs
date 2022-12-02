use std::{str::FromStr, time};

use aoc_2019::computer::{Computer, Intcodes};
use aoc_helper::load_input;

fn main() {
    bench(part1);
    bench(part2);
}

fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn part1() {
    let code = Intcodes::from_str(&load_input(2019, 5)).unwrap();
    let mut comp = Computer::load(code).start().unwrap();
    comp = comp.input(1).unwrap();
    let mut result = Vec::new();
    while let Some(out) = comp.output() {
        result.push(out)
    }
    println!("{:#?}", result)
}

fn part2() {
    let code = Intcodes::from_str(&load_input(2019, 5)).unwrap();
    let mut comp = Computer::load(code).start().unwrap();
    comp = comp.input(5).unwrap();
    let mut result = Vec::new();
    while let Some(out) = comp.output() {
        result.push(out)
    }
    println!("{:#?}", result)
}

#[cfg(test)]
mod tests {
    const _EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1() {
        // assert_eq!(x,0);
    }
}

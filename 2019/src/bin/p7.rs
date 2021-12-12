use std::{time, str::FromStr};

use aoc_2019::computer::Intcodes;
use aoc_helper::load_input;

fn main() {
    let code = Intcodes::from_str(&load_input(2019, 5)).unwrap();
    bench(|| part1(code.clone()));
    bench(|| part2(code));
}

fn bench<F>(f: F) where F: FnOnce() {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn part1(_code: Intcodes) {
}

fn part2(_code: Intcodes) {
    let _input = load_input(0, 0);
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

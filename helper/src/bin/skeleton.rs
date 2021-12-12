use std::time;

use aoc_helper::load_input;

fn main() {
    let input = load_input(2021, 0);
    // let input = parse_input(&input);
    bench(|| part1(&input));
    bench(|| part2(&input));
}

fn bench<F>(f: F) where F: FnOnce() {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

#[allow(dead_code)]
fn parse_input(_input: &str) {

}

fn part1(_input: &str) {
}

fn part2(_input: &str) {
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

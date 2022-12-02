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
    let input = load_input(2019, 2);
    let mut code = Intcodes::from_str(&input).expect("Parse error");
    code[1] = 12;
    code[2] = 2;
    let res = Computer::load(code).start().unwrap().dump();

    println!("Part1: {}", res[0])
}

fn part2() {
    let input = load_input(2019, 2);
    let code = Intcodes::from_str(&input).expect("Parse error");
    'outer: for i in 0..100 {
        for j in 0..100 {
            let mut code = code.clone();
            code[1] = i;
            code[2] = j;
            if Computer::load(code).start().unwrap().dump()[0] == 19690720 {
                println!("Part2: {}", 100 * i + j);
                break 'outer;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use aoc_2019::computer::{Computer, Intcodes};

    const EXAMPLE1: &str = "1,0,0,0,99";
    const EXAMPLE2: &str = "2,3,0,3,99";
    const EXAMPLE3: &str = "2,4,4,5,99,0";
    const EXAMPLE4: &str = "1,1,1,4,99,5,6,0,99";

    #[test]
    fn test_example1() {
        let code = Intcodes::from_str(EXAMPLE1).unwrap();
        let code_after = Computer::load(code).start().unwrap().dump();
        assert_eq!(code_after[0], 2);
    }

    #[test]
    fn test_example2() {
        let code = Intcodes::from_str(EXAMPLE2).unwrap();
        let code_after = Computer::load(code).start().unwrap().dump();
        assert_eq!(code_after[3], 6);
    }

    #[test]
    fn test_example3() {
        let code = Intcodes::from_str(EXAMPLE3).unwrap();
        let code_after = Computer::load(code).start().unwrap().dump();
        assert_eq!(code_after[5], 9801);
    }

    #[test]
    fn test_example4() {
        let code = Intcodes::from_str(EXAMPLE4).unwrap();
        let code_after = Computer::load(code).start().unwrap().dump();
        assert_eq!(code_after[0], 30);
        assert_eq!(code_after[4], 2);
    }
}

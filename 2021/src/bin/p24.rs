use aoc_helper::load_input;
use itertools::Itertools;
use scan_fmt::parse;
use std::{collections::HashSet, error::Error, fmt::Display, panic, str::FromStr, time};

fn main() {
    let input = load_input(2021, 24);
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

#[derive(Debug, Clone, Copy)]
enum Argument {
    Variable(usize),
    Value(isize),
}

impl FromStr for Argument {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let output = match s {
            "w" => Self::Variable(0),
            "x" => Self::Variable(1),
            "y" => Self::Variable(2),
            "z" => Self::Variable(3),
            _ => {
                let val = s.parse()?;
                Self::Value(val)
            }
        };
        Ok(output)
    }
}

use Argument::*;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Input,
    Addition(usize, Argument),
    Multiply(usize, Argument),
    Divide(usize, Argument),
    Modulo(usize, Argument),
    Equal(usize, Argument),
}

impl FromStr for Operand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.trim().split_ascii_whitespace();
        let arg0 = chunks.next().unwrap();
        let arg1 = chunks.next().unwrap();
        let arg1: Argument = arg1.parse()?;
        let arg1 = match arg1 {
            Variable(c) => c,
            Value(v) => panic!(),
        };
        let arg2 = match chunks.next() {
            Some(arg2) => Some(arg2.parse()?),
            None => None,
        };

        let result = match arg0 {
            "inp" => Self::Input,
            "add" => Self::Addition(arg1, arg2.unwrap()),
            "mul" => Self::Multiply(arg1, arg2.unwrap()),
            "div" => Self::Divide(arg1, arg2.unwrap()),
            "mod" => Self::Modulo(arg1, arg2.unwrap()),
            "eql" => Self::Equal(arg1, arg2.unwrap()),
            _ => panic!(),
        };
        Ok(result)
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Operand> {
    input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect_vec()
}
type State = ([isize; 4], usize);

fn run(
    operands: &Vec<Operand>,
    visited: &mut HashSet<State>,
    vars: &mut [isize; 4],
    start_ptr: usize,
) -> Option<Vec<isize>> {
    let get_arg = |arg: Argument, vars: &[isize; 4]| -> isize {
        match arg {
            Variable(a) => vars[a],
            Value(v) => v,
        }
    };

    for ptr in start_ptr..operands.len() {
        match operands[ptr] {
            Operand::Input => {
                if visited.len() % 1000 == 0 {
                    println!("{}", visited.len())
                }
                for i in (1..10).rev() {
                    if ptr == 18 {
                        println!("{}", i)
                    };
                    let mut new_vars = *vars;
                    new_vars[0] = i;
                    if visited.contains(&(new_vars, ptr)) {
                        return None;
                    }
                    let result = run(operands, visited, &mut new_vars, ptr + 1);
                    match result {
                        Some(mut v) => {
                            v.insert(0, i);
                            return Some(v);
                        }
                        None => {
                            visited.insert((new_vars, ptr));
                        }
                    };
                }
            }
            Operand::Addition(arg1, arg2) => vars[arg1] += get_arg(arg2, vars),
            Operand::Multiply(arg1, arg2) => vars[arg1] *= get_arg(arg2, vars),
            Operand::Divide(arg1, arg2) => {
                let a = get_arg(arg2, vars);
                if a == 0 {
                    return None;
                };
                vars[arg1] /= a
            }
            Operand::Modulo(arg1, arg2) => {
                let a = get_arg(arg2, vars);
                if a == 0 {
                    return None;
                };
                vars[arg1] %= a
            }
            Operand::Equal(arg1, arg2) => {
                if vars[arg1] == get_arg(arg2, vars) {
                    vars[arg1] = 1
                } else {
                    vars[arg1] = 0
                }
            }
            _ => panic!(),
        }
    }
    if vars[3] == 0 {
        Some(Vec::new())
    } else {
        None
    }
}

#[derive(Debug)]
struct TreeNode {
    left: Box<TR>,
    right: Box<TR>,
    operand: Operand,
}
impl TreeNode {
    fn new(left: TR, right: TR, operand: Operand) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            operand,
        }
    }
}

#[derive(Debug)]
enum TR {
    Input(usize),
    Node(TreeNode),
    Val(isize),
    Empty,
}
use TR::*;

fn build_tree(operands: &Vec<Operand>, arg: usize, index: usize) -> TR {
    // dbg!(arg, index);
    for i in (0..index).rev() {
        match operands[i] {
            Operand::Input if 0 == arg => return Input(i),
            Operand::Addition(a, b) if a == arg => {
                let left = build_tree(operands, a, i);

                let right = match b {
                    Variable(v_arg) => build_tree(operands, v_arg, i),
                    Value(v) => Val(v),
                };
                let ret = match (&left, &right) {
                    (Val(a), Val(b)) => Val(a + b),
                    (_, _) => Node(TreeNode::new(left, right, operands[i])),
                };
                return ret;
            }
            Operand::Multiply(a, b) if a == arg => {
                let left = build_tree(operands, a, i);

                let right = match b {
                    Variable(v_arg) => build_tree(operands, v_arg, i),
                    Value(v) => {
                        if v == 0 {
                            return Val(0);
                        }
                        if v == 1 {
                            return left;
                        }
                        Val(v)
                    }
                };
                if let Val(1) = left {
                    return right;
                }

                let ret = match (&left, &right) {
                    (Val(a), Val(b)) => Val(a * b),
                    (_, _) => Node(TreeNode::new(left, right, operands[i])),
                };
                return ret;
            }
            Operand::Divide(a, b) if a == arg => {
                let left = build_tree(operands, a, i);
                if let Val(0) = left {
                    return Val(0);
                }

                let right = match b {
                    Variable(v_arg) => build_tree(operands, v_arg, i),
                    Value(v) => {
                        if v == 1 {
                            return left;
                        }
                        Val(v)
                    }
                };
                let ret = match (&left, &right) {
                    (Val(a), Val(b)) => Val(a / b),
                    (_, _) => Node(TreeNode::new(left, right, operands[i])),
                };
                return ret;
            }
            Operand::Modulo(a, b) if a == arg => {
                let left = build_tree(operands, a, i);
                if let Val(0) = left {
                    return Val(0);
                }

                let right = match b {
                    Variable(v_arg) => build_tree(operands, v_arg, i),
                    Value(v) => {
                        if v == 1 {
                            return left;
                        }
                        Val(v)
                    }
                };
                let ret = match (&left, &right) {
                    (Val(a), Val(b)) => Val(a % b),
                    (_, _) => Node(TreeNode::new(left, right, operands[i])),
                };
                return ret;
            }

            Operand::Addition(a, b) if a == arg => {
                let left = build_tree(operands, a, i);

                let right = match b {
                    Variable(v_arg) => build_tree(operands, v_arg, i),
                    Value(v) => Val(v),
                };
                let ret = match (&left, &right) {
                    (Val(a), Val(b)) => {
                        if a == b {
                            Val(1)
                        } else {
                            Val(0)
                        }
                    }
                    (_, _) => Node(TreeNode::new(left, right, operands[i])),
                };
                return ret;
            }
            _ => continue,
        }
    }
    return Val(0);
}

fn part1(input: &str) -> usize {
    let operands = parse_input(input);
    // let tree = build_tree(&operands, 3,operands.len());
    // dbg!(tree);
    let mut vars = [0; 4];
    let mut visited = HashSet::<State>::new();
    let result = run(&operands, &mut visited, &mut vars, 0);
    dbg!(&result);
    result.unwrap().into_iter().for_each(|v| print!("{}", v));
    println!();
    0
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 0)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}

use aoc_helper::load_input;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use std::{fmt::Display, time};
use std::hash::Hash;

fn main() {
    let input = load_input(2017, 7);
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
fn parse_input(input: &str) -> HashMap<String, Node> {

    let mut arena = input.trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let id:String = parts.next().unwrap().into();
            let weight = parts.next().unwrap().trim_start_matches('(').trim_end_matches(')').parse().unwrap();
            parts.next();  // throw away the ->
            let children = parts.map(|c| {c.trim_end_matches(',').into() }).collect_vec();
        (id.clone(), Node { id, parent: None, children, weight, stack_weight: 0 })
    }).collect::<HashMap<String, Node>>();
    // let mut arena = Box::new(arena);

    let ids = arena.keys().map(|id| id.clone()).collect_vec();
    for id in ids {
        let node = arena.get(&id).unwrap().clone();
        node.children.iter().for_each(|child| {
            arena.get_mut(&child.clone()).unwrap().parent = Some(node.id.clone());
        })
    };

    arena
}

fn part1(input: &str) -> String {
    let arena = parse_input(input);
    let root = arena.values().find(|n| n.parent.is_none() ).unwrap();
    // let odd = arena.values().find(|n| n.children.len() > 0 && n.children.len() < 3 );
    // dbg!(odd);
    root.id.clone()
}

fn part2(input: &str) -> usize {
    let mut arena = parse_input(input);
    let root = arena.values().find(|n| n.parent.is_none() ).unwrap();
    // let odd = arena.values().find(|n| n.children.len() > 0 && n.children.len() < 3 );
    // dbg!(odd);
    calc_weight(root.id.clone(), &mut arena);
    0  // cheated and just calculated the answer by hand after finding the bad tree
}

fn calc_weight(root: String, arena: &mut HashMap<String, Node>) -> usize {
    let node = arena.get(&root).unwrap().clone();
    let vals = node.children.iter().map(|child| {
        calc_weight(child.clone(), arena)
    }).collect_vec();
    if vals.is_empty() { return node.weight };
    let sum = vals.iter().sum::<usize>() + node.weight;
    // dbg!(&node.id, &vals);

    if !vals.iter().all_equal() {
        dbg!(vals, node.children);
    }
    arena.get_mut(&root).unwrap().stack_weight = sum;
    sum
}

#[derive(Eq, PartialEq, Debug, Default, Clone)]
struct Node {
    id: String,
    parent: Option<String>,
    children: Vec<String>,
    weight: usize,
    stack_weight: usize,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), "tknk" )
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 60)
    }
}

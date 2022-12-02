use aoc_helper::load_input;
use bitvec::prelude::*;
use std::{fmt::Display, time};

#[derive(Debug)]
enum Payload {
    Literal(usize),
    Packets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
    length: usize,
}

fn main() {
    let input = load_input(2021, 16);
    // let input = parse_input(&input);
    // part1("8A004A801A8002F478");

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
fn parse_input(input: &str) -> Packet {
    let input = input.trim();
    let input = hex::decode(input).unwrap();
    let input = BitVec::<Msb0, _>::from_vec(input);
    // println!("{}", input);
    let bits = &input[0..];
    parse_packet(bits)
}

fn parse_packet(input: &BitSlice<Msb0, u8>) -> Packet {
    let version = input[0..3].load_be();
    let type_id = input[3..6].load_be();
    println!("{} {} {}", version, type_id, input);

    let length;
    let payload = match type_id {
        4 => {
            let mut val = BitVec::<Msb0, usize>::new();
            let mut ptr: usize = 6;
            loop {
                val.extend(input[ptr + 1..ptr + 5].iter());

                if !input[ptr] {
                    break;
                };
                ptr += 5;
            }
            length = ptr + 5;
            Payload::Literal(val.load())
        }
        _ => {
            let mut ptr: usize = 6;
            // let val = BitVec::<Msb0,usize>::new();
            let len_type = input[ptr]; // false(0) - bitlength of contained packets,  true(1) - number of packets contained
            ptr += 1;
            match len_type {
                true => {
                    // println!("{}", &input[ptr..ptr+15]);
                    let op_count = input[ptr..ptr + 11].load_be::<u16>() as usize;
                    // let ptr_end = ptr + op_length;
                    // println!("len {}", op_length);
                    ptr += 11;
                    let mut payloads = Vec::new();
                    for _ in 0..op_count {
                        let packet = parse_packet(&input[ptr..]);
                        ptr += packet.length;
                        // println!("{} {}", ptr, op_length);
                        // println!("{:#?}", packet);
                        payloads.push(packet);
                    }
                    length = ptr;
                    Payload::Packets(payloads)
                }
                false => {
                    // println!("{}", &input[ptr..ptr+15]);
                    let op_length = input[ptr..ptr + 15].load_be::<u16>() as usize;
                    ptr += 15;
                    let ptr_end = ptr + op_length;
                    // println!("len {}", op_length);
                    let mut payloads = Vec::new();
                    while ptr < ptr_end {
                        // 11 is the minimum packet size, make sure we don't just have cruft
                        let packet = parse_packet(&input[ptr..]);
                        ptr += packet.length;
                        // println!("{} {}", ptr, op_length);
                        // println!("{:#?}", packet);
                        payloads.push(packet);
                    }
                    length = ptr;
                    Payload::Packets(payloads)
                }
            }
        }
    };
    Packet {
        version,
        type_id,
        payload,
        length,
    }
}

fn sum_versions(packet: &Packet) -> usize {
    let mut sum = packet.version as usize;
    sum += if let Payload::Packets(packets) = &packet.payload {
        packets.iter().map(|p| sum_versions(p)).sum()
    } else {
        0
    };
    sum
}

fn part1(input: &str) -> usize {
    let packet = parse_input(input);
    let sum = sum_versions(&packet);
    println!("{:#?}", packet);
    sum
}

fn compute_packet(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::Literal(val) => *val,
        Payload::Packets(packets) => match packet.type_id {
            0 => packets.iter().map(|p| compute_packet(p)).sum(),
            1 => packets.iter().fold(1, |acc, p| acc * compute_packet(p)),
            2 => packets.iter().map(|p| compute_packet(p)).min().unwrap(),
            3 => packets.iter().map(|p| compute_packet(p)).max().unwrap(),
            5 => {
                if compute_packet(&packets[0]) > compute_packet(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if compute_packet(&packets[0]) < compute_packet(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if compute_packet(&packets[0]) == compute_packet(&packets[1]) {
                    1
                } else {
                    0
                }
            }

            _ => panic!(),
        },
    }
}

fn part2(input: &str) -> usize {
    let packet = parse_input(input);
    compute_packet(&packet)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2("9C0141080250320F1802104A08"), 1)
    }
}

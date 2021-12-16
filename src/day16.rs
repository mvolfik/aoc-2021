use crate::utils::Failable;
use std::ops::{Add, Mul};
use {crate::utils::DayResult, std::str::Lines};

type Num = u128;

pub(crate) fn main(mut stdin: Lines) -> DayResult {
    let l = match stdin.next() {
        None => return Err("No input".to_string()),
        Some(l) => l,
    };
    let mut bits = Vec::with_capacity(l.len() * 4);
    for c in l.chars() {
        let n = c.to_digit(16).ok_or("Expected base16 digit")?;
        for i in 0..4 {
            bits.push(((n >> (3 - i)) % 2) == 1)
        }
    }
    let mut reader = BitsReader { bits, pointer: 0 };
    let root = reader.read_packet();
    Ok((
        Ok(sum_versions(&root).to_string()),
        calc(&root).map(|x| x.to_string()),
    ))
}

fn sum_versions(packet: &Packet) -> Num {
    let mut s = packet.version as Num;
    if let PacketContent::Operator(children) = &packet.content {
        for c in children {
            s += sum_versions(c)
        }
    }
    s
}

fn calc(packet: &Packet) -> Failable<Num> {
    match &packet.content {
        PacketContent::Value(x) => Ok(*x),
        PacketContent::Operator(children) => {
            let (mut val, func): (Num, Box<dyn Fn(Num, Num) -> Num>) = match &packet.type_id {
                0 => (0, Box::new(Num::add)),
                1 => (1, Box::new(Num::mul)),
                2 => (Num::MAX, Box::new(Num::min)),
                3 => (Num::MIN, Box::new(Num::max)),
                x @ (5 | 6 | 7) => {
                    let func: Box<dyn for<'r> Fn(&'r Num, &'r Num) -> bool> = match x {
                        5 => Box::new(Num::gt),
                        6 => Box::new(Num::lt),
                        7 => Box::new(Num::eq),
                        _ => {
                            panic!("Unreachable branch reached")
                        }
                    };
                    let mut iter = children.iter();
                    let (a, b) = match (iter.next(), iter.next()) {
                        (Some(a), Some(b)) => (calc(a)?, calc(b)?),
                        _ => {
                            return Err(format!(
                                "Expected two children for operation packet ver. {}",
                                x
                            ))
                        }
                    };
                    return Ok(func(&a, &b) as Num);
                }
                x => return Err(format!("Unexpected packet version: {}", x)),
            };
            for c in children {
                val = func(val, calc(c)?)
            }
            Ok(val)
        }
    }
}

#[derive(Debug)]
enum PacketContent {
    Value(Num),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}

struct BitsReader {
    bits: Vec<bool>,
    pointer: usize,
}

impl BitsReader {
    fn read_bit(&mut self) -> bool {
        let bit = self.bits[self.pointer];
        self.pointer += 1;
        bit
    }
    fn read_num(&mut self, bitsize: usize) -> Num {
        let mut n = 0;
        for _ in 0..bitsize {
            n = (n << 1) + self.read_bit() as Num;
        }
        n
    }
    fn read_packet(&mut self) -> Packet {
        let version = self.read_num(3) as u8;
        let type_id = self.read_num(3) as u8;
        let content = if type_id == 4 {
            let mut val = 0;
            loop {
                let is_end = !self.read_bit();
                val = (val << 4) + self.read_num(4);
                if is_end {
                    break;
                }
            }
            PacketContent::Value(val)
        } else {
            let (mut val, end_fn): (Num, Box<dyn Fn(&Self, Num) -> (Num, bool)>) =
                match self.read_bit() {
                    false => {
                        let len = self.read_num(15) as usize;
                        let end = len + self.pointer;
                        (
                            0,
                            Box::new(move |this_pack, _| (0, this_pack.pointer >= end)),
                        )
                    }
                    true => {
                        let n = self.read_num(11);
                        (1, Box::new(move |_, x| (x + 1, x >= n)))
                    }
                };
            let mut packets = Vec::new();
            loop {
                packets.push(self.read_packet());
                let res = end_fn(self, val);
                if res.1 {
                    break;
                }
                val = res.0;
            }
            PacketContent::Operator(packets)
        };

        Packet {
            version,
            type_id,
            content,
        }
    }
}

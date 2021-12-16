use {crate::utils::DayResult, std::str::Lines};

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
    // dbg!(&root);
    Ok((
        Ok(sum_versions(&root).to_string()),
        Err("noyet".to_string()),
    ))
}

fn sum_versions(packet: &Packet) -> u32 {
    let mut s = packet.version as u32;
    if let PacketContent::Operator(children) = &packet.content {
        for c in children {
            s += sum_versions(c)
        }
    }
    s
}

#[derive(Debug)]
enum PacketContent {
    Value(u32),
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
    fn read_num(&mut self, bitsize: usize) -> u32 {
        let mut num = 0;
        for _ in 0..bitsize {
            num = (num << 1) + self.read_bit() as u32;
        }
        num
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
            let (mut val, end_fn): (u32, Box<dyn Fn(&Self, u32) -> (u32, bool)>) =
                match self.read_bit() {
                    false => {
                        let end = self.pointer + self.read_num(15) as usize;
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

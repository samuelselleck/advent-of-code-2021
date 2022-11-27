use bitvec::{field::BitField, prelude as bv};
use hex::decode;
use std::fs;

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Literal(u128),
    Expr(Operator, Vec<Packet>),
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Packet {
    fn parse_at(index: usize, bits: &bv::BitVec<u8, bv::Msb0>) -> (usize, Self) {
        let mut i = index;
        let mut take = |n: usize| {
            let taken = &bits[i..(i + n)];
            i += n;
            taken
        };

        let version: u8 = take(3).load_be();
        let type_id: u8 = take(3).load_be();

        if type_id == 4 {
            let mut val: u128 = 0;
            loop {
                let head = take(1)[0];
                val = (val << 4) + take(4).load_be::<u128>();
                if !head {
                    break;
                }
            }
            return (
                i,
                Packet {
                    version,
                    packet_type: PacketType::Literal(val),
                },
            );
        }
        let mut sub_packets = Vec::new();

        let mut parse_subpacket = |n| {
            let (i_new, packet) = Packet::parse_at(n, bits);
            sub_packets.push(packet);
            i_new
        };
        let len_type_id = take(1)[0];
        match len_type_id {
            false => {
                let total_bit_length = take(15).load_be::<usize>();
                let end = i + total_bit_length;
                while i < end {
                    i = parse_subpacket(i);
                }
            }
            true => {
                let contained_packets = take(11).load_be::<usize>();
                for _ in 0..contained_packets {
                    i = parse_subpacket(i);
                }
            }
        }
        (
            i,
            Packet {
                version,
                packet_type: PacketType::Expr(
                    match type_id {
                        0 => Operator::Sum,
                        1 => Operator::Product,
                        2 => Operator::Minimum,
                        3 => Operator::Maximum,
                        5 => Operator::GreaterThan,
                        6 => Operator::LessThan,
                        7 => Operator::EqualTo,
                        _ => panic!("unknown type_id code"),
                    },
                    sub_packets,
                ),
            },
        )
    }

    fn version_sum(&self) -> u128 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as u128,
            PacketType::Expr(_, sub_packets) => sub_packets.iter().map(|p| p.version_sum()).sum(),
        }
    }

    fn value(&self) -> u128 {
        match &self.packet_type {
            PacketType::Literal(v) => *v,
            PacketType::Expr(op, packets) => {
                let mut ints = packets.iter().map(|p| p.value());
                match op {
                    Operator::Sum => ints.sum(),
                    Operator::Product => ints.product(),
                    Operator::Minimum => ints.min().unwrap(),
                    Operator::Maximum => ints.max().unwrap(),
                    Operator::GreaterThan => (ints.next() > ints.next()) as u128,
                    Operator::LessThan => (ints.next() < ints.next()) as u128,
                    Operator::EqualTo => (ints.next() == ints.next()) as u128,
                }
            }
        }
    }
}

impl From<bv::BitVec<u8, bv::Msb0>> for Packet {
    fn from(bits: bv::BitVec<u8, bv::Msb0>) -> Self {
        let (_, packet) = Packet::parse_at(0, &bits);
        packet
    }
}

fn main() {
    let input = fs::read_to_string("bits.txt").expect("file not found");
    let hex_str = decode(input).unwrap();
    let bits: bv::BitVec<_, bv::Msb0> = bv::BitVec::from_vec(hex_str);

    let packet = Packet::from(bits);
    println!("{:?}", packet.version_sum());
    println!("{:?}", packet.value());
}

use bitvec::prelude::*;
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
    Operator(Operator, Vec<Packet>),
}

#[derive(Debug)]
enum Operator {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7
}


impl Packet {
    fn parse_at(index: usize, bits: &BitVec<u8, Msb0>) -> (usize, Self) {
        let mut i = index;
        let version: u8 = bits[i..(i + 3)].load_be();
        i += 3;
        let type_id: u8 = bits[i..(i + 3)].load_be();
        i += 3;
        match type_id {
            4 => {
                let mut val = 0;
                loop {
                    let head = bits[i];
                    i += 1;
                    val = (val << 4) + bits[i..(i + 4)].load_be::<u128>();
                    i += 4;
                    if !head {
                        break;
                    }
                }
                (i, Packet{
                    version,
                    packet_type: PacketType::Literal(val)
                })
            }
            operator_type_id => {
                let mut sub_packets = Vec::new();
                let len_type_id = bits[i];
                i += 1;
                match len_type_id {
                    false => {
                        let total_bit_length = bits[i..(i + 15)].load_be::<usize>();
                        i += 15;
                        let end = i + total_bit_length;
                        while i < end {
                            let (i_new, packet) = Packet::parse_at(i, bits);
                            sub_packets.push(packet);
                            i = i_new;
                        }
                    },
                    true => {
                        let contained_packets = bits[i..(i + 11)].load_be::<usize>();
                        i += 11;
                        for _ in 0..contained_packets {
                            let (i_new, packet) = Packet::parse_at(i, bits);
                            sub_packets.push(packet);
                            i = i_new;
                        }
                    }
                }
                (i, Packet { 
                    version,
                    packet_type: PacketType::Operator(
                        match operator_type_id {
                            0 => Operator::Sum,
                            1 => Operator::Product,
                            2 => Operator::Minimum,
                            3 => Operator::Maximum,
                            5 => Operator::GreaterThan,
                            6 => Operator::LessThan,
                            7 => Operator::EqualTo,
                            _ => panic!("unknown type_id code")
                        }, 
                    sub_packets) 
                })
            }
        }
    }

    fn version_sum(&self) -> u128 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as u128,
            PacketType::Operator(_, sub_packets) => {
                let mut sum = self.version as u128;
                for packet in sub_packets {
                    sum += packet.version_sum();
                }
                sum
            }
        }
    }

    fn value(&self) -> u128 {
        match &self.packet_type {
            PacketType::Literal(v) => *v,
            PacketType::Operator(op, v) => match op {
                Operator::Sum =>  v.iter().map(|p| p.value()).sum(),
                Operator::Product => v.iter().map(|p| p.value()).product(),
                Operator::Minimum => v.iter().map(|p| p.value()).min().unwrap(),
                Operator::Maximum => v.iter().map(|p| p.value()).max().unwrap(),
                Operator::GreaterThan => if v[0].value() > v[1].value() {1} else{0},
                Operator::LessThan => if v[0].value() < v[1].value() {1} else{0},
                Operator::EqualTo => if v[0].value() == v[1].value() {1} else{0},
            }
        }
    }
}

impl From<BitVec<u8, Msb0>> for Packet {
    fn from(bits: BitVec<u8, Msb0>) -> Self {
        let (_, packet) = Packet::parse_at(0, &bits);
        packet
    }
}

fn main() {
    let input = fs::read_to_string("bits.txt").expect("file not found");
    let hex_str = decode(input).unwrap();
    let bits: BitVec<_, Msb0> = BitVec::from_vec(hex_str);

    let packet = Packet::from(bits);
    println!("{:?}", packet);
    println!("{:?}", packet.version_sum());
    println!("{:?}", packet.value());
}

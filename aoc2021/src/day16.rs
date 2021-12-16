use std::fmt::Write;

use anyhow::{Context, Result};
use bitvec::prelude::*;

const INPUT: &str = include_str!("../input/day16.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let packet: Packet = input.parse()?;

    Ok(packet.version_sum())
}

#[derive(PartialEq, Eq, Debug)]
enum PacketType {
    Litteral(LitteralPacket),
    Operator(OperatorPacket),
}

#[derive(PartialEq, Eq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    packet: PacketType,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match &self.packet {
            PacketType::Litteral(_) => self.version as u64,
            PacketType::Operator(op) => op.sub_packets.iter().map(Packet::version_sum).sum(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct LitteralPacket {
    value: u64,
}

#[derive(PartialEq, Eq, Debug)]
struct OperatorPacket {
    sub_packets: Vec<Packet>,
}

impl std::str::FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let mut bits = BitVec::<Msb0, usize>::new();
        bits.resize(4 * s.len(), false);

        for (i, c) in s.chars().enumerate() {
            let bit_index = i * 4;
            let digit = c.to_digit(16).context("character wasn't hex digit")?;
            bits[bit_index..(bit_index + 4)].store(digit);
        }

        bits.as_bitslice().try_into()
    }
}

impl<'bits, Store> TryFrom<&'bits BitSlice<Msb0, Store>> for Packet
where
    Store: BitStore,
{
    type Error = anyhow::Error;

    fn try_from(bits: &'bits BitSlice<Msb0, Store>) -> Result<Self> {
        let version: u8 = bits[0..3].load();
        let type_id: u8 = bits[3..6].load();

        match type_id {
            4 => {
                let mut value = 0;
                for i in (6..).step_by(5) {
                    let val = bits[(i + 1)..(i + 5)].load::<u64>();
                    value = (value << 4) + val;

                    if !bits[i] {
                        break;
                    }
                }

                Ok(Packet {
                    version,
                    type_id,
                    packet: PacketType::Litteral(LitteralPacket { value }),
                })
            }
            _ => unimplemented!("Operator packets aren't implemented yet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "D2FE28";
    const PROVIDED2: &str = "38006F45291200";
    const PROVIDED3: &str = "EE00D40C823060";

    const PROVIDED4: &str = "8A004A801A8002F478";
    const PROVIDED5: &str = "620080001611562C8802118E34";
    const PROVIDED6: &str = "C0015000016115A2E0802F182340";
    const PROVIDED7: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn part1_provided() {
        assert_eq!(
            PROVIDED1.parse::<Packet>().unwrap(),
            Packet {
                version: 6,
                type_id: 4,
                packet: PacketType::Litteral(LitteralPacket { value: 2021 }),
            }
        );
    }
}

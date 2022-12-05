use std::fmt::Write;
use std::ops::Range;

use anyhow::{bail, Context, Result};
use bitvec::prelude::*;

const INPUT: &str = include_str!("../input/day16.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let packet: Packet = input.parse()?;

    Ok(packet.version_sum())
}

fn part2(input: &str) -> Result<u64> {
    let packet: Packet = input.parse()?;

    Ok(packet.value())
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
    bits_used: usize,
    packet: PacketType,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        let version = self.version as u64;
        match &self.packet {
            PacketType::Litteral(_) => version,
            PacketType::Operator(op) => {
                version + op.sub_packets.iter().map(Packet::version_sum).sum::<u64>()
            }
        }
    }

    fn value(&self) -> u64 {
        match &self.packet {
            PacketType::Litteral(lit) => lit.value,
            PacketType::Operator(op) => match op.op_type {
                OperatorType::Sum => op.sub_packets.iter().map(Packet::value).sum(),
                OperatorType::Product => op.sub_packets.iter().map(Packet::value).product(),
                OperatorType::Minimum => op.sub_packets.iter().map(Packet::value).min().unwrap(),
                OperatorType::Maximum => op.sub_packets.iter().map(Packet::value).max().unwrap(),
                OperatorType::GreaterThan => {
                    debug_assert_eq!(op.sub_packets.len(), 2);
                    let pack1 = &op.sub_packets[0];
                    let pack2 = &op.sub_packets[1];
                    if pack1.value() > pack2.value() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::LessThan => {
                    debug_assert_eq!(op.sub_packets.len(), 2);
                    let pack1 = &op.sub_packets[0];
                    let pack2 = &op.sub_packets[1];
                    if pack1.value() < pack2.value() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::EqualTo => {
                    debug_assert_eq!(op.sub_packets.len(), 2);
                    let pack1 = &op.sub_packets[0];
                    let pack2 = &op.sub_packets[1];
                    if pack1.value() == pack2.value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct LitteralPacket {
    value: u64,
}

#[derive(PartialEq, Eq, Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(PartialEq, Eq, Debug)]
struct OperatorPacket {
    sub_packets: Vec<Packet>,
    op_type: OperatorType,
}

impl std::str::FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let mut bits = BitVec::<usize, Msb0>::new();
        bits.resize(4 * s.len(), false);

        for (i, c) in s.chars().enumerate() {
            let bit_index = i * 4;
            let digit = c.to_digit(16).context("character wasn't hex digit")?;
            bits[bit_index..(bit_index + 4)].store(digit);
        }

        bits.as_bitslice().try_into()
    }
}

const VERSION_RANGE: Range<usize> = 0..3;
const TYPE_ID_RANGE: Range<usize> = 3..6;

const TYPE_LENGTH_ID_INDEX: usize = 6;
const LENGTH_INDEX: usize = 6 + 1;

const TLID0_SUBPACKET_LENGTH_BITS: usize = 15;
const SUBPACKET_START_INDEX_TLID0: usize = LENGTH_INDEX + TLID0_SUBPACKET_LENGTH_BITS;
const SUBPACKETS_BIT_LENGTH_RANGE: Range<usize> = LENGTH_INDEX..SUBPACKET_START_INDEX_TLID0;

const TLID1_SUBPACKET_NUMBER_BITS: usize = 11;
const SUBPACKET_START_INDEX_TLID1: usize = LENGTH_INDEX + TLID1_SUBPACKET_NUMBER_BITS;
const SUBPACKETS_NUMBER_RANGE: Range<usize> = LENGTH_INDEX..SUBPACKET_START_INDEX_TLID1;

impl<'bits, Store> TryFrom<&'bits BitSlice<Store, Msb0>> for Packet
where
    Store: BitStore,
{
    type Error = anyhow::Error;

    fn try_from(bits: &'bits BitSlice<Store, Msb0>) -> Result<Self> {
        let version: u8 = bits[VERSION_RANGE].load_be();
        let type_id: u8 = bits[TYPE_ID_RANGE].load_be();

        match type_id {
            // LitteralPacket
            4 => {
                let mut value = 0;
                let mut end = None;
                for i in (6..).step_by(5) {
                    let val = bits[(i + 1)..(i + 5)].load_be::<u64>();
                    value = (value << 4) + val;

                    if !bits[i] {
                        end = Some(i + 5);
                        break;
                    }
                }

                Ok(Packet {
                    version,
                    type_id,
                    bits_used: end.unwrap(),
                    packet: PacketType::Litteral(LitteralPacket { value }),
                })
            }
            // OperatorPacket
            _ => {
                let length_type_id = bits[TYPE_LENGTH_ID_INDEX];
                let mut sub_packets = Vec::new();
                let len = if length_type_id {
                    let subpacket_num: usize = bits[SUBPACKETS_NUMBER_RANGE].load_be();
                    let mut start_index = SUBPACKET_START_INDEX_TLID1;

                    while sub_packets.len() < subpacket_num {
                        let packet: Packet = bits[start_index..].try_into()?;
                        start_index += packet.bits_used;
                        sub_packets.push(packet);
                    }

                    start_index
                } else {
                    let length: usize = bits[SUBPACKETS_BIT_LENGTH_RANGE].load_be();

                    let mut start_index = SUBPACKET_START_INDEX_TLID0;
                    while start_index != SUBPACKET_START_INDEX_TLID0 + length {
                        let packet: Packet = bits[start_index..].try_into()?;
                        start_index += packet.bits_used;
                        sub_packets.push(packet);
                    }

                    SUBPACKET_START_INDEX_TLID0 + length
                };

                let op_type = match type_id {
                    0 => OperatorType::Sum,
                    1 => OperatorType::Product,
                    2 => OperatorType::Minimum,
                    3 => OperatorType::Maximum,
                    4 => unreachable!("impossible, would parse as a litteral packet"),
                    5 => OperatorType::GreaterThan,
                    6 => OperatorType::LessThan,
                    7 => OperatorType::EqualTo,
                    _ => bail!("unknown type id: {}", type_id),
                };

                Ok(Packet {
                    version,
                    type_id,
                    bits_used: len,
                    packet: PacketType::Operator(OperatorPacket {
                        sub_packets,
                        op_type,
                    }),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_PROVIDED1: &str = "D2FE28";
    const PART1_PROVIDED2: &str = "38006F45291200";
    const PART1_PROVIDED3: &str = "EE00D40C823060";

    const PART1_PROVIDED4: &str = "8A004A801A8002F478";
    const PART1_PROVIDED5: &str = "620080001611562C8802118E34";
    const PART1_PROVIDED6: &str = "C0015000016115A2E0802F182340";
    const PART1_PROVIDED7: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn part1_provided() {
        assert_eq!(
            PART1_PROVIDED1.parse::<Packet>().unwrap(),
            Packet {
                version: 6,
                type_id: 4,
                bits_used: 21,
                packet: PacketType::Litteral(LitteralPacket { value: 2021 }),
            }
        );

        assert_eq!(
            PART1_PROVIDED2.parse::<Packet>().unwrap(),
            Packet {
                version: 1,
                type_id: 6,
                bits_used: 49,
                packet: PacketType::Operator(OperatorPacket {
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            type_id: 4,
                            bits_used: 11,
                            packet: PacketType::Litteral(LitteralPacket { value: 10 })
                        },
                        Packet {
                            version: 2,
                            type_id: 4,
                            bits_used: 16,
                            packet: PacketType::Litteral(LitteralPacket { value: 20 })
                        },
                    ],
                    op_type: OperatorType::LessThan,
                }),
            }
        );

        assert_eq!(
            PART1_PROVIDED3.parse::<Packet>().unwrap(),
            Packet {
                version: 7,
                type_id: 3,
                bits_used: 51,
                packet: PacketType::Operator(OperatorPacket {
                    sub_packets: vec![
                        Packet {
                            version: 2,
                            type_id: 4,
                            bits_used: 11,
                            packet: PacketType::Litteral(LitteralPacket { value: 1 })
                        },
                        Packet {
                            version: 4,
                            type_id: 4,
                            bits_used: 11,
                            packet: PacketType::Litteral(LitteralPacket { value: 2 })
                        },
                        Packet {
                            version: 1,
                            type_id: 4,
                            bits_used: 11,
                            packet: PacketType::Litteral(LitteralPacket { value: 3 })
                        },
                    ],
                    op_type: OperatorType::Maximum,
                }),
            }
        );

        assert_eq!(PART1_PROVIDED4.parse::<Packet>().unwrap().version_sum(), 16);
        assert_eq!(PART1_PROVIDED5.parse::<Packet>().unwrap().version_sum(), 12);
        assert_eq!(PART1_PROVIDED6.parse::<Packet>().unwrap().version_sum(), 23);
        assert_eq!(PART1_PROVIDED7.parse::<Packet>().unwrap().version_sum(), 31);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 925);
    }

    const PART2_PROVIDED1: &str = "C200B40A82";
    const PART2_PROVIDED2: &str = "04005AC33890";
    const PART2_PROVIDED3: &str = "880086C3E88112";
    const PART2_PROVIDED4: &str = "CE00C43D881120";
    const PART2_PROVIDED5: &str = "D8005AC2A8F0";
    const PART2_PROVIDED6: &str = "F600BC2D8F";
    const PART2_PROVIDED7: &str = "9C005AC2F8F0";
    const PART2_PROVIDED8: &str = "9C0141080250320F1802104A08";

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PART2_PROVIDED1).unwrap(), 3);
        assert_eq!(part2(PART2_PROVIDED2).unwrap(), 54);
        assert_eq!(part2(PART2_PROVIDED3).unwrap(), 7);
        assert_eq!(part2(PART2_PROVIDED4).unwrap(), 9);
        assert_eq!(part2(PART2_PROVIDED5).unwrap(), 1);
        assert_eq!(part2(PART2_PROVIDED6).unwrap(), 0);
        assert_eq!(part2(PART2_PROVIDED7).unwrap(), 0);
        assert_eq!(part2(PART2_PROVIDED8).unwrap(), 1);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 342997120375);
    }
}

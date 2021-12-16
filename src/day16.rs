use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::{parse_hex, BitReader};

fn main() {
    let input = include_bytes!("../input/day16.txt");

    let (packet, dur_p, dur_pc) = run_many(10000, || Packet::parse_hex(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10000, || packet.version_sum());
    let (res_p2, dur_p2, dur_p2c) = run_many(10000, || packet.value());

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct Packet {
    version: u32,
    type_id: u32,
    value: u64,
    subs: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version + self.subs.iter().map(|s| s.version_sum()).sum::<u32>()
    }

    fn value(&self) -> u64 {
        match self.type_id {
            0 => self.subs.iter().map(|v| v.value()).sum::<u64>(),
            1 => self.subs.iter().map(|v| v.value()).product::<u64>(),
            2 => self.subs.iter().map(|v| v.value()).min().unwrap(),
            3 => self.subs.iter().map(|v| v.value()).max().unwrap(),
            4 => self.value,
            5 => if self.subs[0].value() > self.subs[1].value() { 1 } else { 0 },
            6 => if self.subs[0].value() < self.subs[1].value() { 1 } else { 0 },
            7 => if self.subs[0].value() == self.subs[1].value() { 1 } else { 0 },
            _ => unreachable!(),
        }
    }

    #[inline]
    fn literal(version: u32, value: u64) -> Packet {
        Packet {
            version,
            value,
            type_id: 4,
            subs: Vec::new(),
        }
    }

    #[inline]
    fn operator(version: u32, type_id: u32, subs: Vec<Packet>) -> Packet {
        Packet {
            version,
            type_id,
            subs,
            value: 0,
        }
    }

    fn parse(data: &[u8], off: usize) -> (Packet, usize) {
        let mut reader = BitReader::new(data, off);
        let version = reader.read(3);
        let type_id = reader.read(3);

        match type_id {
            4 => {
                let mut value = 0u64;
                loop {
                    let v = reader.read(5);
                    value <<= 4;
                    value |= (v & 0b01111) as u64;

                    if v & 0b10000 == 0 {
                        break;
                    }
                }

                (Packet::literal(version, value), reader.pos())
            }

            _ => {
                let mut subs = Vec::with_capacity(4);
                let size_type = reader.read(1);
                if size_type == 1 {
                    let count = reader.read(11);
                    for _ in 0..count {
                        let (packet, new_pos) = Packet::parse(data, reader.pos());
                        subs.push(packet);
                        reader.set_pos(new_pos);
                    }
                } else {
                    let total_size = reader.read(15) as usize;
                    let target_pos = reader.pos() + total_size;
                    while reader.pos() < target_pos {
                        let (packet, new_pos) = Packet::parse(data, reader.pos());
                        subs.push(packet);
                        reader.set_pos(new_pos);
                    }
                }

                (Packet::operator(version, type_id, subs), reader.pos())
            }
        }
    }

    fn parse_hex(hex: &[u8]) -> Packet {
        let mut data = vec![0u8; (hex.len() / 2) + 1];
        for (i, h) in hex.iter().filter(|v| **v != b'\n').enumerate() {
            let byte_index = i / 2;
            let bits = if i % 2 == 0 { 4 } else { 0 };

            data[byte_index] |= parse_hex(*h) << bits;
        }

        let (packet, _) = Packet::parse(&data, 0);
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Packet::parse_hex(b"D2FE28"), Packet::literal(6, 2021));

        assert_eq!(Packet::parse_hex(b"38006F45291200"), Packet::operator(
            1, 6, vec![
                Packet::literal(6, 10),
                Packet::literal(2, 20),
            ],
        ));

        assert_eq!(Packet::parse_hex(b"EE00D40C823060"), Packet::operator(
            7, 3, vec![
                Packet::literal(2, 1),
                Packet::literal(4, 2),
                Packet::literal(1, 3),
            ],
        ));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Packet::parse_hex(b"8A004A801A8002F478").version_sum(), 16);
        assert_eq!(Packet::parse_hex(b"620080001611562C8802118E34").version_sum(), 12);
        assert_eq!(Packet::parse_hex(b"C0015000016115A2E0802F182340").version_sum(), 23);
        assert_eq!(Packet::parse_hex(b"A0016C880162017C3686B18A3D4780").version_sum(), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Packet::parse_hex(b"C200B40A82").value(), 3);
        assert_eq!(Packet::parse_hex(b"04005AC33890").value(), 54);
        assert_eq!(Packet::parse_hex(b"880086C3E88112").value(), 7);
        assert_eq!(Packet::parse_hex(b"CE00C43D881120").value(), 9);
        assert_eq!(Packet::parse_hex(b"D8005AC2A8F0").value(), 1);
        assert_eq!(Packet::parse_hex(b"F600BC2D8F").value(), 0);
        assert_eq!(Packet::parse_hex(b"9C005AC2F8F0").value(), 0);
        assert_eq!(Packet::parse_hex(b"9C0141080250320F1802104A08").value(), 1);
    }
}
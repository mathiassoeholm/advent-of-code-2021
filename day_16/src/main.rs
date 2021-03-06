use std::{cell::RefCell, str::Chars};

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    id: u8,
    value: Option<u64>,
    subPackets: Option<Vec<Packet>>,
}

#[derive(PartialEq)]
enum ReadMode {
    Packet,
    Bits,
}

// Packet { version, id, value?, subPackets? }

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let bits = input
        .split("")
        .filter(|v| !v.is_empty())
        .map(|c| match c {
            "0" => "0000",
            "1" => "0001",
            "2" => "0010",
            "3" => "0011",
            "4" => "0100",
            "5" => "0101",
            "6" => "0110",
            "7" => "0111",
            "8" => "1000",
            "9" => "1001",
            "A" => "1010",
            "B" => "1011",
            "C" => "1100",
            "D" => "1101",
            "E" => "1110",
            "F" => "1111",
            _ => panic!("Unknown char"),
        })
        .collect::<Vec<&str>>()
        .join("");
    let mut bits = bits.chars();

    let (packets, _) = parse_packets(&RefCell::new(bits), 1, ReadMode::Packet);
    println!("packets {:?}", packets);

    fn get_version_sum(packets: &Vec<Packet>) -> usize {
        return packets.iter().map(|p| p.version as usize).sum::<usize>()
            + packets
                .iter()
                .map(|p| {
                    if let Some(sub) = p.clone().subPackets {
                        get_version_sum(&sub)
                    } else {
                        0
                    }
                })
                .sum::<usize>();
    }

    fn evaluate_packet(packet: Packet) -> usize {
        match packet.id {
            0 => packet
                .subPackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p.clone()))
                .sum(),
            1 => packet
                .subPackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p.clone()))
                .fold(1, |a, b| a * b),
            2 => packet
                .subPackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p.clone()))
                .min()
                .unwrap(),
            3 => packet
                .subPackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p.clone()))
                .max()
                .unwrap(),
            4 => packet.value.unwrap() as usize,
            5 => {
                let sub_packets = packet.subPackets.unwrap();
                if evaluate_packet(sub_packets[0].clone()) > evaluate_packet(sub_packets[1].clone())
                {
                    1
                } else {
                    0
                }
            }
            6 => {
                let sub_packets = packet.subPackets.unwrap();
                if evaluate_packet(sub_packets[0].clone()) < evaluate_packet(sub_packets[1].clone())
                {
                    1
                } else {
                    0
                }
            }
            7 => {
                let sub_packets = packet.subPackets.unwrap();
                if evaluate_packet(sub_packets[0].clone())
                    == evaluate_packet(sub_packets[1].clone())
                {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    println!("{}", evaluate_packet(packets[0].clone()));
}

fn parse_packets(bits: &RefCell<Chars>, to_read: usize, mode: ReadMode) -> (Vec<Packet>, usize) {
    let bits_read = RefCell::new(0);

    let mut read_bit = || {
        *bits_read.borrow_mut() += 1;
        bits.borrow_mut().next().unwrap()
    };

    let mut packets = Vec::new();
    // while bits_read < bits_to_read {
    while (mode == ReadMode::Packet && packets.len() < to_read)
        || (mode == ReadMode::Bits && *bits_read.borrow() < to_read)
    {
        let version = format!("{}{}{}", read_bit(), read_bit(), read_bit());
        let version = u8::from_str_radix(&version, 2).unwrap();

        let id = format!("{}{}{}", read_bit(), read_bit(), read_bit());
        let id = u8::from_str_radix(&id, 2).unwrap();

        if id == 4 {
            let mut value = "".to_owned();

            // Read first groups
            while read_bit() != '0' {
                value += &read_bit().to_string();
                value += &read_bit().to_string();
                value += &read_bit().to_string();
                value += &read_bit().to_string();
            }

            // Read last group
            value += &read_bit().to_string();
            value += &read_bit().to_string();
            value += &read_bit().to_string();
            value += &read_bit().to_string();

            let value = u64::from_str_radix(&value, 2).unwrap();

            println!("Value is {}", value);

            packets.push(Packet {
                version,
                id,
                value: Some(value),
                subPackets: None,
            });
        } else {
            let length_type = read_bit();
            let length_size = if length_type == '0' { 15 } else { 11 };
            let mut length = "".to_owned();
            for _ in 0..length_size {
                length += &read_bit().to_string();
            }
            let length = usize::from_str_radix(&length, 2).unwrap();
            let next_mode = if length_type == '0' {
                ReadMode::Bits
            } else {
                ReadMode::Packet
            };

            let (sub, sub_bits_read) = parse_packets(bits, length, next_mode);

            packets.push(Packet {
                version,
                id,
                value: None,
                subPackets: Some(sub),
            });

            *bits_read.borrow_mut() += sub_bits_read
            // bits_read += how many bits did we read
        }
        // else
        // subpackets = parse_packets(bits, bits_to_read: 32)
    }

    // return packets

    // if is literal:
    // return [Packet...]
    // else
    // subpackets = parse_packets()
    let temp = *bits_read.borrow();
    (packets, temp)
}

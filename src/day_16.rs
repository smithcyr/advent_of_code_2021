use std::str::Chars;

use lazy_static::lazy_static;

use crate::input::load_resource;

lazy_static! {
    static ref INPUT: String = load_resource("day_16.txt");
}

fn parse_char(c: char) -> String {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Invalid character receieved: \"{}\"", c),
    }
    .to_string()
}

fn parse_input(input: &String) -> String {
    input
        .chars()
        .map(|c| parse_char(c))
        .collect::<Vec<String>>()
        .join("")
}

fn get_version(c_iter: &mut Chars<'_>) -> u8 {
    u8::from_str_radix(&c_iter.take(3).collect::<String>(), 2).unwrap()
}

#[derive(Debug, Eq, PartialEq)]
enum TypeId {
    Literal,
    Operator(u8),
}

fn get_type_id(c_iter: &mut Chars<'_>) -> TypeId {
    match u8::from_str_radix(&c_iter.take(3).collect::<String>(), 2).unwrap() {
        4 => TypeId::Literal,
        n => TypeId::Operator(n),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Length {
    Bits(u32),
    Packets(u32),
}

fn get_length(c_iter: &mut Chars<'_>) -> Length {
    let length_type = c_iter.take(1).next().unwrap();
    if length_type == '0' {
        Length::Bits(u32::from_str_radix(&c_iter.take(15).collect::<String>(), 2).unwrap())
    } else {
        Length::Packets(u32::from_str_radix(&c_iter.take(11).collect::<String>(), 2).unwrap())
    }
}

fn parse_bytes(c_iter: &mut Chars<'_>, result: &mut String) -> u32 {
    let mut counter: u32 = 0;
    loop {
        let byte: Vec<char> = c_iter.take(5).collect();
        counter += 5;
        let byte_string = byte[1..5].iter().collect::<String>();
        *result += &byte_string;
        if byte[0] == '0' {
            break;
        }
    }
    counter
}

fn get_packet_version(c_iter: &mut Chars<'_>, result: &mut u32) -> u32 {
    let mut packet_length: u32 = 0;
    let version = get_version(c_iter);
    *result += version as u32;
    packet_length += 3;
    let type_id = get_type_id(c_iter);
    packet_length += 3;
    if type_id == TypeId::Literal {
        let mut packet: String = String::new();
        packet_length += parse_bytes(c_iter, &mut packet);
    } else {
        packet_length += get_packets_version(c_iter, result);
    }

    packet_length
}
fn get_packets_version(c_iter: &mut Chars<'_>, result: &mut u32) -> u32 {
    let mut total_size: u32 = 0;
    let packets_length = get_length(c_iter);
    match packets_length {
        Length::Bits(length) => {
            total_size += 16;
            let mut remaining_buffer = length;
            while remaining_buffer > 0 {
                let packet_size = get_packet_version(c_iter, result);
                remaining_buffer -= packet_size;
                total_size += packet_size;
            }
        }
        Length::Packets(num_packets) => {
            total_size += 12;
            for _ in 0..num_packets {
                total_size += get_packet_version(c_iter, result);
            }
        }
    };
    total_size
}

fn sum_version(input: &String) -> u32 {
    let mut c_iter = input.trim().chars();

    let mut version: u32 = 0;
    get_packet_version(&mut c_iter, &mut version);
    version
}

pub fn part_1() -> u32 {
    let input = parse_input(&INPUT);
    sum_version(&input)
}

fn parse_packet(c_iter: &mut Chars<'_>, result: &mut u128) -> u32 {
    let mut packet_length: u32 = 0;
    let _version = get_version(c_iter);
    packet_length += 3;
    let type_id = get_type_id(c_iter);
    packet_length += 3;
    match type_id {
        TypeId::Literal => {
            let mut packet: String = String::new();
            packet_length += parse_bytes(c_iter, &mut packet);
            *result = u128::from_str_radix(&packet, 2).unwrap();
        }
        TypeId::Operator(n) => {
            let mut operatee: Vec<u128> = Vec::new();
            packet_length += parse_packets(c_iter, &mut operatee);

            *result = match n {
                // sum
                0 => operatee.iter_mut().fold(0, |acc, x| acc + *x),
                // mul
                1 => operatee.iter_mut().fold(1, |acc, x| acc * *x),
                // min
                2 => operatee
                    .iter_mut()
                    .fold(u128::MAX, |acc, x| if *x < acc { *x } else { acc }),
                // max
                3 => operatee
                    .iter_mut()
                    .fold(0, |acc, x| if *x > acc { *x } else { acc }),
                // greater than
                5 => {
                    if operatee[0] > operatee[1] {
                        1
                    } else {
                        0
                    }
                }
                // less than
                6 => {
                    if operatee[0] < operatee[1] {
                        1
                    } else {
                        0
                    }
                }
                // equal to
                7 => {
                    if operatee[0] == operatee[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("this should not happen, unknown operator {}", n),
            };
        }
    }

    packet_length
}

fn parse_packets(c_iter: &mut Chars<'_>, result: &mut Vec<u128>) -> u32 {
    let mut total_size: u32 = 0;
    match get_length(c_iter) {
        Length::Bits(length) => {
            total_size += 16;
            let mut remaining_buffer = length;
            while remaining_buffer > 0 {
                let mut packet_result: u128 = 0;
                let packet_size = parse_packet(c_iter, &mut packet_result);
                result.push(packet_result);
                remaining_buffer -= packet_size;
                total_size += packet_size;
            }
        }
        Length::Packets(num_packets) => {
            total_size += 12;
            for _ in 0..num_packets {
                let mut packet_result: u128 = 0;
                total_size += parse_packet(c_iter, &mut packet_result);
                result.push(packet_result);
            }
        }
    };

    total_size
}

fn parse_message(input: &String) -> u128 {
    let mut result: u128 = 0;
    parse_packet(&mut input.chars(), &mut result);
    result
}

pub fn part_2() -> u128 {
    let input = parse_input(&INPUT);

    parse_message(&input)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let raw_input = String::from("8A004A801A8002F478");
        let input = parse_input(&raw_input);
        assert_eq!(sum_version(&input), 16);
        let raw_input = String::from("620080001611562C8802118E34");
        let input = parse_input(&raw_input);
        assert_eq!(sum_version(&input), 12);
        let raw_input = String::from("C0015000016115A2E0802F182340");
        let input = parse_input(&raw_input);
        assert_eq!(sum_version(&input), 23);
        let raw_input = String::from("A0016C880162017C3686B18A3D4780");
        let input = parse_input(&raw_input);
        assert_eq!(sum_version(&input), 31);
    }

    #[test]
    fn test_sum() {
        let raw_input = String::from("C200B40A82");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 3);
    }
    #[test]
    fn test_mul() {
        let raw_input = String::from("04005AC33890");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 54);
    }
    #[test]
    fn test_min() {
        let raw_input = String::from("880086C3E88112");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 7);
    }
    #[test]
    fn test_max() {
        let raw_input = String::from("CE00C43D881120");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 9);
    }
    #[test]
    fn test_less_than() {
        let raw_input = String::from("D8005AC2A8F0");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 1);
    }
    #[test]
    fn test_greater_than() {
        let raw_input = String::from("F600BC2D8F");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 0);
    }
    #[test]
    fn test_equal() {
        let raw_input = String::from("9C005AC2F8F0");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 0);
    }
    #[test]
    fn test_add_mul_equality() {
        let raw_input = String::from("9C0141080250320F1802104A08");
        let input = parse_input(&raw_input);
        assert_eq!(parse_message(&input), 1);
    }
}

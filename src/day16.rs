use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let line = &read_lines_as_str("./day16.input")[0];
    let hexadecimal: String = line
        .chars()
        .map(|c| parse_hex(c))
        .collect::<Vec<String>>()
        .join("");
    let answer1 = solve1(&hexadecimal);
    let answer2 = solve2(&hexadecimal);
    println!("Day 16 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn parse_hex(code: char) -> String {
    // 0 = 0000
    // 1 = 0001
    // 2 = 0010
    // 3 = 0011
    // 4 = 0100
    // 5 = 0101
    // 6 = 0110
    // 7 = 0111
    // 8 = 1000
    // 9 = 1001
    // A = 1010
    // B = 1011
    // C = 1100
    // D = 1101
    // E = 1110
    // F = 1111
    match code {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => panic!("poop"),
    }
}

fn parse_packet(
    hex_code: &String,
    mut curr_index: usize,
    mut curr_op_length: usize,
    max_op_length: usize,
    mut curr_op_count: usize,
    max_op_count: usize,
) -> (usize, Vec<usize>) {
    let mut curr_label = BitLabel::VERSION;
    let mut curr_type = PacketType::UNKNOWN;
    let mut versions: Vec<usize> = Vec::new();
    println!("Hex code {}", hex_code);

    loop {
        let mut end_index = curr_index
            + match curr_label {
                BitLabel::VERSION => 3,
                BitLabel::TYPE_ID => 3,
                BitLabel::LITERAL_VALUE => {
                    let (end_pos, val) = find_end_of_literal_packet(hex_code, curr_index);
                    end_pos - curr_index
                }
                BitLabel::OPERATOR_LENGTH_TYPE_ID => 1,
                BitLabel::OPERATOR_LENGTH => 15,
                BitLabel::OPERATOR_SUBPACKET_COUNT => 11,
                _ => panic!("hey"),
            };
        if end_index >= hex_code.len() {
            break;
        }

        let part = hex_code.get(curr_index..end_index).unwrap();
        let parsed_part = usize::from_str_radix(part, 2).unwrap();
        println!(
            "Label {:?} Part {} Parsed {}\n",
            curr_label, part, parsed_part
        );

        if max_op_length > 0 {
            println!(
                "Increase {} by {}",
                curr_op_length,
                (end_index - curr_index)
            );
            curr_op_length += (end_index - curr_index);
        }

        match curr_label {
            BitLabel::VERSION => {
                versions.push(parsed_part);
                curr_label = BitLabel::TYPE_ID;
                if max_op_count > 0 {
                    curr_op_count += 1;
                }
            }
            BitLabel::TYPE_ID => {
                curr_type = match parsed_part {
                    4 => PacketType::LITERAL,
                    0 => PacketType::SUM,
                    1 => PacketType::PRODUCT,
                    2 => PacketType::MINIMUM,
                    3 => PacketType::MAXIMUM,
                    5 => PacketType::GT,
                    6 => PacketType::LT,
                    7 => PacketType::EQUAL,

                    _ => PacketType::UNKNOWN,
                };
                match curr_type {
                    PacketType::LITERAL => {
                        curr_label = BitLabel::LITERAL_VALUE;
                    }
                    _ => {
                        curr_label = BitLabel::OPERATOR_LENGTH_TYPE_ID;
                    }
                    _ => panic!("asdf"),
                }
            }
            BitLabel::OPERATOR_LENGTH_TYPE_ID => {
                curr_label = match parsed_part == 0 {
                    true => BitLabel::OPERATOR_LENGTH,
                    _ => BitLabel::OPERATOR_SUBPACKET_COUNT,
                };
            }
            BitLabel::OPERATOR_LENGTH => {
                let (index, subpacket_versions) =
                    parse_packet(hex_code, end_index, 0, parsed_part, 0, 0);
                for version in subpacket_versions.iter() {
                    versions.push(*version);
                }
                end_index = index;
                curr_label = BitLabel::VERSION;
            }
            BitLabel::OPERATOR_SUBPACKET_COUNT => {
                let (index, subpacket_versions) =
                    parse_packet(hex_code, end_index, 0, 0, 0, parsed_part);
                for version in subpacket_versions.iter() {
                    versions.push(*version);
                }
                end_index = index;
                curr_label = BitLabel::VERSION;
            }
            BitLabel::LITERAL_VALUE => {
                println!("Current op length {}, {}", curr_op_length, max_op_length);
                if max_op_length > 0 && curr_op_length == max_op_length {
                    curr_index = end_index;
                    break;
                }

                println!("Current op count {}, {}", curr_op_length, max_op_length);
                if max_op_count > 0 && curr_op_count == max_op_count {
                    curr_index = end_index;
                    break;
                }
                curr_label = BitLabel::VERSION;
            }
        };

        if max_op_length > 0 || max_op_count > 0 {
            println!(
                "Max op len {}, curr op len {}, Max op ct {}, curr op ct {}",
                max_op_length, curr_op_length, max_op_count, curr_op_count
            );
        }

        curr_index = end_index;
    }

    (curr_index, versions)
}

fn find_end_of_literal_packet(hex_code: &String, start: usize) -> (usize, usize) {
    let mut index = start;
    let mut bin_val = "".to_owned();

    loop {
        bin_val.push_str(hex_code.get(index + 1..index + 5).unwrap());
        if hex_code.chars().nth(index).unwrap() == '0' {
            index += 5;
            break;
        }

        index += 5;
    }

    (index, usize::from_str_radix(&bin_val, 2).unwrap())
}

fn parse_operator_packet() {
    // Type ID
    // - Any type ID value other than 4 - Operator packet
    //   - packet version (3 bits)
    //   - packet type ID (3 bits)
    //   - length type ID
    //      - 0 - next 15 bits = total length in bits of the sub-packets contained by this packet
    //      - 1 - next 11 bits = number of sub-packets immediately contained by this packet
}

#[derive(Debug)]
enum PacketType {
    LITERAL,
    SUM,
    PRODUCT,
    MINIMUM,
    MAXIMUM,
    GT,
    LT,
    EQUAL,
    OPERATOR,
    UNKNOWN,
}

#[derive(Debug)]
enum BitLabel {
    VERSION,
    TYPE_ID,
    LITERAL_VALUE,
    OPERATOR_LENGTH_TYPE_ID,
    OPERATOR_LENGTH,
    OPERATOR_SUBPACKET_COUNT,
}

fn solve1(hex_code: &String) -> usize {
    let (_, versions) = parse_packet(hex_code, 0, 0, 0, 0, 0);
    versions.iter().sum()
}

fn solve2(hex_code: &String) -> u32 {
    0
}

fn read_lines_as_str<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

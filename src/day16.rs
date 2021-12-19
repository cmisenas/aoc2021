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
    let (_, versions, result) = solve(&hexadecimal);
    let answer1: usize = versions.iter().sum();
    let answer2: usize = result[0];
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

fn get_type(bits: usize) -> PacketType {
    match bits {
        4 => PacketType::LITERAL,
        0 => PacketType::SUM,
        1 => PacketType::PRODUCT,
        2 => PacketType::MINIMUM,
        3 => PacketType::MAXIMUM,
        5 => PacketType::GT,
        6 => PacketType::LT,
        7 => PacketType::EQUAL,
        _ => PacketType::UNKNOWN,
    }
}

fn convert_bin_to_number(bin_val: &str) -> usize {
    usize::from_str_radix(bin_val, 2).unwrap()
}

fn parse_packet(
    hex_code: &String,
    curr_type: PacketType,
    mut curr_index: usize,
    mut curr_op_length: usize,
    max_op_length: usize,
    mut curr_op_count: usize,
    max_op_count: usize,
) -> (usize, Vec<usize>, Vec<usize>) {
    let mut curr_label = BitLabel::VERSION;
    let mut subpacket_type = PacketType::UNKNOWN;
    let mut operands: Vec<usize> = Vec::new();
    let mut literal_val = 0;
    let mut versions: Vec<usize> = Vec::new();

    loop {
        let mut end_index = curr_index
            + match curr_label {
                BitLabel::VERSION => 3,
                BitLabel::TYPE_ID => 3,
                BitLabel::LITERAL_VALUE => {
                    let (end_pos, val) = find_end_of_literal_packet(hex_code, curr_index);
                    literal_val = val;
                    end_pos - curr_index
                }
                BitLabel::OPERATOR_LENGTH_TYPE_ID => 1,
                BitLabel::OPERATOR_LENGTH => 15,
                BitLabel::OPERATOR_SUBPACKET_COUNT => 11,
                _ => panic!("Invalid label {:?}", curr_label),
            };

        if end_index > hex_code.len() {
            break;
        }

        let parsed_part = convert_bin_to_number(hex_code.get(curr_index..end_index).unwrap());

        if max_op_length > 0 {
            curr_op_length += (end_index - curr_index);
        }

        match curr_label {
            BitLabel::VERSION => {
                curr_label = BitLabel::TYPE_ID;
                versions.push(parsed_part);
            }
            BitLabel::TYPE_ID => {
                subpacket_type = get_type(parsed_part);
                match subpacket_type {
                    PacketType::LITERAL => {
                        curr_label = BitLabel::LITERAL_VALUE;
                    }
                    PacketType::UNKNOWN => panic!("asdf"),
                    _ => {
                        curr_label = BitLabel::OPERATOR_LENGTH_TYPE_ID;
                    }
                }
            }
            BitLabel::OPERATOR_LENGTH_TYPE_ID => {
                curr_label = match parsed_part == 0 {
                    true => BitLabel::OPERATOR_LENGTH,
                    _ => BitLabel::OPERATOR_SUBPACKET_COUNT,
                };
            }
            BitLabel::OPERATOR_LENGTH => {
                let (index, subpacket_versions, subpacket_operands) = parse_packet(
                    hex_code,
                    subpacket_type.clone(),
                    end_index,
                    0,
                    parsed_part,
                    0,
                    0,
                );
                for version in subpacket_versions.iter() {
                    versions.push(*version);
                }
                operands.push(subpacket_operands[0]);
                if max_op_count > 0 {
                    curr_op_count += 1;
                }
                if max_op_length > 0 {
                    curr_op_length += (index - end_index);
                }
                end_index = index;
                curr_label = BitLabel::VERSION;
            }
            BitLabel::OPERATOR_SUBPACKET_COUNT => {
                let (index, subpacket_versions, subpacket_operands) = parse_packet(
                    hex_code,
                    subpacket_type.clone(),
                    end_index,
                    0,
                    0,
                    0,
                    parsed_part,
                );
                for version in subpacket_versions.iter() {
                    versions.push(*version);
                }
                operands.push(subpacket_operands[0]);
                if max_op_count > 0 {
                    curr_op_count += 1;
                }
                if max_op_length > 0 {
                    curr_op_length += (index - end_index);
                }
                end_index = index;
                curr_label = BitLabel::VERSION;
            }
            BitLabel::LITERAL_VALUE => {
                operands.push(literal_val);

                if max_op_count > 0 {
                    curr_op_count += 1;
                }
                curr_label = BitLabel::VERSION;
            }
        };

        curr_index = end_index;
        if (max_op_count > 0 && curr_op_count == max_op_count)
            || (max_op_length > 0 && curr_op_length == max_op_length)
        {
            break;
        }
    }

    let result = match curr_type {
        PacketType::SUM => operands.iter().sum::<usize>(),
        PacketType::PRODUCT => operands.iter().product::<usize>(),
        PacketType::MINIMUM => *operands.iter().min().unwrap(),
        PacketType::MAXIMUM => *operands.iter().max().unwrap(),
        PacketType::GT => match operands[0] > operands[1] {
            true => 1,
            _ => 0,
        },
        PacketType::LT => match operands[0] < operands[1] {
            true => 1,
            _ => 0,
        },
        PacketType::EQUAL => match operands[0] == operands[1] {
            true => 1,
            _ => 0,
        },
        _ => operands[0],
    };

    operands = vec![result];
    (curr_index, versions, operands)
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

    (index, convert_bin_to_number(&bin_val))
}

#[derive(Debug, Clone, Copy)]
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

fn solve(hex_code: &String) -> (usize, Vec<usize>, Vec<usize>) {
    parse_packet(hex_code, PacketType::UNKNOWN, 0, 0, 0, 0, 0)
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

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day3.input");
    let answer1 = solve1(&lines);
    let answer2 = solve2(&lines);
    println!("Day 3 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn convert_to_decimal(bin: String) -> usize {
    usize::from_str_radix(&bin, 2).unwrap()
}

fn solve1(lines: &Vec<String>) -> usize {
    let mut count = vec![0; lines[0].len()];
    for line in lines.iter() {
        line.split("")
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
            .for_each(|(i, l)| {
                if (l != &"") {
                    count[i - 1] += l.parse::<usize>().unwrap();
                }
            });
    }

    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();

    for bit in count.iter() {
        if bit > &(lines.len() / 2) {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            epsilon.push_str("1");
            gamma.push_str("0");
        }
    }
    convert_to_decimal(gamma.to_string()) * convert_to_decimal(epsilon.to_string())
}

fn solve2(lines: &Vec<String>) -> usize {
    let mut index = 0;
    let mut temp_oxygen_rating = lines.clone();
    let mut temp_co2_rating = lines.clone();
    while index < lines[0].len() {
        if temp_oxygen_rating.len() > 1 {
            let mut temp_o2_rating_1s = vec![];
            let mut temp_o2_rating_0s = vec![];
            for line in temp_oxygen_rating.iter() {
                if line.chars().nth(index).unwrap() == '1' {
                    temp_o2_rating_1s.push(line.to_string());
                } else {
                    temp_o2_rating_0s.push(line.to_string());
                }
            }

            // Most common value
            if temp_o2_rating_1s.len() >= temp_o2_rating_0s.len() {
                // Keep values with 1 or most common value
                temp_oxygen_rating = temp_o2_rating_1s;
            } else {
                temp_oxygen_rating = temp_o2_rating_0s;
            }
        }

        if temp_co2_rating.len() > 1 {
            let mut temp_co2_rating_1s = vec![];
            let mut temp_co2_rating_0s = vec![];
            for line in temp_co2_rating.iter() {
                if line.chars().nth(index).unwrap() == '1' {
                    temp_co2_rating_1s.push(line.to_string());
                } else {
                    temp_co2_rating_0s.push(line.to_string());
                }
            }

            // Least common value
            if temp_co2_rating_0s.len() <= temp_co2_rating_1s.len() {
                // Keep values with 0 or least common value
                temp_co2_rating = temp_co2_rating_0s;
            } else {
                temp_co2_rating = temp_co2_rating_1s;
            }
        }
        index += 1;
    }

    convert_to_decimal(temp_oxygen_rating[0].to_string())
        * convert_to_decimal(temp_co2_rating[0].to_string())
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

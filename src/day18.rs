extern crate itertools;
extern crate regex;

use self::itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day18.input")
        .into_iter()
        .map(|line| parse_snailfish_num(line))
        .collect::<Vec<Vec<String>>>();
    let answer1 = solve1(&lines);
    println!("Day 18 answers");
    println!("Answer 1 {}", answer1);
    let answer2 = solve2(&lines);
    println!("Answer 2 {}", answer2);
}

fn parse_snailfish_num(num: String) -> Vec<String> {
    let mut tokens = Vec::new();
    for (is_num, line) in num
        .chars()
        .group_by(|line| line != &'[' && line != &']' && line != &',')
        .into_iter()
    {
        match is_num {
            true => tokens.push(line.into_iter().collect::<String>()),
            _ => {
                for c in line.into_iter() {
                    tokens.push(c.to_string());
                }
            }
        }
    }
    tokens
}

fn solve1(lines: &[Vec<String>]) -> u32 {
    let mut sum = lines[0].to_vec();

    for x in (1..lines.len()) {
        let mut operand = lines[x].to_vec();
        sum = snailfish_reduce(sum);
        operand = snailfish_reduce(operand);
        sum = snailfish_add(sum, operand);
    }
    sum = calculate_magnitude(snailfish_reduce(sum));

    sum.iter().nth(0).unwrap().parse::<u32>().unwrap()
}

fn solve2(lines: &[Vec<String>]) -> u32 {
    let mut highest_sum = 0;

    for (i, x) in lines.iter().enumerate() {
        for (j, y) in lines.iter().skip(i).enumerate() {
            if i == j {
                continue;
            }
            let mut sum1 =
                calculate_magnitude(snailfish_reduce(snailfish_add(x.to_vec(), y.to_vec())))
                    .iter()
                    .nth(0)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
            let mut sum2 =
                calculate_magnitude(snailfish_reduce(snailfish_add(y.to_vec(), x.to_vec())))
                    .iter()
                    .nth(0)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
            if sum1 > highest_sum {
                highest_sum = sum1;
            }
            if sum2 > highest_sum {
                highest_sum = sum2;
            }
        }
    }

    highest_sum
}

fn can_be_exploded(num: Vec<String>) -> bool {
    let mut level = 0;
    num.iter().any(|c| {
        if c == "[" {
            level = level + 1;
        } else if c == "]" {
            level = level - 1;
        }
        level == 5
    })
}

fn can_be_split(num: Vec<String>) -> bool {
    num.iter()
        .any(|c| is_num(c) && c.parse::<u8>().unwrap() > 9)
}

fn snailfish_reduce(num: Vec<String>) -> Vec<String> {
    let mut level = 0;
    let mut reduced_num = num.to_vec();
    loop {
        if !can_be_exploded(reduced_num.to_vec()) && !can_be_split(reduced_num.to_vec()) {
            break;
        }
        reduced_num = snailfish_explode(reduced_num);
        reduced_num = snailfish_split(reduced_num);
    }

    reduced_num
}

fn snailfish_explode(num: Vec<String>) -> Vec<String> {
    if !can_be_exploded(num.to_vec()) {
        return num;
    }

    let mut i = 0;
    let mut c = "".to_string();
    let mut level = 0;

    for (temp_i, temp_c) in num.clone().iter().enumerate() {
        if level == 5 {
            i = temp_i;
            c = temp_c.to_string();
            break;
        }

        if temp_c == "[" {
            level = level + 1;
        } else if temp_c == "]" {
            level = level - 1;
        }
    }

    let mut new_num: Vec<String> = Vec::new();
    let mut found_open = false;
    let mut found_close = false;
    let mut found_left = false;
    let mut found_right = false;

    // Form the head part
    let left_num = c.to_string();
    for h in num.iter().rev().skip(num.len() - i) {
        if !found_open && h == "[" {
            // Remove one level of nesting
            found_open = true;
        } else if is_num(h) && !found_left {
            let sum = c.parse::<u8>().unwrap() + h.parse::<u8>().unwrap();
            new_num.push(sum.to_string());
            found_left = true;
        } else {
            new_num.push(h.to_string());
        }
    }
    // Make sure to reverse left part
    new_num = new_num
        .iter()
        .rev()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    // Add 0
    new_num.push("0".to_string());

    // Form the tail part
    let right_num = num.iter().nth(i + 2).unwrap();
    for t in num.iter().skip(i + 3) {
        if !found_close && t == "]" {
            // Remove one level of nesting
            found_close = true;
        } else if is_num(t) && !found_right {
            let sum = right_num.parse::<u8>().unwrap() + t.parse::<u8>().unwrap();
            new_num.push(sum.to_string());
            found_right = true;
        } else {
            new_num.push(t.to_string());
        }
    }

    snailfish_explode(new_num)
}

fn snailfish_split(num: Vec<String>) -> Vec<String> {
    if !can_be_split(num.to_vec()) {
        return num;
    }
    let mut new_num = Vec::new();
    let mut already_split = false;
    for c in num.iter() {
        if c != "[" && c != "]" && c != "," && !already_split {
            let n = c.parse::<u8>().unwrap();
            if n > 9 {
                new_num.push("[".to_string());
                new_num.push(((n as f32 / 2.0).floor()).to_string());
                new_num.push(",".to_string());
                new_num.push(((n as f32 / 2.0).ceil()).to_string());
                new_num.push("]".to_string());
                already_split = true;
            } else {
                new_num.push(c.to_string());
            }
        } else {
            new_num.push(c.to_string());
        }
    }
    new_num = snailfish_explode(new_num);

    snailfish_split(new_num)
}

fn snailfish_add(num_a: Vec<String>, num_b: Vec<String>) -> Vec<String> {
    vec![
        vec!["[".to_string()],
        num_a.to_vec(),
        vec![",".to_string()],
        num_b.to_vec(),
        vec!["]".to_string()],
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn calculate_magnitude(snailfish_number: Vec<String>) -> Vec<String> {
    if snailfish_number.len() == 1 {
        return snailfish_number;
    }

    let mut reduced_num: Vec<String> = Vec::new();
    let max = snailfish_number.len();
    let mut i = 0;
    let default_val = "No more els".to_string();

    loop {
        if i >= max {
            break;
        }

        let open_paren = snailfish_number.iter().nth(i).unwrap();
        let num_a = snailfish_number.iter().nth(i + 1).unwrap_or(&default_val);
        let comma = snailfish_number.iter().nth(i + 2).unwrap_or(&default_val);
        let num_b = snailfish_number.iter().nth(i + 3).unwrap_or(&default_val);
        let close_paren = snailfish_number.iter().nth(i + 4).unwrap_or(&default_val);

        if open_paren == "[" && is_num(num_a) && comma == "," && is_num(num_b) && close_paren == "]"
        {
            let magnitude =
                (num_a.parse::<usize>().unwrap() * 3) + (num_b.parse::<usize>().unwrap() * 2);
            reduced_num.push(magnitude.to_string());

            i = i + 5;
        } else {
            reduced_num.push(open_paren.to_string());
            i = i + 1;
        }
    }

    calculate_magnitude(reduced_num)
}

fn is_num(c: &String) -> bool {
    c != "[" && c != "]" && c != ","
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

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
    let lines = read_lines_as_str("./day18.input");
    let answer1 = solve1(&lines);
    println!("Day 18 answers");
    println!("Answer 1 {}", answer1);
    //let answer2 = solve2(&lines);
    //println!("Answer 2 {}", answer2);
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
    println!("{:?}", tokens);
    Vec::new()
}

fn solve1(lines: &[String]) -> u32 {
    let mut sum = lines[0].to_string();
    println!("{:?}", parse_snailfish_num(sum.to_string()));

    for x in (1..lines.len()) {
        let mut operand = lines[x].to_string();
        loop {
            if can_be_exploded(sum.to_string()) {
                sum = snailfish_reduce(sum);
            } else {
                break;
            }
        }
        loop {
            if can_be_exploded(operand.to_string()) {
                operand = snailfish_reduce(operand.to_string());
            } else {
                break;
            }
        }
        println!("Adding {} and {}", sum, operand);
        sum = snailfish_add(sum.to_string(), operand.to_string());
        println!("{:?}", sum);
    }
    loop {
        if can_be_exploded(sum.to_string()) {
            sum = snailfish_reduce(sum);
        } else {
            break;
        }
    }
    println!("{:?}", sum);
    //println!("{:?}", snailfish_reduce(sum));
    //println!("{:?}", snailfish_reduce(snailfish_reduce(sum)));
    0
}

//fn solve2(lines: &[u32]) -> u32 {
//    0
//}
fn can_be_exploded(num: String) -> bool {
    let mut level = 0;

    for c in num.chars() {
        if level == 5 {
            break;
        }

        if c == '[' {
            level = level + 1;
        } else if c == ']' {
            level = level - 1;
        }
    }

    level == 5
}

fn snailfish_reduce(num: String) -> String {
    // Loop through the string
    // Keep track of the number of opening braces
    // When you hit a closing brace, reduce the openin brace count
    // When you hit 5 opening braces,
    //   Explode the leftmost pair
    //      Using the index, explode the leftmost number by decrementing
    //      Using the index, explode the rightmost number by incrementing
    //      Add a 0 to the current place
    let mut level = 0;
    let mut reduced_num = "".to_string();
    for (i, c) in num.char_indices() {
        if level == 5 {
            reduced_num = snailfish_explode(num.to_string(), c, i);
            loop {
                if can_be_exploded(reduced_num.to_string()) {
                    reduced_num = snailfish_explode(reduced_num.to_string(), c, i);
                } else {
                    break;
                }
            }
            println!("Reduced {}", reduced_num);
            reduced_num = snailfish_split(reduced_num);
            println!("Split {}", reduced_num);
            break;
        }
        reduced_num.push(c);

        if c == '[' {
            level = level + 1;
        } else if c == ']' {
            level = level - 1;
        }
    }
    reduced_num
}

fn snailfish_explode(num: String, c: char, i: usize) -> String {
    let mut new_num = "".to_string();
    let mut found_open = false;
    let mut found_close = false;
    let mut found_left = false;
    let mut found_right = false;
    // println!("Found it at {}, inv {}", i, num.len() - i - 1);

    // Form the head part
    let left_num = c;
    // println!("Left num {}", left_num);
    for h in num.chars().rev().skip(num.len() - i) {
        // println!("Looking at head {}", h);
        if !found_open && h == '[' {
            // Remove one level of nesting
            // println!("Remove here");
            found_open = true;
        } else if h != '[' && h != ']' && h != ',' && !found_left {
            let sum = c.to_digit(10).unwrap() + h.to_digit(10).unwrap();
            new_num.push_str(&sum.to_string().chars().rev().collect::<String>());
            found_left = true;
            // println!("Add {} to {} = {}", c, h, sum);
        } else {
            new_num.push(h);
        }
    }
    // Make sure to reverse left part
    new_num = new_num.chars().rev().collect::<String>();
    // println!("Left part {}", new_num);

    // Add 0
    new_num.push('0');

    // Form the tail part
    let right_num = num.chars().nth(i + 2).unwrap();
    println!("num {}", num);
    println!("left num {} right num {}", left_num, right_num);
    println!("left i {} right i {}", i, i + 2);
    // println!("Right num {}", right_num);
    for t in num.chars().skip(i + 3) {
        // println!("Looking at tail {}", t);
        if !found_close && t == ']' {
            // Remove one level of nesting
            // println!("Remove here");
            found_close = true;
        } else if t != '[' && t != ']' && t != ',' && !found_right {
            println!("{} and {}", right_num, t);
            let sum = right_num.to_digit(10).unwrap() + t.to_digit(10).unwrap();
            new_num.push_str(&sum.to_string());
            found_right = true;
            // println!("Add {} to {} = {}", c, t, sum);
        } else {
            new_num.push(t);
        }
    }
    new_num
}

fn snailfish_split(num: String) -> String {
    let mut new_num = "".to_string();
    let mut already_split = false;
    for c in num.chars() {
        if c != '[' && c != ']' && c != ',' && !already_split {
            let n = c.to_digit(10).unwrap();
            if n > 9 {
                new_num.push('[');
                new_num.push_str(&((n as f32 / 2.0).floor()).to_string());
                new_num.push(',');
                new_num.push_str(&((n as f32 / 2.0).ceil()).to_string());
                new_num.push(']');
                already_split = true;
            } else {
                new_num.push(c);
            }
        } else {
            new_num.push(c);
        }
    }

    new_num
}

fn snailfish_add(num_a: String, num_b: String) -> String {
    let mut sum = "[".to_string();
    sum.push_str(&num_a);
    sum.push_str(",");
    sum.push_str(&num_b);
    sum.push_str("]");

    sum.to_string()
}

fn calculate_magnitude(snailfish_number: String) -> usize {
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

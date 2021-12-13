use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines: Vec<Vec<String>> = read_lines_as_str("./day10.input")
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let answer1 = solve1(lines.clone());
    let answer2 = solve2(lines.clone());
    println!("Day 10 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(lines: Vec<Vec<String>>) -> u32 {
    let mut total_score = 0;

    for line in lines.iter() {
        let mut heap: Vec<String> = Vec::new();
        for brace in line.iter() {
            if is_open_brace(brace) {
                heap.push(brace.to_string());
            } else {
                let last_char = heap.pop().unwrap();
                if get_open_brace_match(brace) != &last_char && is_close_brace(brace) {
                    total_score += get_first_part_score(brace);
                }
            }
        }
    }
    total_score
}

fn is_open_brace(brace: &str) -> bool {
    vec!["(", "[", "{", "<"].contains(&brace)
}

fn is_close_brace(brace: &str) -> bool {
    vec![")", "]", "}", ">"].contains(&brace)
}

fn get_first_part_score(brace: &str) -> u32 {
    match brace {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => panic!("poop"),
    }
}

fn get_second_part_score(brace: &str) -> usize {
    match brace {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4,
        _ => panic!("poop"),
    }
}

fn get_open_brace_match(brace: &str) -> &str {
    match brace {
        ")" => "(",
        "]" => "[",
        "}" => "{",
        ">" => "<",
        _ => panic!("poop"),
    }
}

fn get_close_brace_match(brace: &str) -> &str {
    match brace {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        "<" => ">",
        _ => panic!("poop"),
    }
}

fn solve2(lines: Vec<Vec<String>>) -> usize {
    let mut scores: Vec<usize> = lines
        .iter()
        .filter_map(|line| {
            let mut heap: Vec<String> = Vec::new();
            let mut corrupt = false;
            for brace in line.iter() {
                if is_open_brace(brace) {
                    heap.push(brace.to_string());
                } else {
                    let last_char = heap.pop().unwrap();
                    if get_open_brace_match(brace) != &last_char && is_close_brace(brace) {
                        corrupt = true;
                        break;
                    }
                }
            }
            match corrupt {
                true => None,
                _ => Some(heap),
            }
        })
        .map(|heap| {
            heap.iter().rev().fold(0, |acc, brace| {
                (acc * 5) + get_second_part_score(get_close_brace_match(brace))
            })
        })
        .collect();
    scores.sort();

    scores[scores.len() / 2]
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

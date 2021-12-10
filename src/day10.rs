use std::collections::HashMap;
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
    // Use Vec as Heap
    let mut OPEN_BRACES = vec!["(", "[", "{", "<"];
    let mut CLOSE_BRACES = vec![")", "]", "}", ">"];
    let mut MATCHES = HashMap::new();
    MATCHES.insert(")".to_string(), "(".to_string());
    MATCHES.insert("]".to_string(), "[".to_string());
    MATCHES.insert("}".to_string(), "{".to_string());
    MATCHES.insert(">".to_string(), "<".to_string());
    let mut SCORES = HashMap::new();
    SCORES.insert(")".to_string(), 3);
    SCORES.insert("]".to_string(), 57);
    SCORES.insert("}".to_string(), 1197);
    SCORES.insert(">".to_string(), 25137);
    let mut total_score = 0;

    for line in lines.iter() {
        let mut heap: Vec<String> = Vec::new();
        for brace in line.iter() {
            if OPEN_BRACES.contains(&brace.as_str()) {
                heap.push(brace.to_string());
            } else {
                let last_char = heap.pop().unwrap();
                if MATCHES.get(brace).unwrap() != &last_char
                    && CLOSE_BRACES.contains(&brace.as_str())
                {
                    total_score += SCORES.get(brace).unwrap();
                }
            }
        }
    }
    total_score
}

fn solve2(mut lines: Vec<Vec<String>>) -> usize {
    // Use Vec as Heap
    let mut OPEN_BRACES = vec!["(", "[", "{", "<"];
    let mut CLOSE_BRACES = vec![")", "]", "}", ">"];
    let mut CLOSE_MATCHES = HashMap::new();
    CLOSE_MATCHES.insert(")".to_string(), "(".to_string());
    CLOSE_MATCHES.insert("]".to_string(), "[".to_string());
    CLOSE_MATCHES.insert("}".to_string(), "{".to_string());
    CLOSE_MATCHES.insert(">".to_string(), "<".to_string());
    let mut OPEN_MATCHES = HashMap::new();
    OPEN_MATCHES.insert("(".to_string(), ")".to_string());
    OPEN_MATCHES.insert("[".to_string(), "]".to_string());
    OPEN_MATCHES.insert("{".to_string(), "}".to_string());
    OPEN_MATCHES.insert("<".to_string(), ">".to_string());
    let mut SCORES = HashMap::new();
    SCORES.insert(")".to_string(), 1);
    SCORES.insert("]".to_string(), 2);
    SCORES.insert("}".to_string(), 3);
    SCORES.insert(">".to_string(), 4);

    let mut scores: Vec<usize> = lines
        .iter()
        .filter_map(|line| {
            let mut heap: Vec<String> = Vec::new();
            let mut corrupt = false;
            for brace in line.iter() {
                if OPEN_BRACES.contains(&brace.as_str()) {
                    heap.push(brace.to_string());
                } else {
                    let last_char = heap.pop().unwrap();
                    if CLOSE_MATCHES.get(brace).unwrap() != &last_char
                        && CLOSE_BRACES.contains(&brace.as_str())
                    {
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
        .map(|mut heap| {
            let mut score = 0;
            while heap.len() > 0 {
                let last_char = heap.pop().unwrap();
                let match_brace = OPEN_MATCHES.get(&last_char).unwrap();
                score = (score * 5) + SCORES.get(match_brace).unwrap();
            }

            score
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

extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Signal {
    segments: String,
    value: usize,
}

impl Signal {
    fn new(segments: String, value: usize) -> Signal {
        Signal { segments, value }
    }
}

pub fn main() {
    let signals = read_lines_as_str("./day8.input")
        .iter()
        .map(|row| {
            row.split("|")
                .map(|tokens| {
                    tokens
                        .split_whitespace()
                        .filter_map(|token| match token != "|" {
                            true => Some(token.to_string()),
                            _ => None,
                        })
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<Vec<Vec<String>>>>()
        .concat();
    let answer1 = solve1(signals.clone());
    let answer2 = solve2(signals.clone());
    println!("Day 8 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut signals: Vec<Vec<String>>) -> usize {
    let mut total = 0;
    let mut index = 1;
    loop {
        if index >= signals.len() {
            break;
        }
        total += signals[index].iter().fold(0, |acc2, seg| {
            let total1 = acc2
                + match seg.len() {
                    2 | 4 | 3 | 7 => 1,
                    _ => 0,
                };
            total1
        });
        index += 2;
    }
    total
}

fn solve2(mut signals: Vec<Vec<String>>) -> usize {
    let mut total = 0;
    let mut index = 1;
    loop {
        if index >= signals.len() {
            break;
        }
        let decoded = decode_mixed_signal(signals[index - 1].clone());
        let max_len = (signals[index].len() - 1) as usize;
        total += signals[index]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, signal)| {
                acc + (10_usize.pow((max_len - i) as u32)
                    * decoded.get(&sort_str_chars(signal)).unwrap())
            });
        index += 2;
    }
    total
}

fn decode_mixed_signal(mut signals: Vec<String>) -> HashMap<String, usize> {
    let mut decoded = HashMap::new();
    let mut one = "";
    let mut four = "";
    let mut six = "";

    /**
     * Figure out the code first of the following
     * 1, 4, 7, 8 are known
     *
     * 6 segments
     * - 9 - has all segments of 4 and 1
     * - 6 - does not have all segments of 1
     * - 0 - does not have all segments of 4 but all segments of 1
     *
     * 5 segments
     * - 3 - will have all segments of 1
     * - 5 - will have all segments of 6
     * - 2 - remaining
     */
    for signal in signals.iter() {
        if signal.len() == 2 {
            one = signal;
            decoded.insert(sort_str_chars(signal), 1);
        } else if signal.len() == 4 {
            four = signal;
            decoded.insert(sort_str_chars(signal), 4);
        } else if signal.len() == 3 {
            decoded.insert(sort_str_chars(signal), 7);
        } else if signal.len() == 7 {
            decoded.insert(sort_str_chars(signal), 8);
        }
    }

    // 6 segments
    for signal in signals.iter().filter(|s| s.len() == 6) {
        if one.chars().all(|c| signal.contains(c)) && four.chars().all(|c| signal.contains(c)) {
            decoded.insert(sort_str_chars(signal), 9);
        } else if !one.chars().all(|c| signal.contains(c)) {
            six = signal;
            decoded.insert(sort_str_chars(signal), 6);
        } else {
            decoded.insert(sort_str_chars(signal), 0);
        }
    }

    // 5 segments
    for signal in signals.iter().filter(|s| s.len() == 5) {
        if one.chars().all(|c| signal.contains(c)) {
            decoded.insert(sort_str_chars(signal), 3);
        } else if signal.chars().all(|c| six.contains(c)) {
            decoded.insert(sort_str_chars(signal), 5);
        } else {
            decoded.insert(sort_str_chars(signal), 2);
        }
    }

    decoded
}

fn sort_str_chars(signal: &String) -> String {
    let mut sorted = signal.chars().collect::<Vec<char>>();
    sorted.sort();
    sorted.iter().collect::<String>()
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

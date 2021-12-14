extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day14.input");
    let grouped_lines = &lines
        .into_iter()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(line.into_iter().collect::<Vec<String>>()),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>();
    let template = &grouped_lines[0][0];
    let mut instructions: HashMap<&str, &str> = HashMap::new();
    for line in grouped_lines[1].iter() {
        let parsed: Vec<&str> = line.split(" -> ").collect();
        instructions.insert(parsed[0], parsed[1]);
    }
    let answer1 = solve1(template, &instructions);
    let answer2 = solve2(template, &instructions);
    println!("Day 14 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(template: &str, instructions: &HashMap<&str, &str>) -> usize {
    let mut template_vec: Vec<String> = template
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_string())
        .collect();
    let mut temporary_vec = Vec::new();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for _ in 0..10 {
        temporary_vec = Vec::new();
        for (i, element) in template_vec.iter().enumerate().skip(1) {
            let mut pair = template_vec[i - 1].to_string();
            pair.push_str(&element.to_string());
            if let Some(instruction) = instructions.get(&pair.as_str()) {
                temporary_vec.push(template_vec[i - 1].to_string());
                temporary_vec.push(instruction.to_string());
            }
        }
        temporary_vec.push(template_vec.last().unwrap().to_string());
        template_vec = temporary_vec.clone();
    }
    for element in template_vec.iter() {
        *counts.entry(element.to_string()).or_insert(0) += 1;
    }
    counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
        - counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1
}

fn solve2(template: &str, instructions: &HashMap<&str, &str>) -> usize {
    let mut template_vec: Vec<String> = template
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_string())
        .collect();
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut el_counts: HashMap<char, usize> = HashMap::new();
    // Last char will always be the same so make sure to add 1
    el_counts
        .entry(template.chars().last().unwrap())
        .or_insert(1);

    for (i, element) in template_vec.iter().enumerate().skip(1) {
        let mut pair = template_vec[i - 1].to_string();
        pair.push_str(&element.to_string());
        *counts.entry(pair).or_insert(0) += 1;
    }

    for day in 0..40 {
        let mut to_increase: HashMap<String, usize> = HashMap::new();
        for (element, count) in counts.iter_mut() {
            if *count > 0 {
                // Decrease this element's count
                // Get the in between and form the new pair
                if let Some(in_between) = instructions.get(&element.as_str()) {
                    // ax and xb and increase their count
                    let mut pair1 = element.chars().nth(0).unwrap().to_string();
                    pair1.push_str(in_between);
                    let mut pair2 = in_between.to_string();
                    pair2.push_str(&element.chars().nth(1).unwrap().to_string());
                    *to_increase.entry(pair1).or_insert(0) += *count;
                    *to_increase.entry(pair2).or_insert(0) += *count;
                }
                *count = 0;
            }
        }

        for (element, count) in to_increase.iter() {
            *counts.entry(element.to_string()).or_insert(0) += count;
        }
    }

    for (pair, count) in counts.iter() {
        *el_counts.entry(pair.chars().nth(0).unwrap()).or_insert(0) += count;
    }

    el_counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
        - el_counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1
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

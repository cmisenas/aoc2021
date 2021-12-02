use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day2.input");
    let parsed_lines = lines.iter().map(|l| l.split(" ").collect()).collect();
    let answer1 = solve1(&parsed_lines);
    let answer2 = solve2(&parsed_lines);
    println!("Day 2 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(lines: &Vec<Vec<&str>>) -> u32 {
    let mut horizontal = 0;
    let mut vertical = 0;

    for instructions in lines.iter() {
        let dir = instructions[0];
        let units = instructions[1].parse::<u32>().unwrap();

        match dir {
            "forward" => horizontal += units,
            "down" => vertical += units,
            _ => vertical -= units,
        }
    }

    horizontal * vertical
}

fn solve2(lines: &Vec<Vec<&str>>) -> u32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for instructions in lines.iter() {
        let dir = instructions[0];
        let units = instructions[1].parse::<u32>().unwrap();

        match dir {
            "down" => aim += units,
            "up" => aim -= units,
            _ => {
                horizontal += units;
                vertical += aim * units;
            }
        }
    }

    horizontal * vertical
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

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Instruction {
    dir: String,
    unit: u32,
}

pub fn main() {
    let lines = read_lines_as_str("./day2.input");
    let parsed_lines = lines
        .iter()
        .map(|l| {
            let ins = l.split(" ").collect::<Vec<&str>>();
            Instruction {
                dir: ins[0].to_string(),
                unit: ins[1].parse::<u32>().unwrap(),
            }
        })
        .collect::<Vec<Instruction>>();
    let answer1 = solve1(&parsed_lines);
    let answer2 = solve2(&parsed_lines);
    println!("Day 2 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(lines: &Vec<Instruction>) -> u32 {
    let mut horizontal = 0;
    let mut vertical = 0;

    for ins in lines.iter() {
        match ins.dir.as_str() {
            "forward" => horizontal += ins.unit,
            "down" => vertical += ins.unit,
            _ => vertical -= ins.unit,
        }
    }

    horizontal * vertical
}

fn solve2(lines: &Vec<Instruction>) -> u32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for ins in lines.iter() {
        match ins.dir.as_str() {
            "down" => aim += ins.unit,
            "up" => aim -= ins.unit,
            _ => {
                horizontal += ins.unit;
                vertical += aim * ins.unit;
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

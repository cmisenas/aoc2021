use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_int("./day1.input");
    let answer1 = solve1(&lines);
    let answer2 = solve2(&lines);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(lines: &[f32]) -> u32 {
    let mut inc = 0;
    for (i, depth) in lines.iter().skip(1).enumerate() {
        let prev_depth = lines[i];
        if depth > &prev_depth {
            inc += 1;
        }
    }
    inc
}

fn solve2(lines: &[f32]) -> u32 {
    let mut inc = 0;
    for (i, depth) in lines.iter().skip(3).enumerate() {
        let prev_depth = lines[i] + lines[i + 1] + lines[i + 2];
        let curr_depth = lines[i + 1] + lines[i + 2] + depth;
        if curr_depth > prev_depth {
            inc += 1;
        }
    }
    inc
}

fn read_lines_as_int<P>(filename: P) -> Vec<f32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<f32>().unwrap())
        .collect()
}

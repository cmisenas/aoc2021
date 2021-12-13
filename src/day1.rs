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
    lines
        .iter()
        .skip(1)
        .enumerate()
        .fold(0, |acc, (i, depth)| match depth > &lines[i] {
            true => acc + 1,
            _ => acc,
        })
}

fn solve2(lines: &[f32]) -> u32 {
    lines.iter().skip(3).enumerate().fold(0, |acc, (i, depth)| {
        let prev_depth = lines[i] + lines[i + 1] + lines[i + 2];
        let curr_depth = lines[i + 1] + lines[i + 2] + depth;
        match curr_depth > prev_depth {
            true => acc + 1,
            _ => acc,
        }
    })
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

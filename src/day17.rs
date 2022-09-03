use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let input = read_lines_as_str("./day17.input");
    let lines = input[0]
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ")
        .collect::<Vec<&str>>();
    let parse_x = lines[0]
        .strip_prefix("x=")
        .unwrap()
        .split("..")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let parse_y = lines[1]
        .strip_prefix("y=")
        .unwrap()
        .split("..")
        .map(|y| y.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let x_range = (parse_x[0], parse_x[1]);
    let y_range = (parse_y[0], parse_y[1]);
    println!("x range {:?}", x_range);
    println!("y range {:?}", y_range);
    let answer1 = solve1(x_range, y_range);
    //let answer2 = solve2(&lines);
    //println!("Day 17 answers");
    //println!("Answer 1 {}", answer1);
    //println!("Answer 2 {}", answer2);
}

fn solve1(x_range: (isize, isize), y_range: (isize, isize)) -> u32 {
    0
}

fn within_range(start: (isize, isize)) -> bool {
    false
}

fn solve2(lines: &[u32]) -> u32 {
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

extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day19.input");
    let grouped_lines = &lines
        .into_iter()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, line)| match is_empty {
            true => None,
            false => {
                let coords = line
                    .into_iter()
                    .filter_map(|l| match l.contains("--- scanner") {
                        true => None,
                        false => {
                            let coord = l
                                .split(",")
                                .map(|c| c.parse::<isize>().unwrap())
                                .collect::<Vec<isize>>();
                            Some((coord[0], coord[1], coord[2]))
                        }
                    })
                    .collect::<Vec<(isize, isize, isize)>>();
                Some(coords)
            }
        })
        .collect::<Vec<Vec<(isize, isize, isize)>>>();
    println!("{:?}", grouped_lines);
    let answer1 = solve1(&grouped_lines);
    let answer2 = solve2(&grouped_lines);
    println!("Day 19 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn rotate_3d(points: Vec<(isize, isize, isize)>, point: usize) -> Vec<(isize, isize, isize)> {
    let mut rotated = Vec::new();
    rotated
}

fn solve1(lines: &Vec<Vec<(isize, isize, isize)>>) -> u32 {
    0
}

fn solve2(lines: &Vec<Vec<(isize, isize, isize)>>) -> u32 {
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

fn read_lines_as_int<P>(filename: P) -> Vec<u32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<u32>().unwrap())
        .collect()
}

extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Grid {
    content: Vec<Vec<String>>,
    marked: HashSet<(usize, usize)>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            content: vec![vec![".".to_string(); width + 1]; height + 1],
            marked: HashSet::new(),
        }
    }

    fn mark_point(&mut self, x: usize, y: usize) {
        self.content[y][x] = "#".to_string();
        self.marked.insert((x, y));
    }

    fn fold_vertical(&mut self, fold_y: usize) {
        let mut new_content = vec![vec![".".to_string(); self.content[0].len()]; fold_y];
        let mut new_marked = HashSet::new();
        for marked in self.marked.iter() {
            let x = marked.0;
            let y = marked.1;
            if y > fold_y {
                let new_y = (fold_y - (y % fold_y)) % fold_y;
                new_content[new_y][x] = "#".to_string();
                new_marked.insert((x, new_y));
            } else if y == fold_y {
                // Do nothing
            } else {
                new_content[y][x] = "#".to_string();
                new_marked.insert((x, y));
            }
        }
        self.content = new_content;
        self.marked = new_marked;
    }

    fn fold_horizontal(&mut self, fold_x: usize) {
        let mut new_content = vec![vec![".".to_string(); fold_x]; self.content.len()];
        let mut new_marked = HashSet::new();
        for marked in self.marked.iter() {
            let x = marked.0;
            let y = marked.1;
            if x > fold_x {
                let new_x = (fold_x - (x % fold_x)) % fold_x;
                new_content[y][new_x] = "#".to_string();
                new_marked.insert((new_x, y));
            } else if x == fold_x {
                // Do nothing
            } else {
                new_content[y][x] = "#".to_string();
                new_marked.insert((x, y));
            }
        }
        self.content = new_content;
        self.marked = new_marked;
    }

    fn count_marked_points(&self) -> usize {
        self.marked.len()
    }
}

pub fn main() {
    let lines = read_lines_as_str("./day13.input");
    let grouped_lines = &lines
        .into_iter()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(line.into_iter().collect::<Vec<String>>()),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>();
    let coords: Vec<(usize, usize)> = grouped_lines[0]
        .iter()
        .map(|line| {
            let parsed: Vec<&str> = line.split(",").collect();
            (parsed[0].parse().unwrap(), parsed[1].parse().unwrap())
        })
        .collect();
    let instructions = &grouped_lines[1];
    let answer1 = solve1(&coords, &instructions);
    let answer2 = solve2(&coords, &instructions);
    println!("Day 13 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(coords: &[(usize, usize)], instructions: &[String]) -> usize {
    let width = coords
        .iter()
        .fold(0, |acc, coord| if coord.0 > acc { coord.0 } else { acc });
    let height = coords
        .iter()
        .fold(0, |acc, coord| if coord.1 > acc { coord.1 } else { acc });
    let mut paper = Grid::new(width, height);
    for coord in coords.iter() {
        paper.mark_point(coord.0, coord.1);
    }
    let parse_ins: Vec<&str> = instructions[0]
        .strip_prefix("fold along ")
        .unwrap()
        .split("=")
        .collect();
    let fold_dir = parse_ins[0];
    let fold_loc: usize = parse_ins[1].parse().unwrap();

    match fold_dir {
        "x" => paper.fold_horizontal(fold_loc),
        "y" => paper.fold_vertical(fold_loc),
        _ => panic!("poo"),
    }

    paper.count_marked_points()
}

fn solve2(coords: &[(usize, usize)], instructions: &[String]) -> usize {
    let width = coords
        .iter()
        .fold(0, |acc, coord| if coord.0 > acc { coord.0 } else { acc });
    let height = coords
        .iter()
        .fold(0, |acc, coord| if coord.1 > acc { coord.1 } else { acc });
    let mut paper = Grid::new(width, height);
    for coord in coords.iter() {
        paper.mark_point(coord.0, coord.1);
    }

    for instruction in instructions.iter() {
        let parse_ins: Vec<&str> = instruction
            .strip_prefix("fold along ")
            .unwrap()
            .split("=")
            .collect();
        let fold_dir = parse_ins[0];
        let fold_loc: usize = parse_ins[1].parse().unwrap();

        match fold_dir {
            "x" => paper.fold_horizontal(fold_loc),
            "y" => paper.fold_vertical(fold_loc),
            _ => panic!("poo"),
        }
    }

    for row in paper.content.iter() {
        println!("{}", row.join(""));
    }

    paper.count_marked_points()
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

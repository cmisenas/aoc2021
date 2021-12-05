use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct SeaFloor {
    points: Vec<Vec<usize>>,
}

impl SeaFloor {
    fn new() -> SeaFloor {
        SeaFloor {
            points: vec![vec![0; 1000]; 1000],
        }
    }

    fn mark_line(&mut self, start: &Coordinate, end: &Coordinate) {
        let x1 = start.x;
        let x2 = end.x;
        let y1 = start.y;
        let y2 = end.y;
        if x1 == x2 {
            let mut start_y = if y1 < y2 { y1 } else { y2 };
            let end_y = if y1 > y2 { y1 } else { y2 };
            while start_y <= end_y {
                self.points[start_y][x1] += 1;
                start_y += 1;
            }
        } else if y1 == y2 {
            let mut start_x = if x1 < x2 { x1 } else { x2 };
            let end_x = if x1 > x2 { x1 } else { x2 };
            while start_x <= end_x {
                self.points[y1][start_x] += 1;
                start_x += 1;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    fn from_str(xy: String) -> Coordinate {
        let coord = xy
            .split(",")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Coordinate {
            x: coord[0],
            y: coord[1],
        }
    }
}

fn is_horizontal_line(start: &Coordinate, end: &Coordinate) -> bool {
    start.x == end.x
}

fn is_vertical_line(start: &Coordinate, end: &Coordinate) -> bool {
    start.y == end.y
}

pub fn main() {
    let lines = read_lines_as_str("./day5.input");
    let parsed_lines = lines
        .iter()
        .map(|line| {
            let coords = line
                .split(" -> ")
                .map(|coord| Coordinate::from_str(coord.to_string()))
                .collect::<Vec<Coordinate>>();
            (coords[0].clone(), coords[1].clone())
        })
        .collect::<Vec<(Coordinate, Coordinate)>>();
    let answer1 = solve1(&parsed_lines);
    let answer2 = solve2(&parsed_lines);
    println!("Day 5 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(lines: &Vec<(Coordinate, Coordinate)>) -> u32 {
    let mut sea = SeaFloor::new();
    for line in lines.iter() {
        if is_horizontal_line(&line.0, &line.1) || is_vertical_line(&line.0, &line.1) {
            sea.mark_line(&line.0, &line.1);
        }
    }

    let result = sea.points.iter().fold(0, |acc1, row| {
        acc1 + row
            .iter()
            .fold(0, |acc2, col| if col >= &2 { acc2 + 1 } else { acc2 })
    });
    result
}

fn solve2(lines: &Vec<(Coordinate, Coordinate)>) -> u32 {
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

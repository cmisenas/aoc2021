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
        if start.is_horizontal_to(end) {
            let mut start_y = if start.y < end.y { start.y } else { end.y };
            let end_y = if start.y > end.y { start.y } else { end.y };
            while start_y <= end_y {
                self.points[start_y][start.x] += 1;
                start_y += 1;
            }
        } else if start.is_vertical_to(end) {
            let mut start_x = if start.x < end.x { start.x } else { end.x };
            let end_x = if start.x > end.x { start.x } else { end.x };
            while start_x <= end_x {
                self.points[start.y][start_x] += 1;
                start_x += 1;
            }
        } else if start.is_diagonal_to(end) {
            let mut start_x = start.x;
            let mut start_y = start.y;
            let x_diff: i32 = if start_x > end.x { -1 } else { 1 };
            let y_diff: i32 = if start_y > end.y { -1 } else { 1 };
            loop {
                self.points[start_y][start_x] += 1;
                if start_x == end.x {
                    break;
                }
                start_x = (start_x as i32 + x_diff) as usize;
                start_y = (start_y as i32 + y_diff) as usize;
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

    fn is_horizontal_to(&self, end: &Coordinate) -> bool {
        self.x == end.x
    }

    fn is_vertical_to(&self, end: &Coordinate) -> bool {
        self.y == end.y
    }

    fn is_diagonal_to(&self, end: &Coordinate) -> bool {
        (self.x as i32 - end.x as i32).abs() == (self.y as i32 - end.y as i32).abs()
    }
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
        if line.0.is_horizontal_to(&line.1) || line.0.is_vertical_to(&line.1) {
            sea.mark_line(&line.0, &line.1);
        }
    }

    sea.points
        .concat()
        .iter()
        .fold(0, |acc, cell| if cell >= &2 { acc + 1 } else { acc })
}

fn solve2(lines: &Vec<(Coordinate, Coordinate)>) -> u32 {
    let mut sea = SeaFloor::new();
    for line in lines.iter() {
        if line.0.is_horizontal_to(&line.1)
            || line.0.is_vertical_to(&line.1)
            || line.0.is_diagonal_to(&line.1)
        {
            sea.mark_line(&line.0, &line.1);
        }
    }

    sea.points
        .concat()
        .iter()
        .fold(0, |acc, cell| if cell >= &2 { acc + 1 } else { acc })
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

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
        if is_horizontal_line(start, end) {
            let mut start_y = if y1 < y2 { y1 } else { y2 };
            let end_y = if y1 > y2 { y1 } else { y2 };
            while start_y <= end_y {
                self.points[start_y][x1] += 1;
                start_y += 1;
            }
        } else if is_vertical_line(start, end) {
            let mut start_x = if x1 < x2 { x1 } else { x2 };
            let end_x = if x1 > x2 { x1 } else { x2 };
            while start_x <= end_x {
                self.points[y1][start_x] += 1;
                start_x += 1;
            }
        } else if is_diagonal_line(start, end) {
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
}

fn is_horizontal_line(start: &Coordinate, end: &Coordinate) -> bool {
    start.x == end.x
}

fn is_vertical_line(start: &Coordinate, end: &Coordinate) -> bool {
    start.y == end.y
}

fn is_diagonal_line(start: &Coordinate, end: &Coordinate) -> bool {
    (start.x as i32 - end.x as i32).abs() == (start.y as i32 - end.y as i32).abs()
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

    sea.points
        .concat()
        .iter()
        .fold(0, |acc, cell| if cell >= &2 { acc + 1 } else { acc })
}

fn solve2(lines: &Vec<(Coordinate, Coordinate)>) -> u32 {
    let mut sea = SeaFloor::new();
    for line in lines.iter() {
        if is_horizontal_line(&line.0, &line.1)
            || is_vertical_line(&line.0, &line.1)
            || is_diagonal_line(&line.0, &line.1)
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

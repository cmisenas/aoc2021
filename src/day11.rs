use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let octopi: Vec<Vec<u32>> = read_lines_as_str("./day11.input")
        .iter()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();
    let answer1 = solve1(octopi.clone(), 100);
    let answer2 = solve2(octopi.clone());
    println!("Day 11 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn get_neighbors(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // Left
    if point.0 > 0 {
        neighbors.push((point.0 - 1, point.1));
    }

    // Right
    if point.0 < width - 1 {
        neighbors.push((point.0 + 1, point.1));
    }

    // Top
    if point.1 > 0 {
        // Left
        if point.0 > 0 {
            neighbors.push((point.0 - 1, point.1 - 1));
        }

        // X
        neighbors.push((point.0, point.1 - 1));

        // Right
        if point.0 < width - 1 {
            neighbors.push((point.0 + 1, point.1 - 1));
        }
    }

    // Bottom
    if point.1 < height - 1 {
        // Left
        if point.0 > 0 {
            neighbors.push((point.0 - 1, point.1 + 1));
        }

        // X
        neighbors.push((point.0, point.1 + 1));

        // Right
        if point.0 < width - 1 {
            neighbors.push((point.0 + 1, point.1 + 1));
        }
    }

    neighbors.clone()
}

fn solve1(mut octopi: Vec<Vec<u32>>, steps: u8) -> usize {
    let width = octopi[0].len();
    let height = octopi.len();
    let mut total_flushes = 0;
    for _ in 0..steps {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut to_flash: Vec<(usize, usize)> = Vec::new();
        for (y, row) in octopi.iter_mut().enumerate() {
            for (x, octo) in row.iter_mut().enumerate() {
                if *octo < 9 {
                    *octo += 1;
                } else {
                    *octo = 0;
                    flashed.insert((x, y));
                    to_flash.push((x, y));
                }
            }
        }
        while to_flash.len() > 0 {
            let point = to_flash.pop().unwrap();
            let neighbors = get_neighbors(point, width, height);
            for neighbor in neighbors.iter() {
                if !flashed.contains(neighbor) && octopi[neighbor.1][neighbor.0] < 9 {
                    octopi[neighbor.1][neighbor.0] += 1;
                } else {
                    octopi[neighbor.1][neighbor.0] = 0;
                    if !flashed.contains(neighbor) {
                        to_flash.push((neighbor.0, neighbor.1));
                    }
                    flashed.insert((neighbor.0, neighbor.1));
                }
            }
        }
        total_flushes += flashed.len();
    }
    total_flushes
}

fn solve2(mut octopi: Vec<Vec<u32>>) -> usize {
    let width = octopi[0].len();
    let height = octopi.len();
    let total = width * height;
    let mut steps = 0;
    loop {
        steps += 1;
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut to_flash: Vec<(usize, usize)> = Vec::new();
        for (y, row) in octopi.iter_mut().enumerate() {
            for (x, octo) in row.iter_mut().enumerate() {
                if *octo < 9 {
                    *octo += 1;
                } else {
                    *octo = 0;
                    flashed.insert((x, y));
                    to_flash.push((x, y));
                }
            }
        }
        while to_flash.len() > 0 {
            let point = to_flash.pop().unwrap();
            let neighbors = get_neighbors(point, width, height);
            for neighbor in neighbors.iter() {
                if !flashed.contains(neighbor) && octopi[neighbor.1][neighbor.0] < 9 {
                    octopi[neighbor.1][neighbor.0] += 1;
                } else {
                    octopi[neighbor.1][neighbor.0] = 0;
                    if !flashed.contains(neighbor) {
                        to_flash.push((neighbor.0, neighbor.1));
                    }
                    flashed.insert((neighbor.0, neighbor.1));
                }
            }
        }
        if flashed.len() == total {
            break;
        }
    }
    steps
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

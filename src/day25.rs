use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day25.input");
    let cucumbers = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let answer1 = solve1(&cucumbers);
    let answer2 = solve2(&cucumbers);
    println!("Day 25 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut cucumbers: &Vec<Vec<char>>) -> u32 {
    let mut i = 0;
    let height = cucumbers.len();
    let width = cucumbers[0].len();
    let mut east_cucumbers = HashSet::new();
    let mut south_cucumbers = HashSet::new();
    for (y, row) in cucumbers.iter().enumerate() {
        for (x, cucumber) in row.iter().enumerate() {
            match cucumber {
                '>' => east_cucumbers.insert((x, y)),
                'v' => south_cucumbers.insert((x, y)),
                _ => false, // Do nothing,
            };
        }
    }
    loop {
        let mut changed = false;
        let mut new_east_cucumbers: HashSet<(usize, usize)> = HashSet::new();
        let mut new_south_cucumbers: HashSet<(usize, usize)> = HashSet::new();
        let mut grid = vec![vec!['.'; width]; height];
        for east_cucumber in east_cucumbers.iter() {
            let next_pos = (
                match east_cucumber.0 == width - 1 {
                    true => 0,
                    _ => east_cucumber.0 + 1,
                },
                east_cucumber.1,
            );
            if east_cucumbers.contains(&next_pos) || south_cucumbers.contains(&next_pos) {
                new_east_cucumbers.insert(*east_cucumber);
                grid[east_cucumber.1][east_cucumber.0] = '>';
            } else {
                new_east_cucumbers.insert(next_pos);
                grid[next_pos.1][next_pos.0] = '>';
                changed = true;
            }
        }
        for south_cucumber in south_cucumbers.iter() {
            let next_pos = (
                south_cucumber.0,
                match south_cucumber.1 == height - 1 {
                    true => 0,
                    _ => south_cucumber.1 + 1,
                },
            );
            if new_east_cucumbers.contains(&next_pos) || south_cucumbers.contains(&next_pos) {
                new_south_cucumbers.insert(*south_cucumber);
                grid[south_cucumber.1][south_cucumber.0] = 'v';
            } else {
                new_south_cucumbers.insert(next_pos);
                grid[next_pos.1][next_pos.0] = 'v';
                changed = true;
            }
        }
        println!("{}", i);
        for row in grid.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
        east_cucumbers = new_east_cucumbers.clone();
        south_cucumbers = new_south_cucumbers.clone();
        i += 1;
        if !changed {
            break;
        }
    }
    i
}

fn solve2(cucumbers: &Vec<Vec<char>>) -> u32 {
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

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day20.input");
    let img_enhancement = &lines[0];
    let input_image = &lines[2..]
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let answer1 = solve1(img_enhancement, &input_image);
    let answer2 = solve2(img_enhancement, &input_image);
    println!("Day 20 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn get_neighbor_str(
    grid: &Vec<Vec<String>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    default: String,
) -> String {
    let mut neighbors = "".to_owned();

    // Top
    if y == 0 {
        neighbors.push_str(&default);
        neighbors.push_str(&default);
        neighbors.push_str(&default);
    } else {
        // Left
        if x == 0 {
            neighbors.push_str(&default);
        } else {
            neighbors.push_str(&grid[y - 1][x - 1]);
        }
        // Aligned
        neighbors.push_str(&grid[y - 1][x]);
        // Right
        if x == width - 1 {
            neighbors.push_str(&default);
        } else {
            neighbors.push_str(&grid[y - 1][x + 1]);
        }
    }

    // Same row
    // Left
    if x == 0 {
        neighbors.push_str(&default);
    } else {
        neighbors.push_str(&grid[y][x - 1]);
    }
    // Aligned
    neighbors.push_str(&grid[y][x]);
    // Right
    if x == width - 1 {
        neighbors.push_str(&default);
    } else {
        neighbors.push_str(&grid[y][x + 1]);
    }

    // Bottom
    if y == height - 1 {
        neighbors.push_str(&default);
        neighbors.push_str(&default);
        neighbors.push_str(&default);
    } else {
        // Left
        if x == 0 {
            neighbors.push_str(&default);
        } else {
            neighbors.push_str(&grid[y + 1][x - 1]);
        }
        // Aligned
        neighbors.push_str(&grid[y + 1][x]);
        // Right
        if x == width - 1 {
            neighbors.push_str(&default);
        } else {
            neighbors.push_str(&grid[y + 1][x + 1]);
        }
    }

    neighbors.to_string()
}

fn convert_str_to_bin(img_str: String) -> String {
    img_str
        .chars()
        .map(|c| match c {
            '.' => "0",
            '#' => "1",
            _ => panic!("Unexpected char {}", c),
        })
        .collect::<String>()
}

fn convert_bin_to_number(bin_val: &str) -> usize {
    usize::from_str_radix(bin_val, 2).unwrap()
}

fn solve1(img_enhancement: &String, input_image: &Vec<Vec<String>>) -> usize {
    let mut img_copy = input_image.clone();
    let mut lit = 0;
    let mut default = ".".to_string();

    for i in 0..2 {
        if i % 2 == 1 {
            if img_enhancement.chars().nth(0).unwrap() != '.' {
                default = "#".to_string();
            }
        }
        lit = 0;
        let width = img_copy[0].len() + 2;
        let height = img_copy.len() + 2;
        let infinite_img = vec![default.to_string(); width];
        for row in img_copy.iter_mut() {
            row.insert(0, default.to_string());
            row.push(default.to_string());
        }
        img_copy.insert(0, infinite_img.clone());
        img_copy.push(infinite_img.clone());

        let mut new_copy = Vec::new();
        for (y, row) in img_copy.iter().enumerate() {
            new_copy.push(Vec::new());
            for (x, pixel) in row.iter().enumerate() {
                let neighbor_str =
                    get_neighbor_str(&img_copy, x, y, width, height, default.to_string());
                let bin_str = convert_str_to_bin(neighbor_str.to_string());
                let index = convert_bin_to_number(&bin_str);
                let val = img_enhancement.chars().nth(index).unwrap().to_string();
                new_copy[y].push(val.to_string());
                if val == "#" {
                    lit += 1;
                }
            }
        }

        img_copy = new_copy.clone();
    }

    lit
}

fn solve2(img_enhancement: &String, input_image: &Vec<Vec<String>>) -> usize {
    let mut img_copy = input_image.clone();
    let mut lit = 0;
    let mut default = ".".to_string();

    for i in 0..50 {
        if i % 2 == 1 {
            if img_enhancement.chars().nth(0).unwrap() != '.' {
                default = "#".to_string();
            }
        } else {
            if img_enhancement.chars().nth(0).unwrap() != '.' {
                default = ".".to_string();
            }
        }
        lit = 0;
        let width = img_copy[0].len() + 2;
        let height = img_copy.len() + 2;
        let infinite_img = vec![default.to_string(); width];
        for row in img_copy.iter_mut() {
            row.insert(0, default.to_string());
            row.push(default.to_string());
        }
        img_copy.insert(0, infinite_img.clone());
        img_copy.push(infinite_img.clone());

        let mut new_copy = Vec::new();
        for (y, row) in img_copy.iter().enumerate() {
            new_copy.push(Vec::new());
            for (x, pixel) in row.iter().enumerate() {
                let neighbor_str =
                    get_neighbor_str(&img_copy, x, y, width, height, default.to_string());
                let bin_str = convert_str_to_bin(neighbor_str.to_string());
                let index = convert_bin_to_number(&bin_str);
                let val = img_enhancement.chars().nth(index).unwrap().to_string();
                new_copy[y].push(val.to_string());
                if val == "#" {
                    lit += 1;
                }
            }
        }

        img_copy = new_copy.clone();
    }

    lit
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

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let fish = read_lines_as_str("./day6.input")[0]
        .split(",")
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let answer1 = solve1(fish.clone(), 80);
    let answer2 = solve2(fish.clone(), 256);
    println!("Day 6 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut fishies: Vec<i32>, days: i32) -> usize {
    let mut zeroes = 0;
    for day in 0..days {
        for _ in 0..zeroes {
            fishies.push(9);
        }
        zeroes = 0;

        for fish in fishies.iter_mut() {
            if fish == &0 {
                *fish = 6;
            } else {
                *fish = *fish - 1;
                if fish == &0 {
                    zeroes += 1;
                }
            }
        }
    }
    fishies.len()
}

fn solve2(mut fishies: Vec<i32>, days: i32) -> usize {
    let mut zeroes = 0;
    let mut inv_fish = vec![0; 9];

    for fish in fishies.iter() {
        inv_fish[*fish as usize] += 1;
    }

    for day in 0..days {
        let temp_0 = inv_fish[0];

        for i in 1..9 {
            inv_fish[i - 1] = inv_fish[i];
        }

        inv_fish[8] = temp_0;
        inv_fish[6] = inv_fish[6] + temp_0;
    }

    inv_fish.iter().fold(0, |acc, fish| fish + acc)
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

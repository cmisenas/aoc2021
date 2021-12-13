use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let crabs = read_lines_as_str("./day7.input")[0]
        .split(",")
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let answer1 = solve1(crabs.clone());
    let answer2 = solve2(crabs.clone());
    println!("Day 7 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut crabs: Vec<i32>) -> usize {
    crabs.sort();
    let lowest = *crabs.first().unwrap() as usize;
    let highest = *crabs.last().unwrap() as usize;
    let mut costs = (lowest..highest + 1)
        .map(|pos| {
            let cost = calc_cost(crabs.clone(), pos);
            (pos, cost)
        })
        .collect::<Vec<(usize, usize)>>();

    costs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    costs.first().unwrap().1
}

fn calc_cost(crabs: Vec<i32>, pos: usize) -> usize {
    crabs
        .iter()
        .map(|crab| (crab - pos as i32).abs() as usize)
        .sum()
}

fn solve2(mut crabs: Vec<i32>) -> usize {
    crabs.sort();
    let lowest = *crabs.first().unwrap() as usize;
    let highest = *crabs.last().unwrap() as usize;
    let mut costs = (lowest..highest + 1)
        .map(|pos| {
            let cost = calc_cost_growth(crabs.clone(), pos);
            (pos, cost)
        })
        .collect::<Vec<(usize, usize)>>();

    costs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    costs.first().unwrap().1
}

fn calc_cost_growth(crabs: Vec<i32>, pos: usize) -> usize {
    crabs
        .iter()
        .map(|crab| {
            let end = (crab - pos as i32).abs() as usize;
            (1..=end).sum::<usize>()
        })
        .sum()
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

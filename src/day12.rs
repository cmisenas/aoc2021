use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let input = read_lines_as_str("./day12.input");
    let mut caves_ref: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.iter() {
        let c = line.split("-").collect::<Vec<&str>>();
        for pairs in vec![[c[0], c[1]], [c[1], c[0]]] {
            let mut cave = caves_ref.entry(pairs[0]).or_insert(Vec::new());
            cave.push(pairs[1]);
        }
    }
    println!("{:?}", caves_ref);
    let answer1 = solve1(&caves_ref);
    let answer2 = solve2(&caves_ref);
    println!("Day 12 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(caves: &HashMap<&str, Vec<&str>>) -> usize {
    dfs("start", caves, vec!["start"], HashMap::new(), 0, false)
}

fn solve2(caves: &HashMap<&str, Vec<&str>>) -> usize {
    dfs("start", caves, vec!["start"], HashMap::new(), 0, true)
}

fn dfs(
    start: &str,
    caves: &HashMap<&str, Vec<&str>>,
    path: Vec<&str>,
    caves_visited: HashMap<&str, usize>,
    mut all_paths: usize,
    allow_small_twice: bool,
) -> usize {
    let neighbors = caves.get(start).unwrap();
    for neighbor in neighbors.iter() {
        if neighbor == &"start" {
            // Do nothing
        } else if neighbor == &"end" {
            all_paths += 1;
        } else if (is_small_cave(neighbor)
            && (!caves_visited.contains_key(*neighbor)
                || (allow_small_twice && all_small_caves_visited_once(caves_visited.clone()))))
            || is_big_cave(neighbor)
        {
            let mut copy_visits = caves_visited.clone();
            let mut copy_path = path.clone();
            *copy_visits.entry(neighbor).or_insert(0) += 1;
            copy_path.push(neighbor);
            all_paths = dfs(
                neighbor,
                caves,
                copy_path,
                copy_visits,
                all_paths,
                allow_small_twice,
            );
        }
    }
    all_paths
}

fn is_small_cave(name: &str) -> bool {
    name != "start" && name != "end" && name.to_lowercase() == name
}

fn is_big_cave(name: &str) -> bool {
    name.to_uppercase() == name
}

fn all_small_caves_visited_once(caves_visited: HashMap<&str, usize>) -> bool {
    caves_visited
        .iter()
        .filter(|(name, _)| is_small_cave(name))
        .all(|(_, visits)| visits <= &1)
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

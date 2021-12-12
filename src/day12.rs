use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut caves_ref: HashMap<String, Vec<String>> = HashMap::new();
    for line in read_lines_as_str("./day12.input").iter() {
        let c = line.split("-").collect::<Vec<&str>>();
        for pairs in vec![[c[0], c[1]], [c[1], c[0]]] {
            let mut cave = caves_ref.entry(pairs[0].to_string()).or_insert(Vec::new());
            cave.push(pairs[1].to_string());
        }
    }
    println!("{:?}", caves_ref);
    let answer1 = solve1(caves_ref.clone());
    let answer2 = solve2(caves_ref.clone());
    println!("Day 12 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(caves: HashMap<String, Vec<String>>) -> usize {
    let all_paths = dfs(
        "start".to_string(),
        caves.clone(),
        vec!["start".to_string()],
        HashSet::new(),
    );

    all_paths.len()
}

fn dfs(
    start: String,
    caves: HashMap<String, Vec<String>>,
    mut path: Vec<String>,
    mut all_paths: HashSet<String>,
) -> HashSet<String> {
    let neighbors = caves.get(&start).unwrap();
    for neighbor in neighbors.iter() {
        if neighbor == "start" {
            // Do nothing
        } else if neighbor == "end" {
            let mut copy_path = path.clone();
            copy_path.push(neighbor.to_string());
            all_paths.insert(copy_path.join(","));
        } else if (is_small_cave(neighbor.to_string())
            && !path.clone().iter().any(|n| n == neighbor))
            || is_big_cave(neighbor.to_string())
        {
            let mut copy_path = path.clone();
            copy_path.push(neighbor.to_string());
            for path in dfs(
                neighbor.to_string(),
                caves.clone(),
                copy_path,
                all_paths.clone(),
            )
            .iter()
            {
                all_paths.insert(path.to_string());
            }
        }
    }
    all_paths
}

fn solve2(caves: HashMap<String, Vec<String>>) -> usize {
    dfs2(
        "start".to_string(),
        caves.clone(),
        vec!["start".to_string()],
        HashMap::new(),
        0,
    )
}

fn dfs2(
    start: String,
    caves: HashMap<String, Vec<String>>,
    mut path: Vec<String>,
    mut caves_visited: HashMap<String, usize>,
    mut all_paths: usize,
) -> usize {
    let neighbors = caves.get(&start).unwrap();
    for neighbor in neighbors.iter() {
        if neighbor == "start" {
            // Do nothing
        } else if neighbor == "end" {
            let mut copy_path = path.clone();
            copy_path.push(neighbor.to_string());
            all_paths += 1;
        } else if (is_small_cave(neighbor.to_string())
            && (!caves_visited.contains_key(neighbor)
                || all_small_caves_visited_once(caves_visited.clone())))
            || is_big_cave(neighbor.to_string())
        {
            let mut copy_visits = caves_visited.clone();
            let mut copy_path = path.clone();
            *copy_visits.entry(neighbor.to_string()).or_insert(0) += 1;
            copy_path.push(neighbor.to_string());
            all_paths = dfs2(
                neighbor.to_string(),
                caves.clone(),
                copy_path,
                copy_visits,
                all_paths,
            );
        }
    }
    all_paths
}

fn is_small_cave(name: String) -> bool {
    name.as_str() != "start" && name.as_str() != "end" && name.to_lowercase() == name
}

fn is_big_cave(name: String) -> bool {
    name.to_uppercase() == name
}

fn all_small_caves_visited_once(mut caves_visited: HashMap<String, usize>) -> bool {
    caves_visited
        .iter()
        .filter(|(name, _)| is_small_cave(name.to_string()))
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

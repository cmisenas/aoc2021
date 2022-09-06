use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day15.input");
    let grid: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let answer1 = solve1(&grid);
    println!("Day 15 answers");
    println!("Answer 1 {}", answer1);

    let answer2 = solve2(&grid);
    println!("Answer 2 {}", answer2);
}

fn solve1(grid: &[Vec<usize>]) -> usize {
    // Assuming you can only go right or bottom
    let curr = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);

    println!("Start is {}", grid[curr.0][curr.1]);
    println!("End is {}", grid[end.0][end.1]);
    calc_path(grid)
}

fn solve2(lines: &[Vec<usize>]) -> usize {
    0
}

fn calc_path(grid: &[Vec<usize>]) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut unvisited: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            dist.insert((j, i), usize::MAX);
            prev.insert((j, i), (usize::MAX, usize::MAX));
            unvisited.insert((j, i), *col);
        }
    }
    let curr = (0, 0);
    let width = grid[0].len();
    let height = grid.len();
    let target = (width - 1, height - 1);

    dist.entry(curr).and_modify(|d| *d = 0);
    prev.entry(curr).and_modify(|p| *p = (0, 0));

    while unvisited.len() > 0 {
        // Get the vertex in unvisited that has min dist
        let dist_copy = dist.clone();
        let (u, dist_u) = dist_copy
            .iter()
            .filter(|(key, _)| unvisited.contains_key(key))
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();
        // Remove that vertex in unvisited
        unvisited.remove(u);

        // Get neighbors of vertex
        let neighbors = get_neighbors(*u, width, height);
        for v in neighbors.iter() {
            // Calculate the length of distance between neighbor and vertex
            let alt = *dist_u + grid[v.1][v.0];
            let v_dist = dist.get(v).unwrap();

            if alt < *v_dist {
                dist.entry(*v).and_modify(|d| *d = alt);
                prev.entry(*v).and_modify(|p| *p = *u);
            }
        }
    }

    *dist.get(&target).unwrap()
}

fn enlarge(grid: &[Vec<usize>]) -> &[Vec<usize>] {
    let enlarged_grid = grid.clone();
    enlarged_grid
}

fn get_paths_score(path: &Vec<(usize, usize)>, grid: &[Vec<usize>]) -> usize {
    path.iter()
        .fold(0, |acc, coord| acc + grid[coord.1][coord.0])
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
        // X
        neighbors.push((point.0, point.1 - 1));
    }

    // Bottom
    if point.1 < height - 1 {
        // X
        neighbors.push((point.0, point.1 + 1));
    }

    neighbors.clone()
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

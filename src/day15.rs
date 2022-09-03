use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day15.input");
    let paths: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let answer1 = solve1(&paths);
    let answer2 = solve2(&paths);
    println!("Day 15 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(paths: &[Vec<usize>]) -> usize {
    let result = dfs(
        (0, 0),
        paths,
        Vec::new(),
        (paths[0].len() - 1, paths.len() - 1),
    );
    // let result = naive_navigate((0, 0), paths);
    get_paths_score(&result, paths)
}

fn solve2(lines: &[Vec<usize>]) -> usize {
    0
}

fn naive_navigate(mut current_pt: (usize, usize), paths: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut potential_path: Vec<(usize, usize)> = Vec::new();
    let mut end = (paths[0].len() - 1, paths.len() - 1);
    let width = paths[0].len();
    let height = paths.len();

    loop {
        let neighbors = get_neighbors_simple(current_pt, width, height);
        if neighbors.len() > 1 {
            let mut neighbor1 = paths[neighbors[0].1][neighbors[0].0];
            let mut neighbor2 = paths[neighbors[1].1][neighbors[1].0];
            // Compare each one
            if (neighbor1 as i8 - neighbor2 as i8).abs() <= 2 {
                let best_coords1 = get_neighbors_simple(neighbors[0], width, height)
                    .iter()
                    .min_by(|coord1, coord2| {
                        let pt1 = paths[coord1.1][coord1.0];
                        let pt2 = paths[coord2.1][coord2.0];
                        pt1.cmp(&pt2)
                    })
                    .unwrap()
                    .clone();
                let best_coords2 = get_neighbors_simple(neighbors[1], width, height)
                    .iter()
                    .min_by(|coord1, coord2| {
                        let pt1 = paths[coord1.1][coord1.0];
                        let pt2 = paths[coord2.1][coord2.0];
                        pt1.cmp(&pt2)
                    })
                    .unwrap()
                    .clone();
                neighbor1 += paths[best_coords1.1][best_coords1.0];
                neighbor2 += paths[best_coords2.1][best_coords2.0];
            }
            current_pt = match neighbor1 < neighbor2 {
                true => neighbors[0],
                _ => neighbors[1],
            };
            println!(
                "Comparing {:?} < {:?}? {:?} wins by {}",
                neighbors[0],
                neighbors[1],
                current_pt,
                (neighbor1 as i8 - neighbor2 as i8).abs()
            );
        } else {
            current_pt = neighbors[0];
            println!("Only 1 choice? {:?} wins", neighbors[0]);
        }
        potential_path.push(current_pt);

        if current_pt == end {
            break;
        }
    }

    potential_path
}

fn get_best_neighbor_score(coord: (usize, usize), paths: &[Vec<usize>]) -> usize {
    0
}

fn dfs(
    start: (usize, usize),
    grid: &[Vec<usize>],
    mut path: Vec<(usize, usize)>,
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // Distance of 1 from the end
    if (start.0 as i32 - end.0 as i32).abs() + (start.1 as i32 - end.1 as i32).abs() == 1 {
        println!("Found end distance {:?}", start);
        path.push(end);
        return path;
    }

    let neighbors = get_neighbors_simple(start, grid[0].len(), grid[1].len());
    let mut potential_paths: Vec<Vec<(usize, usize)>> = Vec::new();
    for neighbor in neighbors.iter() {
        let mut path_copy = path.clone();
        path_copy.push(start);
        let result = dfs(*neighbor, grid, path_copy, end);
        potential_paths.push(result);
    }

    potential_paths
        .iter()
        .min_by(|path1, path2| get_paths_score(path1, grid).cmp(&get_paths_score(path2, grid)))
        .unwrap()
        .clone()
}

fn get_paths_score(path: &Vec<(usize, usize)>, grid: &[Vec<usize>]) -> usize {
    path.iter()
        .fold(0, |acc, coord| acc + grid[coord.1][coord.0])
}

fn get_neighbors_simple(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // Right
    if point.0 < width - 1 {
        neighbors.push((point.0 + 1, point.1));
    }

    // Bottom
    if point.1 < height - 1 {
        // X
        neighbors.push((point.0, point.1 + 1));
    }

    neighbors.clone()
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

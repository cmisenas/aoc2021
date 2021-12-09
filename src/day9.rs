use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let points: Vec<Vec<u32>> = read_lines_as_str("./day9.input")
        .iter()
        .map(|row| row.chars().map(|pt| pt.to_digit(10).unwrap()).collect())
        .collect();
    let answer1 = solve1(points.clone());
    let answer2 = solve2(points.clone());
    println!("Day 9 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(points: Vec<Vec<u32>>) -> u32 {
    let mut low_points: Vec<u32> = Vec::new();
    for (y, row) in points.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            let mut neighbors: Vec<u32> = Vec::new();
            if x > 0 {
                neighbors.push(row[x - 1]);
            }

            if x < row.len() - 1 {
                neighbors.push(row[x + 1]);
            }

            if y > 0 {
                neighbors.push(points[y - 1][x]);
            }

            if y < points.len() - 1 {
                neighbors.push(points[y + 1][x]);
            }

            if neighbors.iter().all(|n| n > point) {
                low_points.push(*point);
            }
        }
    }
    low_points.iter().fold(0, |acc, pt| pt + 1 + acc)
}

fn solve2(points: Vec<Vec<u32>>) -> usize {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (y, row) in points.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            let mut neighbors: Vec<u32> = Vec::new();
            if x > 0 {
                neighbors.push(row[x - 1]);
            }

            if x < row.len() - 1 {
                neighbors.push(row[x + 1]);
            }

            if y > 0 {
                neighbors.push(points[y - 1][x]);
            }

            if y < points.len() - 1 {
                neighbors.push(points[y + 1][x]);
            }

            if neighbors.iter().all(|n| n > point) {
                low_points.push((x, y));
            }
        }
    }

    // For each low point, calculate the basin
    // To calculate the basin,
    // - start from low point
    // - do a dfs ensure the top, down, left and right are not 9 or have been visited
    let mut basin_scores: Vec<usize> = low_points
        .iter()
        .map(|low_point| {
            dfs(&points, HashSet::new(), *low_point).len()
            //    .iter()
            //    .fold(0, |acc, (x, y)| acc + points[*y][*x])
        })
        .collect();

    basin_scores.sort_by(|a, b| b.cmp(a));
    basin_scores.iter().take(3).product::<usize>()
}

fn dfs(
    points: &Vec<Vec<u32>>,
    mut visited: HashSet<(usize, usize)>,
    start: (usize, usize),
) -> HashSet<(usize, usize)> {
    visited.insert(start);

    // Left
    if start.0 > 0
        && points[start.1][start.0 - 1] != 9
        && !visited.contains(&(start.0 - 1, start.1))
    {
        let left = (start.0 - 1, start.1);
        visited.insert(left);
        for result in dfs(points, visited.clone(), left).iter() {
            visited.insert(*result);
        }
    }

    // Right
    if start.0 < points[0].len() - 1
        && points[start.1][start.0 + 1] != 9
        && !visited.contains(&(start.0 + 1, start.1))
    {
        let right = (start.0 + 1, start.1);
        visited.insert(right);
        for result in dfs(points, visited.clone(), right).iter() {
            visited.insert(*result);
        }
    }

    // Up
    if start.1 > 0
        && points[start.1 - 1][start.0] != 9
        && !visited.contains(&(start.0, start.1 - 1))
    {
        let up = (start.0, start.1 - 1);
        visited.insert(up);
        for result in dfs(points, visited.clone(), up).iter() {
            visited.insert(*result);
        }
    }

    // Down
    if start.1 < points.len() - 1
        && points[start.1 + 1][start.0] != 9
        && !visited.contains(&(start.0, start.1 + 1))
    {
        let down = (start.0, start.1 + 1);
        visited.insert(down);
        for result in dfs(points, visited.clone(), down).iter() {
            visited.insert(*result);
        }
    }

    visited.clone()
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

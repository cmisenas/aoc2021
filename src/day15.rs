use std::collections::HashMap;
use std::collections::HashSet;
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
    dijkstra(grid, grid[0].len(), grid.len())
}

fn solve2(grid: &[Vec<usize>]) -> usize {
    dijkstra(grid, grid[0].len() * 5, grid.len() * 5)
}

fn dijkstra(grid: &[Vec<usize>], width: usize, height: usize) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let curr = (0, 0);
    let target = (width - 1, height - 1);

    dist.insert(curr, 0);

    loop {
        println!("{:?}", visited.len());
        // Get the vertex in visited that has min dist
        let (u, dist_u) = dist
            .iter()
            .filter_map(|(key, val)| {
                if !visited.contains(key) {
                    Some((*key, *val))
                } else {
                    None
                }
            })
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();
        // Remove that vertex in visited
        visited.insert(u);
        if u.1 == target.1 && u.0 == target.0 {
            break;
        }

        // Get neighbors of vertex
        let neighbors = get_neighbors(u, width, height);
        for v in neighbors.iter() {
            // Calculate the length of distance between neighbor and vertex
            let alt = dist_u + get_grid_val(grid, v.0, v.1, width, height);
            dist.entry(*v).or_insert(alt);
            let v_dist = dist.get(v).unwrap();

            if alt < *v_dist {
                dist.entry(*v).and_modify(|d| *d = alt);
            }
        }
    }

    *dist.get(&target).unwrap()
}

fn get_grid_val(grid: &[Vec<usize>], x: usize, y: usize, width: usize, height: usize) -> usize {
    let distance =
        (x as f32 / width as f32).floor() as usize + (y as f32 / height as f32).floor() as usize;
    let true_x = x % width;
    let true_y = y % height;

    if ((grid[true_y][true_x] + distance) % 9) == 0 {
        9
    } else {
        (grid[true_y][true_x] + distance) % 9
    }
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

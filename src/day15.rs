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
    let answer2 = solve2(&grid);
    println!("Day 15 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(grid: &[Vec<usize>]) -> usize {
    dijkstra(grid, grid[0].len(), grid.len())
}

fn solve2(grid: &[Vec<usize>]) -> usize {
    a_star(grid, grid[0].len() * 5, grid.len() * 5)
}

fn dijkstra(grid: &[Vec<usize>], width: usize, height: usize) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let true_width = grid[0].len();
    let true_height = grid[0].len();

    let curr = (0, 0);
    let target = (width - 1, height - 1);

    dist.insert(curr, 0);

    loop {
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
            let alt = dist_u + get_grid_val(grid, v.0, v.1, true_width, true_height);
            dist.entry(*v).or_insert(alt);
            let v_dist = dist.get(v).unwrap();

            if alt < *v_dist {
                dist.entry(*v).and_modify(|d| *d = alt);
            }
        }
    }

    *dist.get(&target).unwrap()
}

fn a_star(grid: &[Vec<usize>], width: usize, height: usize) -> usize {
    let mut to_explore: HashSet<(usize, usize)> = HashSet::new();
    let start = (0, 0);
    let target = (width - 1, height - 1);
    let true_width = grid[0].len();
    let true_height = grid[0].len();

    // Set start as to_explore
    to_explore.insert((0, 0));

    // For node n, prev[n] stores the node immediately preceding it
    // on the cheapest path from start to n currently known
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // For node n, g_score[n] is the cost of the cheapest path from
    // start to n currently known
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    g_score.insert(start, 0);

    // For node n, f_score[n] is the current best guess as to how
    // cheap a path could be from start to finish if it goes through n
    // f_score[n] = g_score[n] + heuristic(n)
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();
    f_score.insert(start, manhattan_d(start, target));

    let mut final_score = 0;

    while to_explore.len() > 0 {
        // Set current to the node in to_explore having the lowest f_score[] value
        // TODO: Use min-heap or a priority queue which can occur in O(Log(N))
        let mut current = to_explore
            .iter()
            .min_by(|x, y| f_score.get(x).unwrap().cmp(f_score.get(y).unwrap()))
            .unwrap()
            .clone();

        // Terminate if current is equal to goal and add path value
        if current.0 == target.0 && current.1 == target.1 {
            final_score = get_grid_val(grid, current.0, current.1, true_width, true_height);
            while let Some(curr) = prev.get(&current) {
                if *curr != start {
                    final_score =
                        final_score + get_grid_val(grid, curr.0, curr.1, true_width, true_height);
                }
                current = *curr;
            }
            break;
        }

        // Remove current from to_explore
        to_explore.remove(&current);

        // Get all neighbors of current
        let neighbors = get_neighbors(current, width, height);
        for n in neighbors.iter() {
            // tentative_g_score is the distance from start to the neighbor through current
            let tentative_g = g_score.get(&current).unwrap()
                + get_grid_val(grid, n.0, n.1, true_width, true_height);
            if !g_score.contains_key(&n) || tentative_g < *g_score.get(&n).unwrap() {
                prev.entry(*n)
                    .and_modify(|p| *p = current)
                    .or_insert(current);
                g_score
                    .entry(*n)
                    .and_modify(|g| *g = tentative_g)
                    .or_insert(tentative_g);
                f_score
                    .entry(*n)
                    .and_modify(|f| *f = tentative_g + manhattan_d(*n, target))
                    .or_insert(tentative_g + manhattan_d(*n, target));

                if !to_explore.contains(n) {
                    to_explore.insert(*n);
                }
            }
        }
    }

    final_score
}

fn manhattan_d(coord: (usize, usize), target: (usize, usize)) -> usize {
    (target.0 - coord.0) + (target.1 + coord.1)
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

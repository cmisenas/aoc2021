use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let input = read_lines_as_str("./day17.input");
    let lines = input[0]
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ")
        .collect::<Vec<&str>>();
    let parse_x = lines[0]
        .strip_prefix("x=")
        .unwrap()
        .split("..")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let parse_y = lines[1]
        .strip_prefix("y=")
        .unwrap()
        .split("..")
        .map(|y| y.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let x_range = (parse_x[0], parse_x[1]);
    let y_range = (parse_y[0], parse_y[1]);
    let answer = solve(x_range, y_range);
    println!("Day 17 answers");
    println!("Answer 1 {}", answer.0);
    println!("Answer 2 {}", answer.1);
}

fn solve(x_range: (isize, isize), y_range: (isize, isize)) -> (isize, usize) {
    let mut highest_y = 0;
    let mut good_hits = 0;

    for x in 1..(x_range.1 + 1) {
        for y in y_range.0..((y_range.0 * -1) + 1) {
            let highest_pos_y = within_range(x, y, x_range, y_range);
            if highest_pos_y >= 0 {
                good_hits = good_hits + 1;
            }
            if highest_pos_y > highest_y {
                highest_y = highest_pos_y;
            }
        }
    }
    (highest_y, good_hits)
}

fn within_range(
    vel_x: isize,
    vel_y: isize,
    x_range: (isize, isize),
    y_range: (isize, isize),
) -> isize {
    let mut does_hit = -1;
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut highest_y = pos_y;
    let mut curr_x = vel_x;
    let mut curr_y = vel_y;
    let valid_x_range = x_range.0..(x_range.1 + 1);
    let valid_y_range = y_range.0..(y_range.1 + 1);
    while pos_x < x_range.1 && pos_y > y_range.0 {
        pos_x = pos_x + curr_x;
        pos_y = pos_y + curr_y;
        if curr_x > 0 {
            curr_x = curr_x - 1;
        }
        curr_y = curr_y - 1;
        if pos_y > highest_y {
            highest_y = pos_y;
        }

        if valid_x_range.contains(&pos_x) && valid_y_range.contains(&pos_y) {
            does_hit = highest_y;
            break;
        }
    }
    does_hit
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

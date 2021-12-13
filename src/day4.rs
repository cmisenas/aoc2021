extern crate itertools;

use self::itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct Board {
    content: Vec<Vec<String>>,
    marked: Vec<Vec<usize>>,
}

impl Board {
    fn new(content: Vec<Vec<String>>) -> Board {
        Board {
            marked: vec![vec![0; content[0].len()]; content.len()],
            content,
        }
    }

    fn mark_number(&mut self, number: String) {
        if let Some((row, col)) = get_row_col(self.content.clone(), number) {
            self.marked[row][col] = 1;
        }
    }

    fn sum_unmarked_nums(&self) -> usize {
        sum_unmarked_nums(self.marked.clone(), self.content.clone())
    }

    fn check_winner(&self) -> Option<Vec<String>> {
        check_winner(self.marked.clone(), self.content.clone())
    }
}

fn check_winner(marked: Vec<Vec<usize>>, content: Vec<Vec<String>>) -> Option<Vec<String>> {
    let max = marked.len();
    let h_result = marked
        .iter()
        .enumerate()
        .find(|(_, row)| row.iter().all(|&col| col == 1));

    let v_result =
        (0..content[0].len()).find(|&i| &marked.iter().fold(0, |acc, row| acc + row[i]) == &max);

    if let Some((h_i, _)) = h_result {
        Some(content[h_i].clone())
    } else if let Some(v_i) = v_result {
        Some(
            content
                .iter()
                .map(|row| row[v_i].to_string())
                .collect::<Vec<String>>(),
        )
    } else {
        None
    }
}

fn get_row_col(content: Vec<Vec<String>>, cell: String) -> Option<(usize, usize)> {
    let mut coords: Option<(usize, usize)> = None;
    content.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, col)| {
            if &cell == col {
                coords = Some((i, j));
            }
        })
    });
    coords
}

fn sum_unmarked_nums(marked: Vec<Vec<usize>>, content: Vec<Vec<String>>) -> usize {
    content.iter().enumerate().fold(0, |acc1, (i, row)| {
        acc1 + row.iter().enumerate().fold(0, |acc2, (j, col)| {
            if marked[i][j] == 0 {
                col.parse::<usize>().unwrap() + acc2
            } else {
                acc2
            }
        })
    })
}

pub fn main() {
    let lines = read_lines_as_str("./day4.input");
    let grouped_lines = lines
        .clone()
        .into_iter()
        .group_by(|line| line == "")
        .into_iter()
        .filter_map(|(is_empty, line)| match !is_empty {
            true => Some(line.into_iter().collect::<Vec<String>>()),
            _ => None,
        })
        .collect::<Vec<Vec<String>>>();
    let seq = &grouped_lines[0][0].split(",").collect::<Vec<&str>>();
    let boards = &mut grouped_lines[1..]
        .iter()
        .map(|b| {
            Board::new(
                b.iter()
                    .map(|row| {
                        row.trim()
                            .split_whitespace()
                            .filter(|chr| chr != &"")
                            .map(|chr| chr.to_string())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>(),
            )
        })
        .collect::<Vec<Board>>();

    let answer1 = solve1(seq.clone(), boards.clone());
    let answer2 = solve2(seq.clone(), boards.clone());
    println!("Day 4 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(seq: Vec<&str>, mut boards: Vec<Board>) -> usize {
    let mut result = 0;
    'outer: for num in seq.iter() {
        for board in boards.iter_mut() {
            board.mark_number(num.to_string());
            if let Some(_) = board.check_winner() {
                let sum = board.sum_unmarked_nums();
                result = sum * num.parse::<usize>().unwrap();
                break 'outer;
            }
        }
    }

    result
}

fn solve2(seq: Vec<&str>, mut boards: Vec<Board>) -> usize {
    let mut result = 0;
    for num in seq.iter() {
        let mut winners: Vec<usize> = Vec::new();
        for (index, board) in boards.iter_mut().enumerate() {
            board.mark_number(num.to_string());
            if let Some(_) = board.check_winner() {
                let sum = board.sum_unmarked_nums();
                result = sum * num.parse::<usize>().unwrap();
                winners.push(index);
            }
        }

        for (offset, index) in winners.iter().enumerate() {
            boards.remove(index - offset);
        }
    }

    result
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

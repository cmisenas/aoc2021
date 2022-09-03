use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Deref, DerefMut};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day24.input");
    let parsed_lines: Vec<Vec<&str>> = lines.iter().map(|line| line.split(" ").collect()).collect();
    let answer1 = solve1(&parsed_lines);
    let answer2 = solve2(&lines);
    println!("Day 24 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

#[derive(Debug)]
struct Monad {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
    model: usize,
    index: usize,
}

impl Monad {
    fn new(model: usize) -> Monad {
        Monad {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            model,
            index: 0,
        }
    }

    fn inp(&mut self, var: char) {
        let val = self.model as isize / 10_isize.pow(13 - self.index as u32) % 10;
        match var {
            'w' => self.w = val,
            'x' => self.x = val,
            'y' => self.y = val,
            'z' => self.z = val,
            _ => panic!("Invalid char {}", var),
        };
        // println!("{:?}", self);
        self.index += 1;
    }

    // TODO: Not sure how to return a ref to a struct field :grimacing:
    fn get_var(&self, var: char) -> isize {
        match var {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("Invalid char {}", var),
        }
    }

    // TODO: Not sure how to have param2 be of 2 possible types
    fn add(&mut self, param1: char, param2: char) {
        let mut op2 = self.get_var(param2);
        self.add_literal(param1, op2);
    }

    fn add_literal(&mut self, param1: char, param2: isize) {
        let mut op1 = self.get_var(param1);
        let mut op2 = param2;
        match param1 {
            'w' => self.w = (op1 + op2),
            'x' => self.x = (op1 + op2),
            'y' => self.y = (op1 + op2),
            'z' => self.z = (op1 + op2),
            _ => panic!("Invalid char {}", param1),
        }
        // println!("Added {:?}", self);
    }

    fn mul(&mut self, param1: char, param2: char) {
        let mut op2 = self.get_var(param2);
        self.mul_literal(param1, op2);
    }

    fn mul_literal(&mut self, param1: char, param2: isize) {
        let mut op1 = self.get_var(param1);
        let mut op2 = param2;
        match param1 {
            'w' => self.w = (op1 * op2),
            'x' => self.x = (op1 * op2),
            'y' => self.y = (op1 * op2),
            'z' => self.z = (op1 * op2),
            _ => panic!("Invalid char {}", param1),
        }
        // println!("Multiplied {:?}", self);
    }

    fn div(&mut self, param1: char, param2: char) {
        let mut op2 = self.get_var(param2);
        self.div_literal(param1, op2);
    }

    fn div_literal(&mut self, param1: char, param2: isize) {
        let mut op1 = self.get_var(param1);
        let mut op2 = param2;
        match param1 {
            'w' => self.w = (op1 / op2),
            'x' => self.x = (op1 / op2),
            'y' => self.y = (op1 / op2),
            'z' => self.z = (op1 / op2),
            _ => panic!("Invalid char {}", param1),
        }
        // println!("Divided {:?}", self);
    }

    fn mod_(&mut self, param1: char, param2: char) {
        let mut op2 = self.get_var(param2);
        self.mod_literal(param1, op2);
    }

    fn mod_literal(&mut self, param1: char, param2: isize) {
        let mut op1 = self.get_var(param1);
        let mut op2 = param2;
        match param1 {
            'w' => self.w = (op1 % op2),
            'x' => self.x = (op1 % op2),
            'y' => self.y = (op1 % op2),
            'z' => self.z = (op1 % op2),
            _ => panic!("Invalid char {}", param1),
        }
        // println!("Modded {:?}", self);
    }

    fn eql(&mut self, param1: char, param2: char) {
        let mut op2 = self.get_var(param2);
        self.eql_literal(param1, op2);
    }

    fn eql_literal(&mut self, param1: char, param2: isize) {
        let mut op1 = self.get_var(param1);
        let mut op2 = param2;
        let result = match op1 == op2 {
            true => 1,
            _ => 0,
        };
        match param1 {
            'w' => self.w = result,
            'x' => self.x = result,
            'y' => self.y = result,
            'z' => self.z = result,
            _ => panic!("Invalid char {}", param1),
        }
        // println!("Equal {:?}", self);
    }
}

fn solve1(lines: &Vec<Vec<&str>>) -> usize {
    let mut model_num = 99999999999999;

    loop {
        // if model_num.to_string().contains('0') {
        //     let add = model_num
        //         .to_string()
        //         .chars()
        //         .map(|c| match c {
        //             '0' => '1',
        //             _ => '0',
        //         })
        //         .collect::<String>()
        //         .parse::<usize>()
        //         .unwrap();
        //     println!("Add this {}", add);
        //     model_num += add;
        // }
        let mut monad = Monad::new(model_num);
        for line in lines.iter() {
            let op1 = line[1].chars().next().unwrap();
            match line[0] {
                "inp" => monad.inp(op1),
                _ => {
                    let op2 = line[2].chars().next().unwrap();
                    match line[2].to_string().parse::<isize>() {
                        Ok(val) => match line[0] {
                            "add" => monad.add_literal(op1, val),
                            "mul" => monad.mul_literal(op1, val),
                            "div" => monad.div_literal(op1, val),
                            "mod" => monad.mod_literal(op1, val),
                            "eql" => monad.eql_literal(op1, val),
                            _ => panic!("Invalid op {}", line[0]),
                        },
                        _ => match line[0] {
                            "add" => monad.add(op1, op2),
                            "mul" => monad.mul(op1, op2),
                            "div" => monad.div(op1, op2),
                            "mod" => monad.mod_(op1, op2),
                            "eql" => monad.eql(op1, op2),
                            _ => panic!("Invalid op {}", line[0]),
                        },
                    }
                }
            }
        }
        if monad.z == 0 {
            break;
        }
        println!("z = {:?} model number {}\n", monad.z, model_num);
        // model_num -= (monad.z as usize) + 1;
        model_num -= 1;
    }
    model_num
}

fn solve2(lines: &Vec<String>) -> u32 {
    0
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

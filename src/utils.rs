use crate::SOLNS;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::MAIN_SEPARATOR;

pub type Res<T> = Result<T, Box<dyn Error>>;

pub struct Args {
    pub input_path: String,
    pub problem: usize,
    pub subproblem: usize,
}

pub type SolnFn = fn(&Vec<String>) -> Res<i32>;
pub struct Soln {
    pub first: SolnFn,
    pub second: SolnFn,
}

#[allow(dead_code)]
pub fn dummy_soln(_input: &Vec<String>) -> Res<i32> {
    unimplemented!("Solution is not implemented");
}

pub fn parse_args() -> Option<Args> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        print_help(pname(&args[0])?);
        return None;
    }

    let problem = match args[1].parse::<i32>() {
        Ok(num) => {
            if num <= 0 || num as usize > SOLNS.len() {
                eprintln!("Only problems from 1 to {} have solutions!", SOLNS.len());
                return None;
            }
            Some(num as usize)
        }
        Err(_) => None,
    }?;

    if let Ok(num) = args[3].parse::<i32>() {
        if num <= 0 || num > 2 {
            eprintln!("Only sub-problems 1 or 2 can be selected!");
            return None;
        } else {
            return Some(Args {
                input_path: args[2].to_string(),
                problem: problem,
                subproblem: num as usize,
            });
        }
    }

    return None;
}

fn pname(callpath: &str) -> Option<&str> {
    callpath
        .split(MAIN_SEPARATOR)
        .collect::<Vec<_>>()
        .last()
        .map(|v| *v)
}

fn print_help(pname: &str) {
    eprintln!("Usage: {} [PROB #] [INPUT] [1|2]", pname);
}

pub fn read_file_lines(fname: &str) -> Res<Vec<String>> {
    let file = File::open(&fname)?;
    let mut lines = Vec::new();
    for line in io::BufReader::new(file).lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn split_columns_int(input: &Vec<String>) -> Res<(Vec<i32>, Vec<i32>)> {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in input.iter() {
        let nums = line
            .split(' ')
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| !x.trim().is_empty())
            .collect::<Vec<_>>();

        if let [f, s] = &nums[..] {
            first.push(f.parse::<i32>()?);
            second.push(s.parse::<i32>()?);
        } else {
            return Err("Failed to parse lines".into());
        }
    }

    Ok((first, second))
}

pub fn lines_to_ints(input: &Vec<String>) -> Res<Vec<Vec<i32>>> {
    let mut vecs = Vec::new();
    for line in input {
        let mut nvec = Vec::new();
        for num in line.split_whitespace() {
            nvec.push(num.parse::<i32>()?);
        }
        vecs.push(nvec);
    }
    Ok(vecs)
}

use utils::{Res, Soln};

mod utils {
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
}

mod day1 {
    use crate::utils;
    use utils::{dummy_soln, Res, Soln};

    pub fn ans(input: &Vec<String>) -> Res<i32> {
        let (mut first, mut second) = utils::split_columns_int(input).map_err(|e| {
            eprintln!("Failed to split columns: {e}");
            e
        })?;

        if first.len() != second.len() {
            return Err(format!(
                "Lists have varying lengths!: {} vs {}",
                first.len(),
                second.len()
            )
            .into());
        }

        first.sort();
        second.sort();
        let sum = first
            .into_iter()
            .zip(second.into_iter())
            .map(|(first, second)| (first - second).abs())
            .sum();

        Ok(sum)
    }

    pub const DAY1_SOLN: Soln = Soln {
        first: ans,
        second: dummy_soln,
    };
}

const SOLNS: &[Soln] = &[day1::DAY1_SOLN];

fn main() -> Res<()> {
    if let Some(args) = utils::parse_args() {
        let prob = args.problem;
        let input = utils::read_file_lines(&args.input_path)?;
        let subprob = args.subproblem;

        let soln = &SOLNS[prob - 1];
        let func = match subprob {
            1 => soln.first,
            2 => soln.second,
            _ => unreachable!(),
        };
        println!(
            "Answer to problem #{} part {}: {}",
            prob,
            subprob,
            func(&input)?
        );

        return Ok(());
    }

    std::process::exit(1);
}

use utils::Res;

mod utils {
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::MAIN_SEPARATOR;

    pub type Res<T> = Result<T, Box<dyn Error>>;

    pub struct Args {
        pub input_path: String,
        pub problem: i32,
    }

    pub fn parse_args() -> Option<Args> {
        let args: Vec<_> = std::env::args().collect();
        if args.len() != 3 {
            print_help(pname(&args[0])?);
            return None;
        }

        if let Ok(num) = args[2].parse::<i32>() {
            if num <= 0 || num > 2 {
                eprintln!("Only problems 1 or 2 can be selected!");
                return None;
            } else {
                return Some(Args {
                    input_path: args[1].to_string(),
                    problem: num,
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
        eprintln!("Usage: {} [INPUT] [1|2]", pname);
    }

    pub fn read_file_lines(fname: &str) -> Res<Vec<String>> {
        let file = File::open(&fname)?;
        let mut lines = Vec::new();
        for line in io::BufReader::new(file).lines() {
            lines.push(line?);
        }

        Ok(lines)
    }

    pub fn split_columns(input: &Vec<String>) -> Res<(Vec<i32>, Vec<i32>)> {
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

fn main() -> Res<()> {
    if let Some(args) = utils::parse_args() {
        let prob = args.problem;
        let input = utils::read_file_lines(&args.input_path)?;
        let (first, second) = utils::split_columns(&input)?;
        println!("Solving problem: {}", prob);
        println!("Sum of first: {}", first.iter().sum::<i32>());
        println!("Sum of second: {}", second.iter().sum::<i32>());
        return Ok(());
    }

    std::process::exit(1);
}

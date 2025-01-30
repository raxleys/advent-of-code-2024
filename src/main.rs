mod day01;
mod day02;
mod day03;
mod day04;
mod utils;
use utils::{Res, Soln};

const SOLNS: &[Soln] = &[
    day01::DAY1_SOLN,
    day02::DAY2_SOLN,
    day03::DAY3_SOLN,
    day04::DAY4_SOLN,
];

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

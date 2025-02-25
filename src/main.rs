mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day13;
mod day14;
mod day17;
mod utils;
use utils::{Res, Soln};

// Dummy
const DUMMY_SOLN: Soln = Soln {
    first: utils::dummy_soln,
    second: utils::dummy_soln,
};

const SOLNS: &[Soln] = &[
    day01::DAY01_SOLN,
    day02::DAY02_SOLN,
    day03::DAY03_SOLN,
    day04::DAY04_SOLN,
    day05::DAY05_SOLN,
    day06::DAY06_SOLN,
    // 7
    DUMMY_SOLN,
    // 8
    DUMMY_SOLN,
    // 9
    DUMMY_SOLN,
    // 10
    DUMMY_SOLN,
    // 11
    DUMMY_SOLN,
    // 12
    DUMMY_SOLN,
    // 13
    day13::DAY13_SOLN,
    day14::DAY14_SOLN,
    DUMMY_SOLN,
    DUMMY_SOLN,
    day17::DAY17_SOLN,
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

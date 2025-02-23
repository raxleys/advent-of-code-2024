use crate::utils;
use utils::{lines_to_ints, Res, Soln};

fn part1(input: &Vec<String>) -> Res<i32> {
    let nums = lines_to_ints(input)?;
    let safes = nums
        .into_iter()
        .map(|set| {
            let inc = (set[1] - set[0]) > 0;
            let is_nok = set
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, n)| n - set[i]) // enumerate still starts from 0
                .any(|diff| (diff > 0) != inc || diff == 0 || diff > 3 || diff < -3);

            if is_nok {
                0
            } else {
                1
            }
        })
        .sum();
    Ok(safes)
}

fn is_nok(nums: &Vec<i32>) -> bool {
    let inc = (nums[1] - nums[0]) > 0;
    nums.iter()
        .skip(1)
        .enumerate()
        .map(|(i, n)| n - nums[i])
        .any(|diff| (diff > 0) != inc || diff == 0 || diff > 3 || diff < -3)
}

#[derive(Debug)]
struct ReportData {
    report: Vec<i32>,
    problems: (usize, usize),
}

fn part2(input: &Vec<String>) -> Res<i32> {
    let nums = lines_to_ints(input)?;
    let n_reports = nums.len();
    let nok_nums: Vec<_> = nums.into_iter().filter(|nums| is_nok(&nums)).collect();

    let mut n_ok_reports = n_reports - nok_nums.len();

    let problem_info = nok_nums
        .into_iter()
        .filter_map(|report| {
            let diffs = report
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, n)| n - report[i])
                .collect::<Vec<_>>();

            let (pos, neg, zero) = diffs
                .iter()
                .scan((0, 0, 0), |(pos, neg, zero), &n| {
                    if n > 0 {
                        *pos += 1;
                    } else if n < 0 {
                        *neg += 1;
                    } else {
                        *zero += 1;
                    }
                    Some((*pos, *neg, *zero))
                })
                .last()
                .unwrap();

            if zero > 1 || (pos > 1 && neg > 1) {
                return None;
            }

            let is_inc = pos > neg;
            let mut problems = diffs
                .iter()
                .enumerate()
                .filter_map(|(i, &n)| {
                    if n == 0 || n.abs() > 3 || (is_inc && n < 0) || (!is_inc && n > 0) {
                        Some((i, i + 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if problems.is_empty() {
                None
            } else {
                Some(ReportData {
                    report,
                    problems: problems.pop().unwrap(),
                })
            }
        })
        .collect::<Vec<_>>();

    for data in problem_info {
        let &(f, s) = &data.problems;
        let mut rep1 = data.report.clone();
        let mut rep2 = data.report.clone();
        rep1.remove(f);
        rep2.remove(s);

        if !is_nok(&rep1) || !is_nok(&rep2) {
            n_ok_reports += 1;
        }
    }

    Ok(n_ok_reports as i32)
}

pub const DAY02_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

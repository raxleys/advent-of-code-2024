#![allow(dead_code)]
#![allow(unused_variables)]
use crate::utils::{self, Res, Soln};
use std::collections::HashSet;

#[derive(Debug)]
struct CraneData {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

fn get_int(s: &str) -> i32 {
    s.chars()
        .skip_while(|c| !c.is_digit(10))
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i32>()
        .expect("Failed to parse integer.")
}

fn parse_crane_data(input: &Vec<String>) -> Vec<CraneData> {
    input
        .chunks(4)
        .map(|chunk| {
            let parts: Vec<_> = chunk[0].split(' ').collect();
            let a_x = get_int(parts[2]);
            let a_y = get_int(parts[3]);

            let parts: Vec<_> = chunk[1].split(' ').collect();
            let b_x = get_int(parts[2]);
            let b_y = get_int(parts[3]);

            let parts: Vec<_> = chunk[2].split(' ').collect();
            let p_x = get_int(parts[1]);
            let p_y = get_int(parts[2]);

            CraneData {
                a: (a_x, a_y),
                b: (b_x, b_y),
                prize: (p_x, p_y),
            }
        })
        .collect()
}

fn solve_a_b(target: i32, x: i32, y: i32) -> HashSet<(i32, i32)> {
    let mut solns = HashSet::new();
    for b in 1..=(target / y + 1) {
        if (target - b * y) % x == 0 {
            let a = (target - b * y) / x;
            if a > 0 {
                solns.insert((a, b));
            }
        }
    }
    solns
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let data = parse_crane_data(&input);
    let mut solns = Vec::new();
    for crane_data in data {
        let xsolns = solve_a_b(crane_data.prize.0, crane_data.a.0, crane_data.b.0);
        let ysolns = solve_a_b(crane_data.prize.1, crane_data.a.1, crane_data.b.1);
        if let Some((x, y)) = xsolns
            .intersection(&ysolns)
            .cloned()
            .min_by_key(|(x, y)| x + y)
        {
            solns.push((x, y));
        }
    }

    let soln: i32 = solns.iter().map(|&(x, y)| 3 * x + y).sum();
    Ok(soln)
}

pub const DAY13_SOLN: Soln = Soln {
    first: part1,
    second: utils::dummy_soln,
};

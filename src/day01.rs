use crate::utils;
use std::collections::HashMap;
use utils::{Res, Soln};

fn part1(input: &Vec<String>) -> Res<i32> {
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

fn part2(input: &Vec<String>) -> Res<i32> {
    let (first, second) = utils::split_columns_int(input).map_err(|e| {
        eprintln!("Failed to split columns: {e}");
        e
    })?;

    let mut map = HashMap::new();
    for i in second {
        map.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }

    let ans: i32 = first
        .into_iter()
        .map(|i| i * map.get(&i).unwrap_or(&0))
        .sum();

    Ok(ans)
}

pub const DAY1_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

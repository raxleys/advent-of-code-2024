use crate::utils;
use std::collections::{HashMap, HashSet};
use utils::{Res, Soln};

fn get_orders_rules(input: &Vec<String>) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    // Map of num -> set or numbers that must come before num
    let mut rules = HashMap::new();
    let mut orders: Vec<Vec<i32>> = Vec::new();
    let mut push_rules = true;
    for line in input {
        if line.is_empty() {
            push_rules = false;
            continue;
        }

        if push_rules {
            let parts: Vec<_> = line
                .split('|')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            rules
                .entry(parts[0])
                .or_insert(HashSet::new())
                .insert(parts[1]);
        } else {
            orders.push(
                line.split(',')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }

    (rules, orders)
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let (rules, orders) = get_orders_rules(input);
    let mut seen_pages = HashSet::new();
    let mut ans = 0;
    for order in orders {
        let mut is_ok = true;
        let mid = order[order.len() / 2];
        for page in order {
            if let Some(after_pages) = rules.get(&page) {
                if seen_pages.intersection(&after_pages).count() > 0 {
                    is_ok = false;
                    break;
                }
            }

            seen_pages.insert(page);
        }

        if is_ok {
            ans += mid;
        }

        seen_pages.clear();
    }

    Ok(ans)
}

fn part2(input: &Vec<String>) -> Res<i32> {
    let (rules, orders) = get_orders_rules(input);
    let bad_orders: Vec<Vec<_>> = orders
        .into_iter()
        .filter_map(|order| {
            let mut seen_pages = HashSet::new();
            for page in &order {
                if let Some(after_pages) = rules.get(&page) {
                    if seen_pages.intersection(after_pages).count() > 0 {
                        return Some(order);
                    }
                }
                seen_pages.insert(*page);
            }
            return None;
        })
        .collect();

    let mut corrected_orders = Vec::new();
    for order in bad_orders {
        let mut corrected = Vec::new();
        for next in order {
            let mut inserted = false;
            for (i, n) in corrected.iter().enumerate() {
                if let Some(after_nums) = rules.get(&next) {
                    if after_nums.contains(n) {
                        corrected.insert(i, next);
                        inserted = true;
                        break;
                    }
                }
            }

            if !inserted {
                corrected.push(next);
            }
        }

        corrected_orders.push(corrected);
    }

    let ans: i32 = corrected_orders
        .into_iter()
        .map(|order| order[order.len() / 2])
        .sum();

    Ok(ans)
}

pub const DAY05_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

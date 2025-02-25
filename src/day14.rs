#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::utils::{self, Res, Soln};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn go(&mut self) {
        let mut dx = self.pos.0 + self.vel.0;
        let mut dy = self.pos.1 + self.vel.1;

        if dx < 0 {
            dx += WIDTH as i32;
        } else if dx as usize >= WIDTH {
            dx -= WIDTH as i32;
        }

        if dy < 0 {
            dy += HEIGHT as i32;
        } else if dy as usize >= HEIGHT {
            dy -= HEIGHT as i32;
        }

        self.pos = (dx, dy);
    }

    fn quad(&self) -> usize {
        if self.pos.0 as usize == WIDTH / 2 || self.pos.1 as usize == HEIGHT / 2 {
            return 4;
        }

        let left = (self.pos.0 as usize) < WIDTH / 2;
        let top = (self.pos.1 as usize) < HEIGHT / 2;
        // 0 | 1
        // -----
        // 3 | 2
        if left && top {
            return 0;
        } else if !left && top {
            return 1;
        } else if !left && !top {
            return 2;
        } else if left && !top {
            return 3;
        }

        unreachable!();
    }
}

fn parse_robot_data(input: &Vec<String>) -> Vec<Robot> {
    let mut data = Vec::new();
    for line in input {
        let parts: Vec<_> = line.split(' ').map(|s| &s[2..]).collect();

        let mut vals: Vec<_> = parts[0].split(',').map(|s| s.parse::<i32>()).collect();
        let py = vals.pop().unwrap().unwrap();
        let px = vals.pop().unwrap().unwrap();
        let mut vals: Vec<_> = parts[1].split(',').map(|s| s.parse::<i32>()).collect();
        let vy = vals.pop().unwrap().unwrap();
        let vx = vals.pop().unwrap().unwrap();

        data.push(Robot {
            pos: (px, py),
            vel: (vx, vy),
        });
    }

    data
}

#[allow(dead_code)]
fn draw_map(data: &Vec<Robot>) {
    let mut map = vec![vec![0; WIDTH]; HEIGHT];
    data.iter().for_each(|robot| {
        let x = robot.pos.0 as usize;
        let y = robot.pos.1 as usize;
        map[y][x] += 1;
    });

    for vec in map {
        for tile in vec {
            if tile == 0 {
                print!(".");
            } else {
                print!("{tile}");
            }
        }
        println!();
    }
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let mut robot_data = parse_robot_data(&input);
    for i in 0..100 {
        robot_data.iter_mut().for_each(|robot| robot.go());
    }

    // 5th 'quadrant' is dummy value for cleaner code.
    let mut quadrant_counts = [0; 5];
    robot_data
        .iter()
        .for_each(|robot| quadrant_counts[robot.quad()] += 1);

    let ans = quadrant_counts
        .iter()
        .take(4)
        .fold(1, |acc, count| acc * count);

    Ok(ans)
}

// Way too vague of a question. Thanks to this post:
// https://todd.ginsberg.com/post/advent-of-code/2024/day14/ for
// explaining that they found out that when there is no robot
// overlapping on any tiles, the christmas tree can be seen.
fn part2(input: &Vec<String>) -> Res<i32> {
    let mut robot_data = parse_robot_data(&input);
    let mut count = 0;
    loop {
        let mut map = vec![vec![0; WIDTH]; HEIGHT];
        let mut next = false;
        for robot in &robot_data {
            let x = robot.pos.0 as usize;
            let y = robot.pos.1 as usize;
            map[y][x] += 1;
            if map[y][x] > 1 {
                next = true;
                break;
            }
        }

        if next {
            robot_data.iter_mut().for_each(|robot| robot.go());
            count += 1;
            continue;
        }

        break;
    }

    draw_map(&robot_data);

    Ok(count)
}

pub const DAY14_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

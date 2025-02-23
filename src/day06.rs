#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use crate::utils;
use std::collections::{HashMap, HashSet};
use utils::{Res, Soln};
use Dir::*;

#[derive(Debug, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    dir: Dir,
}

impl Pos {
    fn new(x: i32, y: i32, dir: Dir) -> Self {
        Self { x, y, dir }
    }
}

type Board = Vec<Vec<char>>;

fn to_board(input: &Vec<String>) -> Board {
    input
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_player_pos(board: &Board) -> Pos {
    for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            match board[i][j] {
                '.' | '#' => continue,
                '^' => return Pos::new(j as i32, i as i32, Up),
                _ => unreachable!("Unexpected character"),
            };
        }
    }
    panic!("Didn't find the player location.");
}

fn next_pos(board: &Board, ppos: &Pos) -> Option<Pos> {
    let ox = ppos.x;
    let oy = ppos.y;
    let (x, y) = match &ppos.dir {
        Up => (ox, oy - 1),
        Down => (ox, oy + 1),
        Left => (ox - 1, oy),
        Right => (ox + 1, oy),
    };

    if x < 0 || y < 0 {
        return None;
    }

    if x as usize >= board[0].len() || y as usize >= board.len() {
        None
    } else {
        Some(Pos::new(x, y, ppos.dir.clone()))
    }
}

fn tile(board: &Board, pos: &Pos) -> char {
    board[pos.y as usize][pos.x as usize]
}

fn rotate(pos: Pos) -> Pos {
    let ndir = match &pos.dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    };

    Pos::new(pos.x, pos.y, ndir)
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let board = to_board(input);
    let mut pos = get_player_pos(&board);
    let mut positions = HashSet::new();
    loop {
        positions.insert((pos.x, pos.y));
        if let Some(new_pos) = next_pos(&board, &pos) {
            match tile(&board, &new_pos) {
                '.' | '^' => pos = new_pos,
                '#' => pos = rotate(pos),
                _ => panic!("Unexpected character"),
            }
        } else {
            break;
        }
    }

    Ok(positions.len() as i32)
}

fn part2(input: &Vec<String>) -> Res<i32> {
    todo!()
}

pub const DAY06_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

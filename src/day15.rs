use crate::utils::{Res, Soln};

struct Map {
    board: Vec<Vec<char>>,
    pos: (i32, i32),
}

impl Map {
    fn new(input: Vec<String>) -> Self {
        let board: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
        let mut pos = (-1, -1);
        for (y, row) in board.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if board[y][x] == '@' {
                    pos = (y as i32, x as i32);
                    break;
                }
            }
        }

        if pos.0 < 0 || pos.1 < 1 {
            panic!("Couldn't find robot location.");
        }

        Self { board, pos }
    }

    fn act(&mut self, action: char) {
        let (y, x) = self.pos;
        let (dy, dx) = self.delta_pos(action);
        let mut i = 1;
        loop {
            if let Some(next_char) = self.char_at(y + dy * i, x + dx * i) {
                match next_char {
                    '#' => return,
                    '.' => {
                        if i > 1 {
                            // Only encountered 'O' until now
                            // Swap pos of first 'O' with this '.'
                            self.swap((y + dy, x + dx), (y + dy * i, x + dx * i));
                        }

                        // Move
                        self.update_pos(y + dy, x + dx);
                        return;
                    }
                    'O' => i += 1,
                    _ => panic!("{}", format!("Unexpected character: {}", next_char)),
                }
            } else {
                panic!("Out of bounds!");
            }
        }
    }

    fn in_bounds(&self, y: i32, x: i32) -> bool {
        if y < 0 || x < 0 {
            return false;
        }

        if y as usize >= self.board.len() || x as usize >= self.board[0].len() {
            return false;
        }

        return true;
    }

    fn char_at(&self, y: i32, x: i32) -> Option<char> {
        if !self.in_bounds(y, x) {
            None
        } else {
            Some(self.board[y as usize][x as usize])
        }
    }

    fn delta_pos(&self, dir: char) -> (i32, i32) {
        match dir {
            '^' => (-1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            _ => panic!("{}", format!("Unexpected character: {dir}")),
        }
    }

    fn swap(&mut self, from: (i32, i32), to: (i32, i32)) {
        let y = from.0 as usize;
        let x = from.1 as usize;
        let yp = to.0 as usize;
        let xp = to.1 as usize;

        let tmp = self.board[yp][xp];
        self.board[yp][xp] = self.board[y][x];
        self.board[y][x] = tmp;
    }

    fn update_pos(&mut self, yp: i32, xp: i32) {
        self.swap(self.pos, (yp, xp));
        self.pos = (yp, xp);
    }

    fn sum(&self) -> i32 {
        let mut sum = 0;
        for (y, row) in self.board.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if self.char_at(y as i32, x as i32).unwrap() == 'O' {
                    sum += 100 * y + x;
                }
            }
        }

        sum as i32
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.board {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

struct ActionSeq {
    seq: String,
    i: usize,
}

impl ActionSeq {
    fn new(input: Vec<String>) -> Self {
        Self {
            seq: input.join(""),
            i: 0,
        }
    }
}

impl Iterator for ActionSeq {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.seq.len() {
            let c = self.seq[self.i..].chars().next().unwrap();
            self.i += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }
}

fn parse(input: &Vec<String>) -> (Map, ActionSeq) {
    let mut map_lines = Vec::new();
    let mut action_lines = Vec::new();
    let mut map = true;
    for line in input {
        if line.trim().is_empty() {
            map = false;
            continue;
        }

        if map {
            map_lines.push(line.trim().to_string());
        } else {
            action_lines.push(line.trim().to_string());
        }
    }

    (Map::new(map_lines), ActionSeq::new(action_lines))
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let (mut map, actions) = parse(input);
    actions.for_each(|action| map.act(action));
    let ans = map.sum();
    Ok(ans)
}

fn part2(input: &Vec<String>) -> Res<i32> {
    input.iter().for_each(|line| println!("{line}"));
    todo!()
}

pub const DAY15_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

use crate::utils;
use utils::{Res, Soln};

#[derive(Debug)]
enum Tok {
    Mul,
    Do,
    Dont,
}

struct Parser {
    src: Vec<char>,
    ptr: usize,
    do_flag: bool,
}

impl Parser {
    pub fn new(src: String) -> Self {
        Self {
            src: src.chars().collect(),
            ptr: 0,
            do_flag: true,
        }
    }

    fn get_src(&self) -> &[char] {
        &self.src[self.ptr..]
    }

    fn drop_to(&mut self, ch: char) {
        self.ptr += self.get_src().iter().take_while(|&&c| c != ch).count();
    }

    fn try_consume(&mut self, tok: &str) -> bool {
        let n_same = self
            .get_src()
            .iter()
            .zip(tok.chars())
            .take_while(|(c1, c2)| *c1 == c2)
            .count();

        if n_same == tok.len() {
            self.ptr += n_same;
            true
        } else {
            false
        }
    }

    fn try_consume_int(&mut self) -> Option<i32> {
        let num: String = self
            .get_src()
            .iter()
            .take_while(|ch| ch.is_digit(10))
            .collect();

        if num.len() > 0 && num.len() <= 3 {
            self.ptr += num.len();
            Some(num.parse().unwrap())
        } else {
            None
        }
    }

    pub fn parse_one(&mut self) -> Vec<(i32, i32)> {
        let mut pairs = Vec::new();
        while self.ptr < self.src.len() {
            self.drop_to('m');
            if !self.try_consume("mul(") {
                self.ptr += 1;
                continue;
            }

            let x = self.try_consume_int();
            if x.is_none() {
                self.ptr += 1;
                continue;
            }

            if !self.try_consume(",") {
                self.ptr += 1;
                continue;
            }

            let y = self.try_consume_int();
            if y.is_none() {
                self.ptr += 1;
                continue;
            }

            if !self.try_consume(")") {
                self.ptr += 1;
                continue;
            }

            pairs.push((x.unwrap(), y.unwrap()));
        }

        pairs
    }

    fn next_tok(&mut self) -> Option<Tok> {
        self.ptr += self
            .get_src()
            .iter()
            .take_while(|&&c| c != 'm' && c != 'd')
            .count();

        if self.try_consume("do()") {
            Some(Tok::Do)
        } else if self.try_consume("don't()") {
            Some(Tok::Dont)
        } else if self.try_consume("mul(") {
            Some(Tok::Mul)
        } else {
            None
        }
    }

    pub fn parse_two(&mut self) -> Vec<(i32, i32)> {
        let mut pairs = Vec::new();
        while self.ptr < self.src.len() {
            let tok = self.next_tok();

            if let Some(tok) = tok {
                match tok {
                    Tok::Do => self.do_flag = true,
                    Tok::Dont => self.do_flag = false,
                    Tok::Mul => {
                        let x = self.try_consume_int();
                        if x.is_none() {
                            self.ptr += 1;
                            continue;
                        }

                        if !self.try_consume(",") {
                            self.ptr += 1;
                            continue;
                        }

                        let y = self.try_consume_int();
                        if y.is_none() {
                            self.ptr += 1;
                            continue;
                        }

                        if !self.try_consume(")") {
                            self.ptr += 1;
                            continue;
                        }

                        if self.do_flag {
                            pairs.push((x.unwrap(), y.unwrap()));
                        }
                    }
                }
            } else {
                self.ptr += 1;
            }
        }
        pairs
    }

    #[allow(dead_code)]
    fn print_src(&self) {
        let src_str: String = self.get_src().iter().collect();
        println!("{src_str}");
    }

    pub fn solve_one(&mut self) -> i32 {
        self.parse_one().iter().map(|(x, y)| x * y).sum()
    }

    pub fn solve_two(&mut self) -> i32 {
        self.parse_two().iter().map(|(x, y)| x * y).sum()
    }
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let src = input.join("\n");
    let mut parser = Parser::new(src);
    let ans = parser.solve_one();
    Ok(ans)
}

fn part2(input: &Vec<String>) -> Res<i32> {
    let src = input.join("\n");
    let mut parser = Parser::new(src);
    let ans = parser.solve_two();
    Ok(ans)
}

pub const DAY3_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_simple() {
        let input = String::from("mul(10,2)");
        let mut parser = Parser::new(input);
        let ans = parser.solve_one();
        assert_eq!(ans, 20);
    }

    #[test]
    fn test_parser_simple_fail() {
        let input = String::from("mul(10,)");
        let mut parser = Parser::new(input);
        let ans = parser.solve_one();
        assert_eq!(ans, 0);

        let input = String::from("mul(10, 2)");
        let mut parser = Parser::new(input);
        let ans = parser.solve_one();
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_parser_mult() {
        let input = String::from("xxxmul(10,2)xxxmul(5,2)");
        let mut parser = Parser::new(input);
        let ans = parser.solve_one();
        assert_eq!(ans, (10 * 2) + (5 * 2));
    }
}

use crate::utils::{self, Res, Soln};

#[derive(Debug)]
struct Program {
    reg: Vec<u32>,
    prog: Vec<u32>,
    ip: usize,
    stdout_buf: Vec<u32>,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            reg: self.reg.clone(),
            prog: self.prog.clone(),
            ip: self.ip,
            stdout_buf: self.stdout_buf.clone(),
        }
    }
}

impl Program {
    fn new(reg: Vec<u32>, prog: Vec<u32>) -> Self {
        Self {
            reg,
            prog,
            ip: 0,
            stdout_buf: Vec::new(),
        }
    }

    fn run(&mut self) -> String {
        while !self.done() {
            self.next();
        }

        return self.stdout();
    }

    fn done(&self) -> bool {
        self.ip >= self.prog.len()
    }

    fn next(&mut self) {
        if !self.done() {
            let next_op = self.prog[self.ip];
            let next_oper = self.prog[self.ip + 1];
            self.ip += 2;
            self.exec(next_op, next_oper)
        }
    }

    fn exec(&mut self, opcode: u32, operand: u32) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Unexpected opcode"),
        };
    }

    fn combo_op(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4..=6 => self.reg[operand as usize - 4],
            _ => panic!("adv: Unexpected operand"),
        }
    }

    fn adv(&mut self, operand: u32) {
        let num = self.reg[0];
        let denum = self.combo_op(operand);
        let base: u32 = 2;
        self.reg[0] = num / base.pow(denum as u32);
    }

    fn bxl(&mut self, operand: u32) {
        let reg = self.reg[1];
        self.reg[1] = reg ^ operand;
    }

    fn bst(&mut self, operand: u32) {
        let num = self.combo_op(operand);
        self.reg[1] = num % 8;
    }

    fn jnz(&mut self, operand: u32) {
        if self.reg[0] != 0 {
            self.ip = operand as usize;
        }
    }

    #[allow(unused_variables)]
    fn bxc(&mut self, operand: u32) {
        self.reg[1] = self.reg[1] ^ self.reg[2];
    }

    fn out(&mut self, operand: u32) {
        let num = self.combo_op(operand);
        self.stdout_buf.push(num % 8);
    }

    fn bdv(&mut self, operand: u32) {
        let num = self.reg[0];
        let denum = self.combo_op(operand);
        let base: u32 = 2;
        self.reg[1] = num / base.pow(denum as u32);
    }

    fn cdv(&mut self, operand: u32) {
        let num = self.reg[0];
        let denum = self.combo_op(operand);
        let base: u32 = 2;
        self.reg[2] = num / base.pow(denum as u32);
    }

    fn stdout(&self) -> String {
        self.stdout_buf
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn parse_program(input: &Vec<String>) -> Program {
    let mut reg_vals: Vec<u32> = Vec::with_capacity(3);
    for line in input.iter().take(3) {
        let val = line
            .split(':')
            .skip(1)
            .filter_map(|part| part.trim().parse::<u32>().ok())
            .next()
            .expect("Expected register value");
        reg_vals.push(val);
    }

    let program: String = input
        .into_iter()
        .skip(4)
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .collect();

    let mut prog_vals = Vec::new();
    for num in program.split(',') {
        prog_vals.push(num.parse::<u32>().unwrap());
    }

    return Program::new(reg_vals, prog_vals);
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let mut program = parse_program(&input);
    println!("Output: {}\n", program.run());
    Ok(-1)
}

pub const DAY17_SOLN: Soln = Soln {
    first: part1,
    second: utils::dummy_soln,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() {
        let mut program = Program::new(vec![16, 2, 0], vec![0, 2]);
        program.next();
        assert_eq!(program.reg[0], 4);

        let mut program = Program::new(vec![16, 2, 0], vec![0, 5]);
        program.next();
        assert_eq!(program.reg[0], 4);
    }

    #[test]
    #[should_panic(expected = "adv: Unexpected operand")]
    fn test_adv2() {
        let mut program = Program::new(vec![0, 0, 0], vec![0, 7]);
        program.next();
    }

    #[test]
    fn test_bxl() {
        let mut program = Program::new(vec![0, 0, 0], vec![1, 0]);
        program.next();
        assert_eq!(program.reg[1], 0b0);

        let mut program = Program::new(vec![0, 0, 0], vec![1, 1]);
        program.next();
        assert_eq!(program.reg[1], 0b1);

        let mut program = Program::new(vec![0, 0b0101, 0], vec![1, 0b1010]);
        program.next();
        assert_eq!(program.reg[1], 0b1111);
    }

    #[test]
    fn test_bst() {
        let mut program = Program::new(vec![0, 0xBEEF, 0], vec![2, 0]);
        program.next();
        assert_eq!(program.reg[1], 0);

        let mut program = Program::new(vec![0, 0xBEEF, 0], vec![2, 3]);
        program.next();
        assert_eq!(program.reg[1], 3);

        let mut program = Program::new(vec![0, 63, 0], vec![2, 5]);
        program.next();
        assert_eq!(program.reg[1], 7);

        let mut program = Program::new(vec![5, 0xBEEF, 0], vec![2, 4]);
        program.next();
        assert_eq!(program.reg[1], 5);
    }

    #[test]
    fn test_jnz() {
        let mut program = Program::new(vec![0, 0xBEEF, 0xDEAD], vec![3, 0, 2, 5, 2, 5]);
        program.next();
        assert_eq!(program.ip, 2);

        let mut program = Program::new(vec![1, 0xBEEF, 0xDEAD], vec![3, 4, 2, 5, 2, 5]);
        program.next();
        assert_eq!(program.ip, 4);
    }

    #[test]
    fn test_bxc() {
        let mut program = Program::new(vec![0, 0b0, 0b0], vec![4, 0]);
        program.next();
        assert_eq!(program.reg[1], 0b0);

        let mut program = Program::new(vec![0, 0b1111, 0b0], vec![4, 4]);
        program.next();
        assert_eq!(program.reg[1], 0b1111);

        let mut program = Program::new(vec![0, 0b0101, 0b1010], vec![4, 4]);
        program.next();
        assert_eq!(program.reg[1], 0b1111);
    }

    #[test]
    fn test_out() {
        let mut program = Program::new(vec![0xDEAD, 0xBEEF, 0xDEED], vec![5, 0]);
        program.next();
        assert_eq!(program.reg, vec![0xDEAD, 0xBEEF, 0xDEED]);
        assert_eq!(program.stdout(), "0");

        let mut program = Program::new(vec![0xDEAD, 0xBEEF, 8], vec![5, 0, 5, 1, 5, 2, 5, 6]);
        for _ in 0..=4 {
            program.next();
        }
        assert_eq!(program.reg, vec![0xDEAD, 0xBEEF, 8]);
        assert_eq!(program.stdout(), "0,1,2,0");
    }

    #[test]
    fn test_bdv() {
        let mut program = Program::new(vec![16, 16, 0], vec![6, 2]);
        program.next();
        assert_eq!(program.reg[0], 16);
        assert_eq!(program.reg[1], 4);

        let mut program = Program::new(vec![16, 2, 0], vec![6, 5]);
        program.next();
        assert_eq!(program.reg[0], 16);
        assert_eq!(program.reg[1], 4);
    }

    #[test]
    fn test_cdv() {
        let mut program = Program::new(vec![16, 0, 16], vec![7, 2]);
        program.next();
        assert_eq!(program.reg[0], 16);
        assert_eq!(program.reg[2], 4);

        let mut program = Program::new(vec![16, 2, 16], vec![7, 5]);
        program.next();
        assert_eq!(program.reg[0], 16);
        assert_eq!(program.reg[2], 4);
    }
}

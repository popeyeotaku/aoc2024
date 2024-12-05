use std::fs;

use op::interp;
use parser::Parser;

pub fn day03() {
    let input = fs::read_to_string("day03_input.txt").unwrap();
    let sum: u64 = Parser::new(&input)
        .mul_parser()
        .map(|(l, r)| (l as u64) * (r as u64))
        .sum();
    println!("sum of muls: {}", sum);
    let sum = interp(Parser::new(&input).op_parser());
    println!("sum of ops: {}", sum);
}

mod op {
    pub type Num = u16;

    #[derive(Clone, Copy)]
    pub enum Op {
        Mul(Num, Num),
        Do,
        Dont,
    }

    pub fn interp<I>(ops: I) -> u64
    where
        I: Iterator<Item = Op>,
    {
        let mut flag = true;
        let mut sum: u64 = 0;
        for op in ops {
            match op {
                Op::Mul(left, right) => {
                    if flag {
                        sum += ((left as u32) * (right as u32)) as u64;
                    }
                }
                Op::Do => flag = true,
                Op::Dont => flag = false,
            }
        }
        sum
    }
}

mod parser {
    //! program = mul*
    //! mul = "mul(" num ',' num ')
    //! num = digit digit? digit?

    use std::{iter::Peekable, str::Chars};

    use super::op::{Num, Op};

    pub struct Parser<'a> {
        chars: Peekable<Chars<'a>>,
    }

    enum WhichOp {
        Mul,
        Do,
        Dont,
    }

    impl<'a> Parser<'a> {
        pub fn new(s: &'a str) -> Self {
            Self {
                chars: s.chars().peekable(),
            }
        }

        fn peek(&mut self) -> Option<char> {
            self.chars.peek().copied()
        }

        fn advance(&mut self) {
            self.chars.next();
        }

        fn eof(&mut self) -> bool {
            self.peek().is_none()
        }

        fn cmatch<F>(&mut self, f: F) -> Option<char>
        where
            F: FnOnce(char) -> bool,
        {
            if let Some(c) = self.peek() {
                if f(c) {
                    self.advance();
                    return Some(c);
                }
            }
            None
        }

        fn smatch(&mut self, s: &str) -> bool {
            for c in s.chars() {
                if self.cmatch(|peeked| c == peeked).is_none() {
                    return false;
                }
            }
            true
        }

        fn digit(&mut self) -> Option<u32> {
            self.cmatch(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
        }

        fn num(&mut self) -> Option<Num> {
            let mut i = self.digit()? as Num;
            for _ in 0..2 {
                if let Some(digit) = self.digit() {
                    i = i * 10 + (digit as Num);
                }
            }
            if self.digit().is_some() {
                None
            } else {
                Some(i)
            }
        }

        fn try_mul(&mut self) -> Option<(Num, Num)> {
            if self.smatch("mul(") {
                let left = self.num()?;
                self.cmatch(|c| c == ',')?;
                let right = self.num()?;
                self.cmatch(|c| c == ')')?;
                Some((left, right))
            } else {
                None
            }
        }

        fn which_op(&mut self) -> Option<WhichOp> {
            match self.peek()? {
                'm' => {
                    if self.smatch("mul(") {
                        return Some(WhichOp::Mul);
                    }
                }
                'd' => {
                    if self.smatch("do") {
                        if self.cmatch(|c| c == '(').is_some() {
                            if self.cmatch(|c| c == ')').is_some() {
                                return Some(WhichOp::Do);
                            }
                        } else if self.smatch("n't()") {
                            return Some(WhichOp::Dont);
                        }
                    }
                }
                _ => (),
            }
            None
        }

        pub fn mul_parser(self) -> MulParser<'a> {
            MulParser { p: self }
        }

        pub fn op_parser(self) -> OpParser<'a> {
            OpParser { p: self }
        }

        fn try_op(&mut self) -> Option<Op> {
            match self.which_op()? {
                WhichOp::Mul => {
                    let left = self.num()?;
                    self.cmatch(|c| c == ',')?;
                    let right = self.num()?;
                    self.cmatch(|c| c == ')')?;
                    Some(Op::Mul(left, right))
                }
                WhichOp::Do => Some(Op::Do),
                WhichOp::Dont => Some(Op::Dont),
            }
        }
    }

    pub struct MulParser<'a> {
        p: Parser<'a>,
    }

    impl Iterator for MulParser<'_> {
        type Item = (Num, Num);

        fn next(&mut self) -> Option<Self::Item> {
            while !self.p.eof() {
                if let Some(pair) = self.p.try_mul() {
                    return Some(pair);
                }
                self.p.advance();
            }
            None
        }
    }

    pub struct OpParser<'a> {
        p: Parser<'a>,
    }

    impl Iterator for OpParser<'_> {
        type Item = Op;

        fn next(&mut self) -> Option<Self::Item> {
            while !self.p.eof() {
                if let Some(op) = self.p.try_op() {
                    return Some(op);
                }
                self.p.advance();
            }
            None
        }
    }
}

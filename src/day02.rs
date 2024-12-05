use std::fs;

pub fn day02() {
    let input = fs::read_to_string("day02_input.txt").unwrap();
    let lines = parse::parse(&input);

    let mut safes: u32 = 0;
    for line in &lines {
        if analyze(line.as_slice()) {
            safes += 1;
        }
    }
    println!("safe reports: {}", safes);

    let mut safes: u32 = 0;
    for line in &lines {
        if find_safe(line.as_slice()) {
            safes += 1;
        }
    }
    println!("safes with toleration: {}", safes);
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Inc,
    Dec,
    None,
}

impl Direction {
    pub fn dir(prev: Num, next: &Num) -> Self {
        match prev.cmp(next) {
            std::cmp::Ordering::Less => Self::Inc,
            std::cmp::Ordering::Greater => Self::Dec,
            std::cmp::Ordering::Equal => Self::None,
        }
    }
}

fn analyze(line: &[Num]) -> bool {
    let mut prev = line[0];
    let dir: Direction = Direction::dir(prev, &line[1]);
    if dir == Direction::None {
        return false;
    }
    for next in &line[1..] {
        if Direction::dir(prev, next) != dir {
            return false;
        }
        match prev.abs_diff(*next) {
            1..=3 => (),
            _ => return false,
        }
        prev = *next;
    }
    true
}

fn find_safe(line: &[Num]) -> bool {
    if analyze(line) {
        true
    } else {
        for i in 0..line.len() {
            let mut line = Vec::from_iter(line.iter().cloned());
            line.remove(i);
            if analyze(&line) {
                return true;
            }
        }
        false
    }
}

type Num = u8;

mod parse {
    use std::{iter::Peekable, str::Chars};

    use super::Num;

    enum Token {
        Num(Num),
        Newline,
    }

    impl Token {
        pub fn num(self) -> Num {
            if let Self::Num(i) = self {
                i
            } else {
                panic!("expected number")
            }
        }

        pub fn newline(self) {
            if let Self::Newline = self {
            } else {
                panic!("expected newline")
            }
        }
    }

    struct Tokenizer<'a> {
        chars: Peekable<Chars<'a>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(s: &'a str) -> Self {
            Self {
                chars: s.chars().peekable(),
            }
        }

        fn num(&mut self) -> Num {
            let mut i: Num = 0;

            while let Some(c) = self.chars.peek() {
                if let Some(digit) = c.to_digit(10) {
                    self.chars.next().unwrap();
                    i = i * 10 + (digit as Num);
                } else {
                    break;
                }
            }

            i
        }
    }

    impl Iterator for Tokenizer<'_> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(c) = self.chars.peek() {
                if *c == '\n' {
                    self.chars.next().unwrap();
                    return Some(Token::Newline);
                } else if c.is_ascii_digit() {
                    return Some(Token::Num(self.num()));
                } else if c.is_whitespace() {
                    self.chars.next().unwrap();
                } else {
                    panic!("unexpected character {}", *c);
                }
            }
            None
        }
    }

    fn line(t: &mut Peekable<Tokenizer<'_>>) -> Vec<Num> {
        let mut nums: Vec<Num> = vec![t.next().unwrap().num()];
        while let Some(Token::Num(i)) = t.peek() {
            nums.push(*i);
            t.next().unwrap();
        }
        t.next().unwrap().newline();
        nums
    }

    fn lines(t: &mut Peekable<Tokenizer<'_>>) -> Vec<Vec<Num>> {
        let mut lines: Vec<Vec<Num>> = vec![line(t)];
        while let Some(&Token::Num(_)) = t.peek() {
            lines.push(line(t));
        }
        lines
    }

    fn trailer(t: &mut Peekable<Tokenizer<'_>>) {
        for t in t.by_ref() {
            t.newline();
        }
    }

    pub fn parse(s: &str) -> Vec<Vec<Num>> {
        let mut t = Tokenizer::new(s).peekable();
        let l = lines(&mut t);
        trailer(&mut t);
        l
    }
}

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use befores::calc_befores;
use parse::parse;
use update::{fix_order, is_good, middle_page};

pub fn day05() {
    let input = fs::read_to_string("day05_input.txt").unwrap();
    let (befores, updates) = parse(&input);
    let before_map = calc_befores(befores);
    let sum: u32 = updates
        .iter()
        .filter(|u| is_good(u, &before_map))
        .map(|u| middle_page(u) as u32)
        .sum();
    println!("sum of good middles: {}", sum);
    let sum: u32 = updates
        .iter()
        .filter(|u| !is_good(u, &before_map))
        .map(|u| fix_order(u, &before_map))
        .map(|u| middle_page(&u) as u32)
        .sum();
    println!("sum of fixed middles: {}", sum);
}

type Page = u8;
type Before = (Page, Page);
type BeforeMap = HashMap<Page, HashSet<Page>>;
type Update = Vec<Page>;

mod befores {
    use std::collections::{HashMap, HashSet};

    use super::{Before, BeforeMap};

    pub fn calc_befores(befores: Vec<Before>) -> BeforeMap {
        let mut map: BeforeMap = HashMap::new();
        for (key, before) in befores {
            map.entry(before).or_default();
            if let Some(befores) = map.get_mut(&key) {
                befores.insert(before);
            } else {
                map.insert(key, HashSet::from([before]));
            }
        }

        map
    }
}

mod update {

    use super::{BeforeMap, Page, Update};

    pub fn fix_order(update: &Update, before_map: &BeforeMap) -> Update {
        let mut update = update.clone();
        let mut swapped = true;
        while swapped {
            swapped = false;

            'l: for i in 1..update.len() {
                let key = update[i];
                let befores = before_map.get(&key).unwrap();
                for j in 0..i {
                    if befores.contains(&update[j]) {
                        update.swap(i, j);
                        swapped = true;
                        break 'l;
                    }
                }
            }
        }
        update
    }

    pub fn is_good(update: &Update, befores: &BeforeMap) -> bool {
        for i in 0..update.len() {
            let befores = befores.get(&update[i]).unwrap();
            for before in &update[0..i] {
                if befores.contains(before) {
                    return false;
                }
            }
        }
        true
    }

    pub fn middle_page(update: &Update) -> Page {
        update[update.len() / 2]
    }
}

mod parse {
    //! data = before+ '\n' update+ trailer
    //! before = num '|' num '\n'
    //! update = num (',' num)* '\n'
    //! trailer = '\n'*

    use std::{iter::Peekable, str::Chars};

    use super::{Before, Page, Update};

    pub fn parse(s: &str) -> (Vec<Before>, Vec<Update>) {
        let mut t: Peekable<Tokenizer<'_>> = Tokenizer::new(s).peekable();
        let mut befores: Vec<Before> = vec![before(&mut t)];
        while t.peek().unwrap() != &Token::NewLine {
            befores.push(before(&mut t));
        }
        t.next().unwrap();
        let mut updates: Vec<Update> = vec![update(&mut t)];
        while let Some(Token::Num(_)) = t.peek() {
            updates.push(update(&mut t));
        }
        trailer(&mut t);
        assert!(t.peek().is_none());
        (befores, updates)
    }

    fn before(t: &mut Peekable<Tokenizer<'_>>) -> Before {
        let left = t.next().unwrap().num();
        assert_eq!(t.next().unwrap(), Token::Or);
        let right = t.next().unwrap().num();
        assert_eq!(t.next().unwrap(), Token::NewLine);
        (left, right)
    }

    fn update(t: &mut Peekable<Tokenizer<'_>>) -> Update {
        let mut nums: Vec<Page> = vec![t.next().unwrap().num()];
        while let Some(&Token::Comma) = t.peek() {
            t.next().unwrap();
            nums.push(t.next().unwrap().num());
        }
        assert_eq!(t.next().unwrap(), Token::NewLine);
        nums
    }

    fn trailer(t: &mut Peekable<Tokenizer<'_>>) {
        for token in t {
            assert_eq!(token, Token::NewLine);
        }
    }

    struct Tokenizer<'a> {
        chars: Peekable<Chars<'a>>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Token {
        Num(Page),
        Or,
        Comma,
        NewLine,
    }

    impl Token {
        pub fn num(self) -> Page {
            if let Self::Num(i) = self {
                i
            } else {
                panic!("expected a number")
            }
        }
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(s: &'a str) -> Self {
            Self {
                chars: s.chars().peekable(),
            }
        }

        fn num(&mut self) -> Page {
            let mut i: Page = 0;
            while let Some(c) = self.chars.peek() {
                if let Some(digit) = c.to_digit(10) {
                    self.chars.next().unwrap();
                    i = i * 10 + (digit as Page);
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
                    return Some(Token::NewLine);
                } else if *c == '|' {
                    self.chars.next().unwrap();
                    return Some(Token::Or);
                } else if *c == ',' {
                    self.chars.next().unwrap();
                    return Some(Token::Comma);
                } else if c.is_ascii_digit() {
                    return Some(Token::Num(self.num()));
                } else if c.is_whitespace() {
                    self.chars.next().unwrap();
                } else {
                    panic!("unexpected character '{}'", c)
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::update::{fix_order, is_good, middle_page};

    use super::{befores::calc_befores, parse::parse};

    #[test]
    fn test_data() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        let (befores, updates) = parse(input);
        let before_map = calc_befores(befores);
        let sum: u32 = updates
            .iter()
            .filter(|u| is_good(u, &before_map))
            .map(|u| middle_page(u) as u32)
            .sum();
        assert_eq!(sum, 143);

        let sum: u32 = updates
            .iter()
            .filter(|u| !is_good(u, &before_map))
            .map(|u| fix_order(u, &before_map))
            .map(|u| middle_page(&u) as u32)
            .sum();
        assert_eq!(sum, 123);
    }
}

use lazy_static::lazy_static;
use regex::Regex;
use rustvent2022::get_input;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum MyError {
    Generic(&'static str),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Generic(s) => write!(f, "{}", s),
        }
    }
}

impl Error for MyError {}

#[derive(Clone)]
enum Operation {
    Add,
    Mult,
}

#[derive(Clone)]
enum Term {
    Old,
    Num(isize),
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<isize>,
    test: isize,
    to_true: usize,
    to_false: usize,
    op: Operation,
    t1: Term,
    t2: Term,
    inspected: usize,
}

impl Monkey {
    fn inspect_part_two(&mut self, scm: isize) -> Option<(isize, usize)> {
        if let Some(wl) = self.items.pop_front() {
            self.inspected += 1;

            let t1 = match self.t1 {
                Term::Old => wl,
                Term::Num(n) => n,
            };
            let t2 = match self.t2 {
                Term::Old => wl,
                Term::Num(n) => n,
            };
            let new_wl = match self.op {
                Operation::Mult => t1 * t2,
                Operation::Add => t1 + t2,
            } % scm;
            Some(match new_wl % self.test == 0 {
                true => (new_wl, self.to_true),
                false => (new_wl, self.to_false),
            })
        } else {
            None
        }
    }
    fn inspect(&mut self) -> Option<(isize, usize)> {
        if let Some(wl) = self.items.pop_front() {
            self.inspected += 1;

            let t1 = match self.t1 {
                Term::Old => wl,
                Term::Num(n) => n,
            };
            let t2 = match self.t2 {
                Term::Old => wl,
                Term::Num(n) => n,
            };
            let new_wl = match self.op {
                Operation::Mult => t1 * t2,
                Operation::Add => t1 + t2,
            } / 3;
            Some(match new_wl % self.test == 0 {
                true => (new_wl, self.to_true),
                false => (new_wl, self.to_false),
            })
        } else {
            None
        }
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {static ref RE: Regex = Regex::new(
            r"Monkey (?P<monkey_num>\d+):\n  Starting items: (?P<items>(?:\d+, )*\d*)\n  Operation: new = (?P<vfirst>old|\d+) (?P<op>\*|\+) (?P<vsecond>old|\d+)\n  Test: divisible by (?P<divby>\d+)\n    If true: throw to monkey (?P<truemonkey>\d+)\n    If false: throw to monkey (?P<falsemonkey>\d+)",
        ).unwrap();}

        let cap = RE.captures(s).ok_or("No captures found")?;
        let mut items = VecDeque::new();
        for i in cap
            .name("items")
            .ok_or("no cap for items")?
            .as_str()
            .split(", ")
        {
            items.push_back(i.parse()?);
        }
        let t1 = match cap.name("vfirst").ok_or("no cap for vfirst")?.as_str() {
            "old" => Term::Old,
            s => Term::Num(s.parse()?),
        };
        let t2 = match cap.name("vsecond").ok_or("no cap for vfirst")?.as_str() {
            "old" => Term::Old,
            s => Term::Num(s.parse()?),
        };

        let op = match cap.name("op").ok_or("no cap for op")?.as_str() {
            "*" => Operation::Mult,
            "+" => Operation::Add,
            _ => return Err(Box::new(MyError::Generic("not a known operation"))),
        };

        let test: isize = cap
            .name("divby")
            .ok_or("no cap for divby")?
            .as_str()
            .parse()?;

        let tm: usize = cap
            .name("truemonkey")
            .ok_or("no cap for divby")?
            .as_str()
            .parse()?;

        let fm: usize = cap
            .name("falsemonkey")
            .ok_or("no cap for divby")?
            .as_str()
            .parse()?;

        Ok(Monkey {
            items: items,
            test: test,
            t1: t1,
            t2: t2,
            op: op,
            to_true: tm,
            to_false: fm,
            inspected: 0,
        })
    }
}

fn parse(input: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    input.split("\n\n").map(|x| Monkey::from_str(x)).collect()
}

fn part_one(mut input: Vec<Monkey>, rounds: usize) -> usize {
    for _ in 0..rounds {
        for i in 0..input.len() {
            while let Some((wl, new_monkey)) = input[i].inspect() {
                input[new_monkey].items.push_back(wl);
            }
        }
    }

    let mut business: Vec<usize> = input.iter().map(|m| m.inspected).collect();
    business.sort();

    business[business.len() - 2] * business[business.len() - 1]
}

fn part_two(mut input: Vec<Monkey>, rounds: usize) -> usize {
    // first find a small common multiple (not nessecarily the smallest, but still)
    let scm = input
        .iter()
        .map(|m| m.test)
        .fold(1, |acc, v| if acc % v != 0 { acc * v } else { acc });

    for _ in 0..rounds {
        for i in 0..input.len() {
            while let Some((wl, new_monkey)) = input[i].inspect_part_two(scm) {
                input[new_monkey].items.push_back(wl);
            }
        }
    }

    let mut business: Vec<usize> = input.iter().map(|m| m.inspected).collect();
    business.sort();

    business[business.len() - 2] * business[business.len() - 1]
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "11"))?;

    println!("Solution part one: {}", part_one(input.clone(), 20));
    println!("Solution part two: {}", part_two(input.clone(), 10000));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse() {
        let input = parse(TEST).unwrap();

        assert_eq!(input[2].items[1], 60);
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST).unwrap();

        assert_eq!(part_one(input.clone(), 20), 10605);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST).unwrap();

        assert_eq!(part_two(input.clone(), 10000), 2713310158);
    }
}

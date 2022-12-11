use regex::Regex;
use rustvent2022::get_input;
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

enum Operation {
    Add,
    Mult,
}

enum Term {
    Old,
    Num(isize),
}

struct Monkey {
    items: Vec<isize>,
    test: isize,
    to_true: usize,
    to_false: usize,
}

impl FromStr for Monkey {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(Box::new(MyError::Generic("Not implemented!")))
    }
}

fn parse(input: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    input.split("\n\n").map(|x| Monkey::from_str(x)).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let input = parse(&get_input("2022", "10"))?;

    // println!("Solution part one: {}", part_one(&input));
    // println!("Solution part two: \n{}", part_two(&input));
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
    }

    // #[test]
    // fn test_part_one() {
    //     let input = parse(TEST).unwrap();

    //     assert_eq!(part_one(&input), 13140);
    // }
}

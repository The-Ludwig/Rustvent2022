use rustvent2022::get_input;
use std::cmp::Ordering;
use std::error::Error;
// use std::fmt;
use std::collections::BinaryHeap;
use std::str::FromStr;

// #[derive(Debug)]
// struct StringError {
//     err: String,
// }

// impl StringError {
//     fn from_str(s: &str) -> Self {
//         (StringError { err: s.to_string() })
//     }

//     fn new(s: String) -> Self {
//         StringError { err: s }
//     }
// }

// impl fmt::Display for StringError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.err)
//     }
// }

// impl Error for StringError {}

#[derive(PartialEq, Eq, Ord, Debug)]
enum RecList {
    List(Vec<RecList>),
    Num(usize),
}

impl FromStr for RecList {
    type Err = Box<dyn Error + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[") {
            let mut v = Vec::<RecList>::new();
            let mut depth = -1;
            let mut start = 1;
            for (i, c) in s.chars().enumerate() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                }
                if (depth == 0 && c == ',') || (depth == -1 && c == ']') {
                    if start != i {
                        v.push(RecList::from_str(&s[start..i])?);
                        start = i + 1;
                    }
                }
            }
            return Ok(RecList::List(v));
        } else {
            return Ok(RecList::Num(s.parse()?));
        }
    }
}

impl PartialOrd for RecList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            RecList::List(l) => match other {
                RecList::List(lr) => {
                    for i in 0..std::cmp::min(l.len(), lr.len()) {
                        let res = l[i].partial_cmp(&lr[i]);
                        if res != Some(Ordering::Equal) {
                            return res;
                        }
                    }
                    l.len().partial_cmp(&lr.len())
                }
                RecList::Num(n) => self.partial_cmp(&RecList::List(vec![RecList::Num(*n)])),
            },
            RecList::Num(n) => match other {
                RecList::List(_) => RecList::List(vec![RecList::Num(*n)]).partial_cmp(other),
                RecList::Num(nr) => n.partial_cmp(nr),
            },
        }
    }
}

fn parse(input: &str) -> Vec<(RecList, RecList)> {
    input
        .split("\n\n")
        .map(|pairs| {
            let mut lines = pairs.lines();
            (
                RecList::from_str(lines.next().unwrap()).unwrap(),
                RecList::from_str(lines.next().unwrap()).unwrap(),
            )
        })
        .collect()
}

fn part_one(input: &Vec<(RecList, RecList)>) -> usize {
    let mut sum = 0;
    for (i, (l1, l2)) in input.iter().enumerate() {
        if l1 <= l2 {
            sum += i + 1;
        }
    }
    sum
}

fn part_two(input: &Vec<(RecList, RecList)>) -> usize {
    let mut heap = BinaryHeap::new();
    for (l1, l2) in input.iter() {
        heap.push(l1);
        heap.push(l2);
    }
    let div1 = RecList::List(vec![RecList::List(vec![RecList::Num(2)])]);
    let div2 = RecList::List(vec![RecList::List(vec![RecList::Num(6)])]);
    heap.push(&div1);
    heap.push(&div2);
    let sorted = heap.into_sorted_vec();
    let i1 = sorted.iter().position(|&x| *x == div1).unwrap() + 1;
    let i2 = sorted.iter().position(|&x| *x == div2).unwrap() + 1;
    i1 * i2
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "13"));
    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse() {
        let input = parse(TEST);

        assert_eq!(input[2].0, RecList::List(vec![RecList::Num(9)]));
        assert_eq!(input[5].0, RecList::List(Vec::new()));
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST);
        assert_eq!(part_two(&input), 140);
    }
}

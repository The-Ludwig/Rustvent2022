use rustvent2022::get_input;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct StringError {
    err: String,
}

impl StringError {
    fn from_str(s: &str) -> Self {
        (StringError { err: s.to_string() })
    }

    fn new(s: String) -> Self {
        StringError { err: s }
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl Error for StringError {}

#[derive(PartialEq, Debug)]
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
                } else if depth == 0 && c == ',' {
                    v.push(RecList::from_str(&s[start..i])?);
                    start = i + 1;
                }
            }
            return Ok(RecList::List(v));
        } else {
            return Ok(RecList::Num(s.parse()?));
        }
    }
}

impl RecList{
    fn less(&self, other: &RecList) -> Option(bool){
        match self {
            RecList::List(l) => {
                match other {
                    
                }
            },
            RecList::Num(n) => {
                match other {
                    RecList::List(l) {
                    
                    },
                    RecList::Num()
                }
            }
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

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let input = parse(&get_input("2022", "13"));

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

        assert_eq!(input[2].0, RecList::from_str("[9]").unwrap());
    }

    //     #[test]
    //     fn test_part_one() {
    //         let input = parse(TEST);

    //         assert_eq!(part_one(&input), 31);
    //     }

    //     #[test]
    //     fn test_part_two() {
    //         let input = parse(TEST);

    //         assert_eq!(part_two(&input), 29);
    //     }
}

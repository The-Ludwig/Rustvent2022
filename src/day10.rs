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

#[derive(Debug, PartialEq)]
enum Instruction {
    Addx(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let mut ps = s.split(" ");

        match ps.next().ok_or("String is empty!")? {
            "addx" => Ok(Addx(
                ps.next().ok_or("Addx instruction too short")?.parse()?,
            )),
            "noop" => Ok(Noop),
            _ => Err(Box::new(MyError::Generic("not a valid instruction"))),
        }
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input.lines().map(|x| Instruction::from_str(x)).collect()
}

fn part_two(input: &Vec<Instruction>) -> String {
    use Instruction::*;
    let mut x = 1;
    let mut cycle = 0;
    let mut processing = 0;

    let mut iter = input.iter();
    let mut pos = Some(&Instruction::Noop);
    let mut s = String::with_capacity(40 * 7);

    while let Some(i) = pos {
        if cycle > 0 {
            if (((cycle - 1) % 40 - x) as isize).abs() <= 1 {
                s.push('#');
            } else {
                s.push('.');
            }
            if cycle % 40 == 0 {
                s.push('\n');
            }
        }

        if processing == 0 {
            if let Addx(s) = i {
                x += s;
            }
            pos = iter.next();
            if let Some(i) = pos {
                if let Addx(_) = i {
                    processing += 2;
                } else {
                    processing += 1;
                }
            }
        }
        cycle += 1;
        processing -= 1;
    }
    s
}

fn part_one(input: &Vec<Instruction>) -> isize {
    use Instruction::*;
    let mut x = 1;
    let mut strengths = 0;
    let mut cycle = 0;
    let mut processing = 0;

    let mut iter = input.iter();
    let mut pos = Some(&Instruction::Noop);

    while let Some(i) = pos {
        if (cycle as isize - 20) % 40 == 0 {
            strengths += x * cycle as isize;
        }

        if processing == 0 {
            if let Addx(s) = i {
                x += s;
            }
            pos = iter.next();
            if let Some(i) = pos {
                if let Addx(_) = i {
                    processing += 2;
                } else {
                    processing += 1;
                }
            }
        }
        cycle += 1;
        processing -= 1;
    }

    strengths
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "10"))?;

    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: \n{}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_parse() {
        let input = parse(TEST).unwrap();
        use Instruction::*;

        assert_eq!(input[input.len() - 1], Noop);
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST).unwrap();

        assert_eq!(part_one(&input), 13140);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST).unwrap();
        let s = part_two(&input);

        println!("{}", s);
        assert_eq!(
            s,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}

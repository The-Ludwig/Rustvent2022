use rustvent2022::get_input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    R,
    L,
    U,
    D,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    dir: Direction,
    len: usize,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let dir = parts.next().ok_or("No Direction")?;
        let number: usize = parts
            .next()
            .ok_or("No Number")?
            .parse()
            .or(Err("Can't parse number"))?;

        Ok(match dir {
            "R" => Instruction {
                dir: Direction::R,
                len: number,
            },
            "L" => Instruction {
                dir: Direction::L,
                len: number,
            },
            "U" => Instruction {
                dir: Direction::U,
                len: number,
            },
            "D" => Instruction {
                dir: Direction::D,
                len: number,
            },
            _ => Err("Can't parse direction")?,
        })
    }
}

struct Marker<'a> {
    x: isize,
    y: isize,
    visited: HashSet<(isize, isize)>,
    son: Option<&'a mut Marker<'a>>,
}

impl<'a> Marker<'a> {
    fn new(son: Option<&'a mut Marker<'a>>) -> Self {
        Marker {
            x: 0,
            y: 0,
            visited: HashSet::from([(0, 0)]),
            son: son,
        }
    }

    fn follow(&mut self, other: &'a mut Marker<'a>) {}

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::R => self.x += 1,
            Direction::L => self.x -= 1,
            Direction::U => self.y += 1,
            Direction::D => self.y -= 1,
        }
        self.visited.insert((self.x, self.y));

        if let Some(son) = self.son {}
    }

    fn move_all(&mut self, inst: &Instruction) {
        for _ in 0..inst.len {
            self.move_dir(&inst.dir);
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let input = parse(&get_input("2022", "8"));
    // println!("Solution part one: {}", part_one(&input));
    // println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_parse() {
        let input = parse(TEST);

        assert_eq!(
            input[4],
            Instruction {
                dir: Direction::R,
                len: 4
            }
        );
    }

    // #[test]
    // fn test_part_two() {
    //     let input = parse(TEST);

    //     assert_eq!(part_two(&input), 8);
    // }

    // #[test]
    // fn test_part_one() {
    //     let input = parse(TEST);

    //     assert_eq!(part_one(&input), 21);
    // }
}

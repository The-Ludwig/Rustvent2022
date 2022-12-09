use rustvent2022::get_input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    R,
    L,
    U,
    D,
    UR,
    UL,
    DR,
    DL,
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

#[derive(Clone)]
struct Marker {
    x: isize,
    y: isize,
    visited: HashSet<(isize, isize)>,
}

impl Marker {
    fn new() -> Self {
        Marker {
            x: 0,
            y: 0,
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn follow(&mut self, other: &Marker) {
        if (self.x - other.x).abs() > 1 && self.y == other.y {
            if self.x < other.x {
                self.move_dir(&Direction::R);
            } else {
                self.move_dir(&Direction::L);
            }
        } else if (self.y - other.y).abs() > 1 && self.x == other.x {
            if self.y < other.y {
                self.move_dir(&Direction::U);
            } else {
                self.move_dir(&Direction::D);
            }
        } else if (self.y - other.y).abs() > 1 || (self.x - other.x).abs() > 1 {
            if self.x < other.x && self.y < other.y {
                self.move_dir(&Direction::UR);
            } else if self.x < other.x && self.y > other.y {
                self.move_dir(&Direction::DR);
            } else if self.x > other.x && self.y < other.y {
                self.move_dir(&Direction::UL);
            } else if self.x > other.x && self.y > other.y {
                self.move_dir(&Direction::DL);
            } else {
                panic!("Not a valid position");
            }
        }
    }

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::R => self.x += 1,
            Direction::L => self.x -= 1,
            Direction::U => self.y += 1,
            Direction::D => self.y -= 1,
            Direction::UR => {
                self.x += 1;
                self.y += 1;
            }
            Direction::UL => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::DR => {
                self.y -= 1;
                self.x += 1;
            }
            Direction::DL => {
                self.y -= 1;
                self.x -= 1;
            }
        }
        self.visited.insert((self.x, self.y));
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}

fn part_one(input: &Vec<Instruction>) -> usize {
    let mut h = Marker::new();
    let mut t = Marker::new();

    for instr in input {
        for _ in 0..instr.len {
            h.move_dir(&instr.dir);
            t.follow(&h);
        }
    }
    t.visited.len()
}

fn part_two(input: &Vec<Instruction>) -> usize {
    let mut rope = vec![Marker::new(); 10];

    for instr in input {
        for _ in 0..instr.len {
            rope[0].move_dir(&instr.dir);
            for i in 1..rope.len() {
                let par = rope[i - 1].clone();
                rope[i].follow(&par);
            }
        }
    }
    rope.last().unwrap().visited.len()
}
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "9"));
    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
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

    #[test]
    fn test_part_one() {
        let input = parse(TEST);

        assert_eq!(part_one(&input), 13);
    }

    const TEST_TWO: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_two() {
        let input = parse(TEST_TWO);

        assert_eq!(part_two(&input), 36);
    }
}

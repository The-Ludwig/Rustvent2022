use rustvent2022::get_input;
use std::io;

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Tie => 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_char(c: &str) -> Result<RPS, ()> {
        use RPS::*;
        match c {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }

    fn value(&self) -> i32 {
        use RPS::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn loser(self) -> RPS {
        use RPS::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
    fn winner(self) -> RPS {
        use RPS::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn win(&self, other: &RPS) -> Outcome {
        if self == other {
            return Outcome::Tie;
        }
        let winner = self.winner();
        if *other == winner {
            return Outcome::Lose;
        }
        return Outcome::Win;
    }
}

fn parse(input: &str) -> io::Result<Vec<Vec<RPS>>> {
    Ok(input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|c| RPS::from_char(c).unwrap())
                .collect()
        })
        .collect())
}

fn part_one(input: &Vec<Vec<RPS>>) -> i32 {
    input
        .iter()
        .map(|v| v[1].value() + v[1].win(&v[0]).value())
        .sum()
}

fn part_two(input: &Vec<Vec<RPS>>) -> i32 {
    use RPS::*;
    input
        .iter()
        .map(|v| {
            // not gonna be the nice, readable version, but I don't want to implement another enum
            let chosen = match v[1] {
                Rock => v[0].loser(),
                Paper => v[0],
                Scissors => v[0].winner(),
            };
            chosen.value() + chosen.win(&v[0]).value()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "2")).unwrap();
    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use RPS::*;
        let input = parse("inputs/day02_test").unwrap();
        assert_eq!(
            input,
            vec![
                vec![Rock, Paper],
                vec![Paper, Rock],
                vec![Scissors, Scissors]
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = parse("inputs/day02_test").unwrap();
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        let input = parse("inputs/day02_test").unwrap();
        assert_eq!(part_two(&input), 12);
    }
}

use rustvent2022::get_input;
use std::io;
use std::str::FromStr;

#[derive(Debug, std::cmp::PartialEq)]
struct Interval<T: std::cmp::PartialOrd> {
    low: T,
    high: T,
}

impl<T: std::cmp::PartialOrd> Interval<T> {
    fn new(low: T, high: T) -> Interval<T> {
        Interval::<T> { low, high }
    }

    fn contains(&self, num: &T) -> bool {
        self.low <= *num && *num <= self.high
    }

    fn includes(&self, other: &Interval<T>) -> bool {
        self.contains(&other.low) && self.contains(&other.high)
    }

    fn overlaps(&self, other: &Interval<T>) -> bool {
        self.contains(&other.low) || self.contains(&other.high) || other.includes(&self)
    }
}

impl<T: std::cmp::PartialOrd + std::str::FromStr> FromStr for Interval<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split("-");
        Ok(Self {
            low: T::from_str(spl.next().unwrap())?,
            high: T::from_str(spl.next().unwrap())?,
        })
    }
}

type Input = Vec<(Interval<usize>, Interval<usize>)>;
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut spl = l.split(",");
            (
                Interval::from_str(spl.next().unwrap()).unwrap(),
                Interval::from_str(spl.next().unwrap()).unwrap(),
            )
        })
        .collect()
}

fn part_one(input: &Input) -> usize {
    input
        .iter()
        .filter(|is| is.0.includes(&is.1) || is.1.includes(&is.0))
        .count()
}

fn part_two(input: &Input) -> usize {
    input.iter().filter(|is| is.0.overlaps(&is.1)).count()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "4"));
    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        let input = parse(&fs::read_to_string("inputs/day04_test").unwrap());
        assert_eq!(input[0], (Interval::new(2, 4), Interval::new(6, 8)));
    }

    #[test]
    fn test_part_one() {
        let input = parse(&fs::read_to_string("inputs/day04_test").unwrap());
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = parse(&fs::read_to_string("inputs/day04_test").unwrap());
        assert_eq!(part_two(&input), 4);
    }
}

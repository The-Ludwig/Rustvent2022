use std::fs;
use std::io;

fn parse(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let input = fs::read_to_string(filename)?;
    Ok(input
        .split("\n\n")
        .map(|group| group.lines().map(|x| x.parse::<i32>().unwrap()).collect())
        .collect())
}

fn part_one(input: &Vec<Vec<i32>>) -> i32 {
    let total_cal: Vec<i32> = input.iter().map(|x| x.iter().sum()).collect();
    total_cal.iter().fold(std::i32::MIN, |acc, x| acc.max(*x))
}

fn part_two(input: &Vec<Vec<i32>>) -> i32 {
    let mut total_cal: Vec<i32> = input.iter().map(|x| x.iter().sum()).collect();
    total_cal.sort_by(|a, b| b.cmp(a));
    total_cal[..=2].iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse("inputs/day01").unwrap();
    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let res = parse("inputs/day01_test").unwrap();
        assert_eq!(res, vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]]);
    }

    #[test]
    fn test_part_one() {
        let res = parse("inputs/day01_test").unwrap();
        assert_eq!(part_one(&res), 24000);
    }
    
    #[test]
    fn test_part_two() {
        let res = parse("inputs/day01_test").unwrap();
        assert_eq!(part_two(&res), 45000);
    }
}

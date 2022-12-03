use rustvent2022::get_input;
use std::io;

type Input = Vec<(String, String)>;

fn parse(input: &str) -> io::Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            (
                String::from(&line[..line.len() / 2]),
                String::from(&line[line.len() / 2..]),
            )
        })
        .collect())
}

type InputPartTwo = Vec<Vec<String>>;
fn parse_part_two(input: &str) -> io::Result<InputPartTwo> {
    Ok(input
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(Vec::from)
        .collect())
}

fn find_first_same(v1: &String, v2: &String) -> Option<char> {
    for e1 in v1.chars() {
        match v2.contains(e1) {
            true => return Some(e1.clone()),
            false => {}
        }
    }
    None
}

fn find_contains_all_first(v: &Vec<String>) -> Option<char> {
    'outer: for c in v[0].chars() {
        for s in &v[1..] {
            match s.contains(c) {
                true => (),
                false => continue 'outer,
            }
        }
        return Some(c);
    }

    None
}

fn to_prio(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u8 - 'a' as u8 + 1,
        false => c as u8 - 'A' as u8 + 27,
    }
    .into()
}

fn part_one(input: &Input) -> u32 {
    input
        .iter()
        .map(|x| to_prio(find_first_same(&x.0, &x.1).unwrap()))
        .sum()
}

fn part_two(input: &InputPartTwo) -> u32 {
    input
        .iter()
        .map(|x| to_prio(find_contains_all_first(x).unwrap()))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "3")).unwrap();
    println!("Solution part one: {}", part_one(&input));
    let input = parse_part_two(&get_input("2022", "3")).unwrap();
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        let input = parse(&fs::read_to_string("inputs/day03_test").unwrap()).unwrap();
        assert_eq!(
            input[0],
            (String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp"))
        );
    }

    #[test]
    fn test_part_one() {
        let input = parse(&fs::read_to_string("inputs/day03_test").unwrap()).unwrap();
        assert_eq!(part_one(&input), 157);
    }

    #[test]
    fn test_part_two() {
        let input = parse_part_two(&fs::read_to_string("inputs/day03_test").unwrap()).unwrap();
        assert_eq!(part_two(&input), 70);
    }
}

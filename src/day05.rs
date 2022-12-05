use regex::Regex;
use rustvent2022::get_input;
use std::io;
use std::str::FromStr;

type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);
fn parse(input: &str) -> Input {
    let mut spl = input.split("\n\n");

    let st = spl.next().unwrap();
    let mut lines = st.lines().rev();
    let n_stacks = lines.next().unwrap().replace(" ", "").len();
    let mut stacks = vec![Vec::<char>::new(); n_stacks];

    for line in lines {
        let mut chars = line.chars();
        chars.next().unwrap();
        for i in 0..n_stacks {
            match chars.next() {
                Some(' ') => (),
                Some(c) => stacks[i].push(c),
                None => panic!("Line in input too short!"),
            }
            for _ in 0..3 {
                chars.next();
            }
        }
    }

    let re_inst = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let inst = spl.next().unwrap();
    let instructions: Vec<(usize, usize, usize)> = inst
        .lines()
        .map(|l| {
            let cap = re_inst.captures_iter(l).next().unwrap();
            (
                usize::from_str(&cap[1]).unwrap(),
                usize::from_str(&cap[2]).unwrap(),
                usize::from_str(&cap[3]).unwrap(),
            )
        })
        .collect();

    (stacks, instructions)
}

fn part_one(instructions: &Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<char>>) -> String {
    for (n, from, to) in instructions {
        for _ in 0..*n {
            let o = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(o);
        }
    }

    let mut s = String::with_capacity(stacks.len());
    for stack in stacks {
        match stack.last() {
            Some(c) => s.push(*c),
            None => (),
        }
    }
    s
}

fn part_two(instructions: &Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<char>>) -> String {
    for (n, from, to) in instructions {
        let len = stacks[*from - 1].len();
        let mut tail = stacks[*from - 1].split_off(len.saturating_sub(*n));
        stacks[*to - 1].append(&mut tail);
    }

    let mut s = String::with_capacity(stacks.len());
    for stack in stacks {
        match stack.last() {
            Some(c) => s.push(*c),
            None => (),
        }
    }
    s
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let (mut stacks, instructions) = parse(&get_input("2022", "5"));
    println!(
        "Solution part one: {}",
        part_one(&instructions, &mut stacks.clone())
    );
    println!(
        "Solution part two: {}",
        part_two(&instructions, &mut stacks.clone())
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        let input = parse(&fs::read_to_string("inputs/day05_test").unwrap());
        assert_eq!(input.0[0][0], 'Z');
        assert_eq!(input.1[0], (1, 2, 1));
    }

    #[test]
    fn test_part_one() {
        let (mut stacks, instructions) = parse(&fs::read_to_string("inputs/day05_test").unwrap());
        assert_eq!(part_one(&instructions, &mut stacks), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let (mut stacks, instructions) = parse(&fs::read_to_string("inputs/day05_test").unwrap());
        assert_eq!(part_two(&instructions, &mut stacks), "MCD");
    }
}

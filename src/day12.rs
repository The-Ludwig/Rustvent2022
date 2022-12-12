use rustvent2022::get_input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn normal(c: char) -> char {
    match c {
        'E' => 'z',
        'S' => 'a',
        c => c,
    }
}

fn min_dist(input: &Vec<Vec<char>>, startx: isize, starty: isize) -> usize {
    let mut pos = (starty, startx);
    let mut visited: HashMap<_, usize> = HashMap::from([(pos, 0)]);
    let mut to_visit = VecDeque::<(isize, isize)>::new();

    loop {
        match input[pos.0 as usize][pos.1 as usize] {
            'E' => break,
            c => {
                let this_dist = visited[&(pos.0, pos.1)];
                for (y, x) in [
                    (pos.0 + 1, pos.1),
                    (pos.0 - 1, pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1 - 1),
                ] {
                    if x < 0 || y < 0 || x >= input[0].len() as isize || y >= input.len() as isize {
                        continue;
                    }
                    let new_c = input[y as usize][x as usize];
                    if normal(new_c) as u8 <= normal(c) as u8 + 1 {
                        let dist = visited.entry((y, x)).or_insert_with(|| {
                            to_visit.push_back((y, x));
                            this_dist + 1
                        });
                        if *dist > this_dist + 1 {
                            *dist = this_dist + 1;
                        }
                    }
                }
            }
        };
        match to_visit.pop_front() {
            Some(p) => pos = p,
            None => return usize::MAX,
        }
    }
    visited[&pos]
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    let mut lens = Vec::<usize>::new();

    // starting pos
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'S' || input[y][x] == 'a' {
                lens.push(min_dist(input, x as isize, y as isize));
            }
        }
    }
    println!("{lens:#?}");
    *lens.iter().min().unwrap()
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    // starting pos
    let mut pos: (isize, isize) = (0, 0);
    'outer: for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'S' {
                pos.0 = y as isize;
                pos.1 = x as isize;
                break 'outer;
            }
        }
    }

    min_dist(input, pos.1, pos.0)
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "12"));

    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_parse() {
        let input = parse(TEST);

        assert_eq!(input[0][1], 'a');
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST);

        assert_eq!(part_one(&input), 31);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST);

        assert_eq!(part_two(&input), 29);
    }
}

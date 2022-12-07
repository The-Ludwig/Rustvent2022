use rustvent2022::get_input;

fn has_duplicate(s: &str) -> bool {
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            if s.chars().nth(i) == s.chars().nth(j) {
                return true;
            }
        }
    }
    false
}

fn part(input: &str, len: usize) -> usize {
    for i in len..input.len() {
        if !has_duplicate(&input[i - len..i]) {
            return i;
        }
    }
    panic!("Input is not correct, should not be here.");
}

// fn part_two(instructions: &Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<char>>) -> String {
//     for (n, from, to) in instructions {
//         let len = stacks[*from - 1].len();
//         let mut tail = stacks[*from - 1].split_off(len.saturating_sub(*n));
//         stacks[*to - 1].append(&mut tail);
//     }

//     let mut s = String::with_capacity(stacks.len());
//     for stack in stacks {
//         match stack.last() {
//             Some(c) => s.push(*c),
//             None => (),
//         }
//     }
//     s
// }

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2022", "6");
    println!("Solution part one: {}", part(&input, 4));
    println!("Solution part two: {}", part(&input, 14));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part(&test, 4), 5);
        assert_eq!(part(&test, 14), 23);
    }
}

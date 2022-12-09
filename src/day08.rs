use rustvent2022::get_input;

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut ret = Vec::new();
    for line in input.lines() {
        ret.push(line.chars().map(|c| c as u8 - '0' as u8).collect());
    }
    ret
}

fn part_two(input: &Vec<Vec<u8>>) -> usize {
    let (len_x, len_y) = (input[0].len(), input.len());

    let mut best_scenic_score = 0;

    // urgs this is going to be ugly this time...
    for x in 0..len_x {
        for y in 0..len_y {
            let tree = input[y][x];

            let mut vd_left = 0;
            for x_c in (0..x).rev() {
                vd_left += 1;
                if input[y][x_c] >= tree {
                    break;
                }
            }

            let mut vd_right = 0;
            for x_c in x + 1..len_x {
                vd_right += 1;
                if input[y][x_c] >= tree {
                    break;
                }
            }

            let mut vd_up = 0;
            for y_c in (0..y).rev() {
                vd_up += 1;
                if input[y_c][x] >= tree {
                    break;
                }
            }

            let mut vd_down = 0;
            for y_c in y + 1..len_y {
                vd_down += 1;
                if input[y_c][x] >= tree {
                    break;
                }
            }

            let sscore = vd_up * vd_down * vd_right * vd_left;
            if sscore > best_scenic_score {
                best_scenic_score = sscore;
            }
        }
    }
    best_scenic_score
}

fn part_one(input: &Vec<Vec<u8>>) -> usize {
    let (len_x, len_y) = (input[0].len(), input.len());

    let mut vis_trees = 2 * len_x + 2 * len_y - 4;

    // urgs this is going to be ugly this time...
    for x in 1..len_x - 1 {
        for y in 1..len_y - 1 {
            let tree = input[y][x];

            let mut vis = true;
            for x_c in 0..x {
                if input[y][x_c] >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                vis_trees += 1;
                continue;
            }

            vis = true;
            for x_c in x + 1..len_x {
                if input[y][x_c] >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                vis_trees += 1;
                continue;
            }

            vis = true;
            for y_c in 0..y {
                if input[y_c][x] >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                vis_trees += 1;
                continue;
            }

            vis = true;
            for y_c in y + 1..len_y {
                if input[y_c][x] >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                vis_trees += 1;
                continue;
            }
        }
    }

    vis_trees
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2022", "8"));
    println!("Solution part one: {}", part_one(&input));
    println!("Solution part two: {}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn test_parse() {
        let input = parse(TEST);

        assert_eq!(input[1][1], 5);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST);

        assert_eq!(part_two(&input), 8);
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST);

        assert_eq!(part_one(&input), 21);
    }
}

use regex::Regex;
use rustvent2022::get_input;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::str::FromStr;
use std::time;

fn parse(input: &str) -> HashMap<&str, (usize, Vec<&str>)> {
    let re = Regex::new(
        r"Valve (?P<ValveName>[A-Z]{2}) has flow rate=(?P<FlowRate>\d+); tunnels? leads? to valves? (?P<Adjacent>(?:[A-Z]{2}, )*[A-Z]{2})",
    ).unwrap();

    let mut hs = HashMap::new();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let adjacent = cap.name("Adjacent").unwrap().as_str().split(", ").collect();

        hs.insert(
            cap.name("ValveName").unwrap().as_str(),
            (
                cap.name("FlowRate").unwrap().as_str().parse().unwrap(),
                adjacent,
            ),
        );
    }

    hs
}

fn possible(
    path: &Vec<&str>,
    dists: &HashMap<&str, HashMap<&str, usize>>,
    mut minutes: isize,
) -> bool {
    let mut cur = "AA";
    for pos in path {
        minutes -= dists[pos][cur] as isize + 1;
        cur = pos;
    }
    minutes >= 0
}

fn extend<'a>(
    path: &Vec<&'a str>,
    dists: &HashMap<&'a str, HashMap<&'a str, usize>>,
    minutes: isize,
) -> Vec<Vec<&'a str>> {
    let mut new_paths = Vec::new();

    for next in dists.keys() {
        if !path.contains(next) {
            let mut new_path = path.clone();
            new_path.push(next);
            if possible(&new_path, dists, minutes) {
                new_paths.push(new_path);
            }
        }
    }

    new_paths
}

fn dists_floyd_warshall<'a>(
    input: &HashMap<&'a str, (usize, Vec<&'a str>)>,
) -> HashMap<&'a str, HashMap<&'a str, usize>> {
    // calculate shortest path between every vertex non-zero-vertex
    // not using two fold dijkstra, but floyd-warshall
    let mut dists: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for key in input.keys() {
        let mut ds: HashMap<&str, usize> = HashMap::new();
        for key2 in input.keys() {
            // Most hackiest solution ever
            ds.insert(key2, usize::MAX / 2);
        }
        dists.insert(key, ds);
    }
    for key in input.keys() {
        *dists.get_mut(key).unwrap().get_mut(key).unwrap() = 0;
    }
    for (from, (_flow, tos)) in input {
        for to in tos {
            *dists.get_mut(from).unwrap().get_mut(to).unwrap() = 1;
        }
    }
    for key1 in input.keys() {
        for key2 in input.keys() {
            for key3 in input.keys() {
                if dists[key2][key3] > dists[key2][key1] + dists[key1][key3] {
                    *dists.get_mut(key2).unwrap().get_mut(key3).unwrap() =
                        dists[key2][key1] + dists[key1][key3];
                }
            }
        }
    }

    // delete keys with pressure 0, they do never need to be openend
    for (del_key, _) in input.iter().filter(|(_k, v)| v.0 == 0) {
        dists.remove(del_key);
    }
    dists
}

// I gave up and got tips on the subreddit... This was really really hard for me...
fn part_one(input: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
    let dists = dists_floyd_warshall(&input);

    // this assumes the path is valid
    let scores = |path: &Vec<&str>| {
        let mut mins = 30;
        let mut s = 0;
        let mut cur = "AA";
        for edge in path {
            mins -= dists[edge][cur] + 1;
            s += mins * input[edge].0;
            cur = edge;
        }
        s
    };

    // build all paths
    let mut paths: VecDeque<Vec<&str>> = VecDeque::new();
    // inital paths
    for key in dists.keys() {
        paths.push_back(vec![key]);
    }
    let mut max_score = 0;
    while let Some(cur) = paths.pop_front() {
        // let cur = paths.pop_front().unwrap();
        let new_paths = extend(&cur, &dists, 30);
        if new_paths.len() == 0 {
            let score = scores(&cur);
            if score > max_score {
                max_score = score;
            }
        } else {
            for path in new_paths.into_iter() {
                paths.push_back(path);
            }
        }
    }

    max_score
}

// I gave up and got tips on the subreddit... This was really really hard for me...
fn part_two(input: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
    let dists = dists_floyd_warshall(&input);

    // this assumes the path is valid
    let scores = |path: &Vec<&str>| {
        let mut mins = 26;
        let mut s = 0;
        let mut cur = "AA";
        for edge in path {
            mins -= dists[edge][cur] + 1;
            s += mins * input[edge].0;
            cur = edge;
        }
        s
    };

    // build all paths
    let mut paths: VecDeque<Vec<&str>> = VecDeque::new();
    let mut final_paths: Vec<Vec<&str>> = Vec::new();
    // inital paths
    for key in dists.keys() {
        paths.push_back(vec![key]);
    }
    while let Some(cur) = paths.pop_front() {
        // let cur = paths.pop_front().unwrap();
        let new_paths = extend(&cur, &dists, 26);
        for path in new_paths.into_iter() {
            paths.push_back(path);
        }
        final_paths.push(cur);
    }

    let mut max_score = 0;

    for i in 0..final_paths.len() {
        println!(
            "{}/{} {}%",
            final_paths.len() * i,
            final_paths.len() * final_paths.len(),
            (final_paths.len() * i) as f32 / (final_paths.len() * final_paths.len()) as f32
        );
        'path_compt: for j in i + 1..final_paths.len() {
            for el in &final_paths[i] {
                if final_paths[j].contains(&el) {
                    continue 'path_compt;
                }
            }
            let score = scores(&final_paths[i]) + scores(&final_paths[j]);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", final_paths.len());

    max_score
}
// fn part_one_stupid_does_not_work(input: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
//     let mut dists: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

//     for key in input.keys() {
//         let mut ds: HashMap<&str, usize> = HashMap::new();
//         ds.insert(key, 0);
//         let mut to_visit: VecDeque<&str> = VecDeque::new();
//         to_visit.push_back(key);
//         while let Some(current) = to_visit.pop_front() {
//             let this_dist = ds[current];
//             for adj in &input[current].1 {
//                 if !ds.contains_key(adj) {
//                     ds.insert(adj, this_dist + 1);
//                     to_visit.push_back(adj);
//                 }
//             }

//             dists.insert(key, ds);
//         }
//     }

//     // here I tried a greedy algorithm, which is incorrect, lets see if I can implement
//     // a time-acceptable brute-force variant...
//     // let mut mins_left = 30;
//     // let mut score = 0;
//     // let mut current = "AA";

//     // loop {
//     //     let mut next = current;
//     //     let mut max_gain = 0;
//     //     for opt in dists.keys() {
//     //         if dists[opt][current] + 1 < mins_left {
//     //             let gain = (mins_left - dists[opt][current] - 1) * input[opt].0;
//     //             if gain == max_gain {
//     //                 println!("FUUUUCK {gain}");
//     //             }
//     //             if gain > max_gain {
//     //                 max_gain = gain;
//     //                 next = opt;
//     //             }
//     //         }
//     //     }
//     //     if max_gain == 0 {
//     //         break;
//     //     }
//     //     println!(
//     //         "Move to: {next} in {} mins, releasing {max_gain}, mins_left: {mins_left}",
//     //         dists[next][current]
//     //     );
//     //     mins_left -= dists[next][current] + 1;
//     //     score += max_gain;
//     //     dists.remove(next);
//     //     current = next;
//     // }
//     0
// }

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let in_str = get_input("2022", "16");
    let input = parse(&in_str);
    let now = time::Instant::now();
    let sol_p1 = part_one(&input);
    println!(
        "Solution part one: {sol_p1} took: {}s",
        now.elapsed().as_secs_f32()
    );
    let now = time::Instant::now();
    let sol_p2 = part_two(&input);
    println!(
        "Solution part two: {sol_p2} took: {}s",
        now.elapsed().as_secs_f32()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_parse() {
        let input = parse(TEST);

        assert_eq!(input["AA"].0, 0);
        assert_eq!(input["II"].1, vec!["AA", "JJ"]);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST);
        assert_eq!(part_two(&input), 1707);
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST);
        assert_eq!(part_one(&input), 1651);
    }

    //     #[test]
    //     fn test_part_two() {
    //         let input = Field::from_str(TEST).unwrap();
    //         assert_eq!(part_two(&input, 20), 56000011);
    //     }
}

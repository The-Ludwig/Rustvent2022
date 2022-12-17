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
        minutes -= dists[cur][pos] as isize + 1;
    }
    minutes >= 0
}

fn part_one(input: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
    let mut dists: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for key in input.keys() {
        // Dijkstra for every key
        let mut ds: HashMap<&str, usize> = HashMap::new();
        ds.insert(key, 0);
        let mut to_visit: VecDeque<&str> = VecDeque::new();
        to_visit.push_back(key);
        while let Some(current) = to_visit.pop_front() {
            let this_dist = ds[current];
            for adj in &input[current].1 {
                if !ds.contains_key(adj) {
                    ds.insert(adj, this_dist + 1);
                    to_visit.push_back(adj);
                }
            }
        }
        dists.insert(key, ds);
    }

    // here I tried a greedy algorithm, which is incorrect, lets see if I can implement
    // a time-acceptable brute-force variant...
    // let mut mins_left = 30;
    // let mut score = 0;
    // let mut current = "AA";

    // loop {
    //     let mut next = current;
    //     let mut max_gain = 0;
    //     for opt in dists.keys() {
    //         if dists[opt][current] + 1 < mins_left {
    //             let gain = (mins_left - dists[opt][current] - 1) * input[opt].0;
    //             if gain == max_gain {
    //                 println!("FUUUUCK {gain}");
    //             }
    //             if gain > max_gain {
    //                 max_gain = gain;
    //                 next = opt;
    //             }
    //         }
    //     }
    //     if max_gain == 0 {
    //         break;
    //     }
    //     println!(
    //         "Move to: {next} in {} mins, releasing {max_gain}, mins_left: {mins_left}",
    //         dists[next][current]
    //     );
    //     mins_left -= dists[next][current] + 1;
    //     score += max_gain;
    //     dists.remove(next);
    //     current = next;
    // }

    score
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let in_str = get_input("2022", "16");
    let input = parse(&in_str);
    let now = time::Instant::now();
    let sol_p1 = part_one(&input);
    println!(
        "Solution part one: {sol_p1} took: {}s",
        now.elapsed().as_secs_f32()
    );
    // let now = time::Instant::now();
    // let sol_p2 = part_two(&input, 4000000);
    // println!(
    //     "Solution part two: {sol_p2} took: {}s",
    //     now.elapsed().as_secs_f32()
    // );
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

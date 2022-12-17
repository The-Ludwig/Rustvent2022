use regex::Regex;
use rustvent2022::get_input;
use std::error::Error;
use std::str::FromStr;
use std::time;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Sensor,
    Beacon,
    Empty,
    NoBeacon,
}

#[derive(Clone)]
struct Field {
    sensors: Vec<Point>,
    beacons: Vec<Point>,
    dists: Vec<usize>,
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl FromStr for Field {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Sensor at x=(?P<xS>-?\d+), y=(?P<yS>-?\d+): closest beacon is at x=(?P<xB>-?\d+), y=(?P<yB>-?\d+)",
        )?;
        let mut coords: Vec<(isize, isize, isize, isize)> = Vec::new();
        let mut sensors = Vec::new();
        let mut beacons = Vec::new();
        let mut dists = Vec::new();
        let mut xmin = isize::MAX;
        let mut xmax = isize::MIN;
        let mut ymin = isize::MAX;
        let mut ymax = isize::MIN;

        for l in s.lines() {
            let m = re.captures(l).ok_or("No capture found")?;

            let x_s: isize = m.name("xS").ok_or("No such name")?.as_str().parse()?;
            let y_s: isize = m.name("yS").ok_or("No such name")?.as_str().parse()?;
            let x_b: isize = m.name("xB").ok_or("No such name")?.as_str().parse()?;
            let y_b: isize = m.name("yB").ok_or("No such name")?.as_str().parse()?;

            let d = (x_s - x_b).abs() + (y_s - y_b).abs();

            if x_s - d < xmin {
                xmin = x_s - d;
            }
            if x_s + d > xmax {
                xmax = x_s + d;
            }
            if y_s - d < ymin {
                ymin = y_s - d;
            }
            if y_s + d > ymax {
                ymax = y_s + d;
            }

            sensors.push(Point { x: x_s, y: y_s });
            dists.push(d as usize);
            beacons.push(Point { x: x_b, y: y_b });
        }

        Ok(Field {
            sensors,
            beacons,
            dists,
            xmin,
            xmax,
            ymin,
            ymax,
        })
    }
}

impl Field {
    fn get_jump(&self, x: isize, y: isize) -> Option<(Tile, isize)> {
        if x <= self.xmax && x >= self.xmin && y <= self.ymax && y >= self.ymin {
            if self.beacons.contains(&Point { x, y }) {
                return Some((Tile::Beacon, 1));
            }
            if self.sensors.contains(&Point { x, y }) {
                return Some((Tile::Sensor, 1));
            }
            let mut ynext = Vec::new();
            for (sens, dist) in self.sensors.iter().zip(self.dists.iter()) {
                if ((x - sens.x).abs() + (y - sens.y).abs()) as usize <= *dist {
                    ynext.push((sens.y + *dist as isize - (x - sens.x).abs()) - y + 1);
                }
            }
            match ynext.iter().min() {
                Some(ypp) => Some((Tile::NoBeacon, *ypp)),
                None => Some((Tile::Empty, 0)),
            }
        } else {
            None
        }
    }
    fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if x <= self.xmax && x >= self.xmin && y <= self.ymax && y >= self.ymin {
            if self.beacons.contains(&Point { x, y }) {
                return Some(Tile::Beacon);
            }
            if self.sensors.contains(&Point { x, y }) {
                return Some(Tile::Sensor);
            }
            for (sens, dist) in self.sensors.iter().zip(self.dists.iter()) {
                if ((x - sens.x).abs() + (y - sens.y).abs()) as usize <= *dist {
                    return Some(Tile::NoBeacon);
                }
            }
            Some(Tile::Empty)
        } else {
            None
        }
    }
}

fn part_one(f: &Field, y: isize) -> usize {
    let mut no_beacon = 0;
    for x in f.xmin..=f.xmax {
        if let Some(tile) = f.get(x, y) {
            match tile {
                Tile::NoBeacon => no_beacon += 1,
                _ => (),
            }
        } else {
            panic!("oh fuck no");
        }
    }
    no_beacon
}

fn part_two(f: &Field, max: isize) -> usize {
    let mut x = 0;
    while x <= max {
        let mut y = 0;
        while y <= max {
            if let Some((tile, jump)) = f.get_jump(x, y) {
                match tile {
                    Tile::Empty => return (x * 4000000 + y) as usize,
                    _ => y += jump,
                }
            } else {
                panic!("oh fuck no");
            }
        }
        x += 1;
    }
    panic!("oh fuck no 2");
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = Field::from_str(&get_input("2022", "15"))?;
    let now = time::Instant::now();
    let sol_p1 = part_one(&input, 2000000);
    println!(
        "Solution part one: {sol_p1} took: {}s",
        now.elapsed().as_secs_f32()
    );
    let now = time::Instant::now();
    let sol_p2 = part_two(&input, 4000000);
    println!(
        "Solution part two: {sol_p2} took: {}s",
        now.elapsed().as_secs_f32()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_parse() {
        let input = Field::from_str(TEST).unwrap();

        assert_eq!(input.get(10, 8), Some(Tile::NoBeacon));
    }

    #[test]
    fn test_part_one() {
        let input = Field::from_str(TEST).unwrap();
        assert_eq!(part_one(&input, 10), 26);
    }

    #[test]
    fn test_part_two() {
        let input = Field::from_str(TEST).unwrap();
        assert_eq!(part_two(&input, 20), 56000011);
    }
}

use rustvent2022::get_input;
use std::error::Error;
// use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Sand,
    Wall,
    Empty,
}

#[derive(Clone)]
struct Field {
    tiles: Vec<Vec<Tile>>,
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    floor: bool,
}

impl FromStr for Field {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords: Vec<Vec<(isize, isize)>> = Vec::new();
        let mut xmin = 500;
        let mut xmax = 500;
        let mut ymin = 0;
        let mut ymax = 0;

        for l in s.lines() {
            let mut w = Vec::new();
            for pair in l.split(" -> ") {
                let mut nums = pair.split(",");
                let x = nums.next().unwrap().parse()?;
                let y = nums.next().unwrap().parse()?;

                if x < xmin {
                    xmin = x;
                }
                if x > xmax {
                    xmax = x;
                }
                if y > ymax {
                    ymax = y;
                }
                if y < ymin {
                    ymin = y;
                }

                w.push((x, y));
            }
            coords.push(w);
        }

        let mut tiles =
            vec![vec![Tile::Empty; (ymax - ymin + 1) as usize]; (xmax - xmin + 1) as usize];

        for wall in coords {
            for i in 1..wall.len() {
                let (x1, y1) = wall[i - 1];
                let (x2, y2) = wall[i];
                if x1 == x2 {
                    for y in isize::min(y1, y2)..=isize::max(y1, y2) {
                        tiles[(x1 - xmin) as usize][(y - ymin) as usize] = Tile::Wall;
                    }
                } else {
                    for x in isize::min(x1, x2)..=isize::max(x1, x2) {
                        tiles[(x - xmin) as usize][(y1 - ymin) as usize] = Tile::Wall;
                    }
                }
            }
        }

        Ok(Field {
            tiles,
            xmin,
            xmax,
            ymin,
            ymax,
            floor: false,
        })
    }
}

impl Field {
    fn add_floor(&mut self) {
        self.floor = true;
        self.ymax += 1;
        for y in &mut self.tiles {
            y.push(Tile::Empty);
        }
    }

    fn set_sand(&mut self, x: isize, y: isize) {
        if x > self.xmax {
            for _ in 0..x - self.xmax {
                self.tiles.push(vec![Tile::Empty; self.tiles[0].len()]);
            }
            self.xmax = x;
        } else if x < self.xmin {
            let mut new_front =
                vec![vec![Tile::Empty; self.tiles[0].len()]; (self.xmin - x) as usize];
            new_front.append(&mut self.tiles);
            self.tiles = new_front;
            self.xmin = x;
        }
        if x <= self.xmax && x >= self.xmin && y <= self.ymax && y >= self.ymin {
            self.tiles[(x - self.xmin) as usize][(y - self.ymin) as usize] = Tile::Sand;
        } else {
            panic!(
                "Can't set this tile! {} {} {} {} {} {}",
                x, y, self.xmin, self.xmax, self.ymin, self.ymax
            );
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if self.floor {
            if y > self.ymax {
                return Some(Tile::Wall);
            }
            if x <= self.xmax && x >= self.xmin && y <= self.ymax && y >= self.ymin {
                Some(self.tiles[(x - self.xmin) as usize][(y - self.ymin) as usize])
            } else {
                Some(Tile::Empty)
            }
        } else {
            if x <= self.xmax && x >= self.xmin && y <= self.ymax && y >= self.ymin {
                Some(self.tiles[(x - self.xmin) as usize][(y - self.ymin) as usize])
            } else {
                None
            }
        }
    }

    fn add_sand(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        match self.get(x, y) {
            Some(Tile::Sand) => return false,
            _ => (),
        };

        loop {
            if let Some(tile) = self.get(x, y + 1) {
                if tile == Tile::Empty {
                    y += 1;
                    continue;
                }
            } else {
                return false;
            }
            if let Some(tile) = self.get(x - 1, y + 1) {
                if tile == Tile::Empty {
                    y += 1;
                    x -= 1;
                    continue;
                }
            } else {
                return false;
            }
            if let Some(tile) = self.get(x + 1, y + 1) {
                if tile == Tile::Empty {
                    y += 1;
                    x += 1;
                    continue;
                }
            } else {
                return false;
            }
            self.set_sand(x, y);
            return true;
        }
    }
}

fn part_one(mut f: Field) -> usize {
    let mut run = 0;
    while f.add_sand() {
        run += 1;
    }

    run
}

fn part_two(mut f: Field) -> usize {
    let mut run = 0;
    f.add_floor();
    while f.add_sand() {
        run += 1;
    }

    run
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = Field::from_str(&get_input("2022", "14"))?;
    println!("Solution part one: {}", part_one(input.clone()));
    println!("Solution part one: {}", part_two(input.clone()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse() {
        let input = Field::from_str(TEST).unwrap();

        assert_eq!(input.get(500, 9), Some(Tile::Wall));
    }

    #[test]
    fn test_part_one() {
        let input = Field::from_str(TEST).unwrap();
        assert_eq!(part_one(input.clone()), 24);
    }

    #[test]
    fn test_part_two() {
        let input = Field::from_str(TEST).unwrap();
        assert_eq!(part_two(input.clone()), 93);
    }
}

use rustvent2022::get_input;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum DirectoryEntry {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

#[derive(Debug)]
struct Directory {
    pub entries: Vec<DirectoryEntry>,
    pub name: String,
    pub parent: Option<Rc<RefCell<Directory>>>,
}

#[derive(Debug)]
struct File {
    pub size: usize,
    pub name: String,
}

impl Directory {
    fn cd(&self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        for entry in &self.entries {
            if let DirectoryEntry::Directory(dir) = entry {
                if dir.borrow().name == name {
                    return Some(Rc::clone(&dir));
                }
            }
        }
        None
    }

    fn size(&self) -> usize {
        let mut size = 0;

        for entry in &self.entries {
            size += match entry {
                DirectoryEntry::File(f) => f.size,
                DirectoryEntry::Directory(d) => d.borrow().size(),
            }
        }

        size
    }

    fn part_two(&self) -> usize {
        let needed_size = 30000000 - (70000000 - self.size());
        let sizes = self._part_two(&needed_size);
        *sizes.iter().min().unwrap()
    }

    fn _part_two(&self, needed: &usize) -> Vec<usize> {
        let mut sizes = Vec::<usize>::new();
        let own_size = self.size();
        if own_size >= *needed {
            sizes.push(own_size);
        }

        for entry in &self.entries {
            if let DirectoryEntry::Directory(d) = entry {
                sizes.append(&mut d.borrow()._part_two(needed))
            }
        }

        sizes
    }

    fn part_one(&self) -> usize {
        let mut size = 0;

        for entry in &self.entries {
            if let DirectoryEntry::Directory(d) = entry {
                size += d.borrow().part_one();
            }
        }

        let own_size = self.size();
        if own_size <= 100_000 {
            size += own_size;
        }

        size
    }

    fn from_str(s: &str) -> Result<Rc<RefCell<Self>>, &str> {
        // parent
        let parent = Rc::new(RefCell::new(Directory {
            entries: Vec::new(),
            name: String::new(),
            parent: None,
        }));

        let mut current = Rc::clone(&parent);

        for line in s.lines() {
            if line.starts_with("$ cd ") {
                current = match &line[5..] {
                    "/" => Rc::clone(&parent),
                    ".." => Rc::clone(
                        (current.borrow().parent)
                            .as_ref()
                            .ok_or("Dir has no Parent")?,
                    ),
                    d => current
                        .borrow()
                        .cd(d)
                        .ok_or("cd-ing into a non-existing dir {d}")?,
                }
            } else if line.starts_with("$ ls") {
                continue;
            } else {
                if line.starts_with("dir") {
                    current
                        .borrow_mut()
                        .entries
                        .push(DirectoryEntry::Directory(Rc::new(RefCell::new(
                            Directory {
                                entries: Vec::new(),
                                name: String::from(&line[4..]),
                                parent: Some(Rc::clone(&current)),
                            },
                        ))));
                } else {
                    let parts: Vec<&str> = line.split(" ").collect();
                    if parts.len() > 2 {
                        return Err("Line should be a file and has not the right format.");
                    }

                    current
                        .borrow_mut()
                        .entries
                        .push(DirectoryEntry::File(File {
                            size: parts[0].parse().or(Err("Cant parse file len"))?,
                            name: String::from(parts[1]),
                        }));
                }
            }
        }

        Ok(parent)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2022", "7");
    let dir = Directory::from_str(&input)?;
    println!("Solution part one: {}", dir.borrow().part_one());
    println!("Solution part one: {}", dir.borrow().part_two());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse() {
        let dir = Directory::from_str(TEST_INPUT).unwrap();

        assert_eq!(dir.borrow().entries.len(), 4);
        match dir.borrow().entries.first().unwrap() {
            DirectoryEntry::File(_) => panic!(),
            DirectoryEntry::Directory(d) => assert_eq!("a", &d.borrow().name),
        };
    }

    #[test]
    fn test_part_one() {
        let dir = Directory::from_str(TEST_INPUT).unwrap();
        assert_eq!(95437, dir.borrow().part_one());
    }

    #[test]
    fn test_part_two() {
        let dir = Directory::from_str(TEST_INPUT).unwrap();
        assert_eq!(24933642, dir.borrow().part_two());
    }
}

use regex::Regex;
use rustvent2022::get_input;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

enum DirectoryEntry {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

struct Directory {
    pub entries: Vec<DirectoryEntry>,
    pub name: String,
    pub parent: Option<Rc<RefCell<Directory>>>,
}

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
}

impl FromStr for Directory {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parent
        let parent = Rc::new(RefCell::new(Directory {
            entries: Vec::new(),
            name: String::new(),
            parent: None,
        }));

        let mut current = Rc::clone(&parent);

        for line in s.lines() {
            println!("{line}");
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
                                name: String::from(&line[5..]),
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

        Ok(Rc::try_unwrap(parent)
            .or(Err("Can't move out of parent RC"))?
            .into_inner())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2022", "7");
    // println!("Solution part one: {}", part(&input, 4));
    // println!("Solution part two: {}", part(&input, 14));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "$ cd /
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
        let dir = Directory::from_str(input).unwrap();

        assert_eq!(dir.entries.len(), 4);
        match &dir.entries[0] {
            DirectoryEntry::File(_) => panic!(),
            DirectoryEntry::Directory(d) => assert_eq!("a", &d.borrow().name),
        }
    }
}

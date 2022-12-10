use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::iter;

#[derive(Debug, Clone)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> File<'a> {
    fn new(size: usize, name: &str) -> File {
        File { size, name }
    }
}

#[derive(Debug, Clone)]
struct Directory<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    directories: Vec<Directory<'a>>,
}

impl std::fmt::Display for Directory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nDirectory {{ name: {}, files: {:?}, directories: {:?} }}",
            self.name, self.files, self.directories
        )
    }
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Directory<'a> {
        Directory {
            name,
            files: vec![],
            directories: vec![],
        }
    }

    fn add_file(&mut self, file: File<'a>) {
        self.files.push(file);
    }

    fn add_directory(&mut self, directory: Directory<'a>) {
        self.directories.push(directory);
    }

    fn size(&self) -> usize {
        let files_size: usize = self.files.iter().map(|f| f.size).sum();
        let directories_size: usize = self.directories.iter().map(|d| d.size()).sum();
        files_size + directories_size
    }
}

pub fn parse_input(path: &str) -> Vec<usize> {
    let content = read_to_string(path).unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let mut dirs: Vec<Directory> = vec![];
    let mut sizes: Vec<_> = vec![];
    for l in &lines {
        let re = Regex::new(r"\$ (\w+) (.*)").unwrap();
        let caps = re.captures(l);
        match caps {
            Some(c) => {
                let cmd = c.get(1).unwrap().as_str();
                let args = c.get(2).unwrap().as_str();
                if cmd == "cd" && args != ".." {
                    let dir = Directory::new(args);
                    dirs.push(dir);
                }
                if cmd == "cd" && args == ".." {
                    let prev = dirs.pop().unwrap();
                    let size = prev.size();
                    dirs.last_mut().unwrap().add_directory(prev);
                    sizes.push(size);
                }
            }
            None => {
                let file = Regex::new(r"(\d+) ").unwrap();
                let caps = file.captures(l);
                match caps {
                    Some(c) => {
                        let size = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let file = File::new(size, "test");
                        let prev = dirs.last_mut().unwrap();
                        let name = prev.name;
                        prev.add_file(file);
                    }
                    None => {}
                }
            }
        }
    }

    while dirs.len() > 0 {
        let prev = dirs.pop().unwrap();
        let prev_size = prev.size();
        sizes.push(prev_size);

        if dirs.len() > 0 {
            dirs.last_mut().unwrap().add_directory(prev);
        }

        // println!("dir {} has size {} and dirs {:?}", name, prev_size, names);
    }

    sizes
}

pub fn get_smallest_sum(file: &str) -> usize {
    let sizes = parse_input(file);
    sizes.iter().filter(|&x| *x < 100000).sum()
}

pub fn get_biggest_sum(file: &str) -> usize {
    let sizes = parse_input(file);
    let total_space = 70000000;
    let min_amount = 30000000;
    let root_size = sizes.last().unwrap();
    let remaining_space = total_space - root_size;
    let delta = min_amount - remaining_space;
    sizes.iter().filter(|&x| *x >= delta).min().unwrap().clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_directory() {
        let mut dir = Directory::new("foo");
        let file = File::new(5, "test");
        dir.add_file(file);
        assert_eq!(dir.files.len(), 1);

        let mut parent_dir = Directory::new("bar");
        let one_more_file = File::new(25, "test");
        parent_dir.add_file(one_more_file);
        parent_dir.add_directory(dir);
        assert_eq!(parent_dir.directories.len(), 1);
        assert_eq!(parent_dir.directories[0].files.len(), 1);
        assert_eq!(parent_dir.directories[0].files[0].name, "test");
        assert_eq!(parent_dir.size(), 30);
    }

    #[test]
    fn test_day5_part1() {
        let file = "src/specs/day7";
        let result = get_smallest_sum(file);
        assert_eq!(result, 95437);
    }

    fn test_day5_part2() {
        let file = "src/specs/day7";
        let result = get_biggest_sum(file);
        assert_eq!(result, 24933642);
    }
}

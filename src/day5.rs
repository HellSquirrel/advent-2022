use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::iter;

pub fn get_marker(path: &str, uniq_size: usize) -> i32 {
    let file = read_to_string(path).expect("Unable to read file");
    for i in 0..file.len() - uniq_size {
        let substr = &file[i..i + uniq_size];
        let u_len = HashSet::<char>::from_iter(substr.chars()).len();
        if u_len == uniq_size {
            return (i + uniq_size) as i32;
        }
    }

    -10
}

pub fn parse_input(path: &str, should_reverse: bool) -> String {
    let file = read_to_string(path).expect("Unable to read file");
    let (first, second) = file.split_at(file.find("\n\n").unwrap());
    let mut first_lines = first.split('\n');
    let numbers = first_lines
        .next_back()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok());
    let mut crates: HashMap<usize, Vec<char>> =
        HashMap::from_iter(numbers.zip(iter::repeat(vec![])).into_iter());

    for l in first_lines {
        for (num, chars) in l.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            let char = chars[1];
            if char != ' ' {
                crates.entry(num + 1).and_modify(|v| v.insert(0, char));
            }
        }
    }

    let regexp = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let commands = second.split('\n').filter_map(|s| {
        let caps = regexp.captures(s)?;
        let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        Some((count, from, to))
    });

    for c in commands {
        let (count, from, to) = c;
        let stack = crates.get(&from).unwrap().clone();
        let (remaining, moved) = stack.split_at(stack.len() - count);
        crates.insert(from, remaining.to_vec());
        let mut new_vec = moved.to_vec();
        if should_reverse {
            new_vec.reverse();
        }
        crates.entry(to).and_modify(|v| v.extend(new_vec));
    }

    let mut keys = crates.keys().cloned().collect::<Vec<usize>>();
    keys.sort();
    keys.iter()
        .map(|k| crates.get(k).unwrap().last().unwrap_or(&' ').to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day5() {
        let file = "src/specs/day5";
        let result = parse_input(file, true);
        assert_eq!(result, "CMZ");
    }
}

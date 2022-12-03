use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

fn get_priorities() -> HashMap<char, usize> {
    let mut items_with_priorities = (b'a'..=b'z')
        .map(char::from)
        .zip(1..)
        .collect::<HashMap<char, usize>>();

    let items_with_priorities_higher = (b'A'..=b'Z')
        .map(char::from)
        .zip(27..)
        .collect::<HashMap<char, usize>>();
    items_with_priorities.extend(items_with_priorities_higher);
    items_with_priorities
}

pub fn calculate_priorities_part1(path: &str) -> usize {
    let items_with_priorities = get_priorities();

    let file = read_to_string(path).expect("Unable to read file");
    file.split("\n").into_iter().fold(0, |acc, s| {
        let len = s.len() / 2;
        let hs1: HashSet<char> = HashSet::from_iter(s[0..len].chars());
        let hs2: HashSet<char> = HashSet::from_iter(s[len..].chars());
        let sum_per_compartment = hs1.intersection(&hs2).into_iter().fold(0, |acc, c| {
            *items_with_priorities.get(c).unwrap_or(&0) + acc
        });

        acc + sum_per_compartment
    })
}

pub fn calculate_priorities_part2(path: &str) -> usize {
    let items_with_priorities = get_priorities();

    let file = read_to_string(path).expect("Unable to read file");
    let hash_vec = file
        .split('\n')
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .collect::<Vec<HashSet<char>>>();

    hash_vec
        .chunks(3)
        .flat_map(|chunk| {
            chunk.into_iter().fold(HashSet::<char>::new(), |acc, hs| {
                if acc.is_empty() {
                    hs.clone()
                } else {
                    acc.intersection(hs).cloned().collect()
                }
            })
        })
        .map(|c| *items_with_priorities.get(&c).unwrap_or(&0))
        .fold(0, |acc, i| acc + i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let file = "src/specs/day3";
        let result = calculate_priorities_part1(file);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_day3_part2() {
        let file = "src/specs/day3";
        let result = calculate_priorities_part2(file);
        assert_eq!(result, 70);
    }
}

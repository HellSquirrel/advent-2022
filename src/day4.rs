use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

pub fn fully_overlaps(split: Vec<usize>) -> bool {
    let l1 = split[1] - split[0] + 1;
    let l2 = split[3] - split[2] + 1;
    if l1 >= l2 {
        return (split[0] <= split[2]) && (split[1] >= split[3]);
    }
    (split[2] <= split[0]) && (split[3] >= split[1])
}

pub fn partially_overlaps(split: Vec<usize>) -> bool {
    if split[0] <= split[2] && split[2] <= split[1] {
        return true;
    }

    if (split[0] <= split[3]) && (split[3] <= split[1]) {
        return true;
    }

    if (split[2] <= split[0]) && (split[0] <= split[3]) {
        return true;
    }

    if (split[2] <= split[1]) && (split[1] <= split[3]) {
        return true;
    }

    return false;
}

pub fn get_intersect_ranges<F>(path: &str, mut checker: F) -> usize
where
    F: FnMut(Vec<usize>) -> bool,
{
    let file = read_to_string(path).expect("Unable to read file");
    file.split('\n')
        .map(|str| {
            let split: Vec<usize> = str
                .split(',')
                .flat_map(|s| s.split('-'))
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            checker(split)
        })
        .filter(|s| *s)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day4_part1() {
        let file = "src/specs/day4";
        let result = get_intersect_ranges(file, fully_overlaps);
        assert_eq!(result, 2);
    }

    fn test_day4_part2() {
        let file = "src/specs/day4";
        let result = get_intersect_ranges(file, partially_overlaps);
        assert_eq!(result, 4);
    }
}

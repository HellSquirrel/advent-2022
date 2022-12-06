use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day6() {
        let file = "src/specs/day6";
        let result = get_marker(file, 14);
        assert_eq!(result, 23);
    }
}

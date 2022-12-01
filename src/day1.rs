use std::env;
use std::fs::read_to_string;

pub fn get_elf_and_energy(path: &str, total_elves: usize) -> (usize, Vec<usize>) {
    let file = read_to_string(path).expect("Unable to read file");
    let mut lines = file
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .into_iter()
                .fold(0, |acc, s| acc + s.parse::<usize>().unwrap_or(0))
        })
        .zip(0..)
        .collect::<Vec<(usize, usize)>>();

    lines.sort_by(|a, b| b.0.cmp(&a.0));
    lines[0..total_elves]
        .iter()
        .fold((0, Vec::new()), |(sum, mut elves), (calories, elf)| {
            elves.push(*elf);
            (sum + calories, elves)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day1() {
        let file = "src/specs/day1";
        let result = get_elf_and_energy(file, 1);
        assert_eq!(result, (24000, vec![3]));
    }
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn check_probe(counter: usize, value: i32, probes: &mut Vec<usize>) -> i32 {
    if probes.len() == 0 {
        return 0;
    };
    let probe = probes.pop().unwrap();
    if counter == probe - 1 {
        return value * probe as i32;
    }
    probes.push(probe);
    return 0;
}

pub fn check_intersection(counter: usize, value: i32) -> bool {
    (counter % 40) as i32 <= value + 1 && (counter % 40) as i32 >= value - 1
}

pub fn parse_input(path: &str) -> (i32, String) {
    let file = File::open(path).expect("Unable to open file");
    let mut counter = 0;
    let mut value = 1;
    let mut cycles: Vec<(usize, i32)> = vec![];
    let mut probes = vec![20, 60, 100, 140, 180, 220];
    let mut result = 0;
    probes.reverse();

    cycles.push((counter, value));

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        if line == "noop" {
            counter += 1;
            cycles.push((counter, value));
            result += check_probe(counter, value, &mut probes);
        } else {
            let next_value = line[5..].parse::<i32>().unwrap();
            counter += 1;
            cycles.push((counter, value));
            result += check_probe(counter, value, &mut probes);

            counter += 1;
            value += next_value;
            cycles.push((counter, value));
            result += check_probe(counter, value, &mut probes);
        }
    }

    let pattern = cycles
        .iter()
        .map(|(counter, value)| {
            if check_intersection(*counter, *value) {
                return "#";
            }

            "."
        })
        .collect::<Vec<&str>>()
        .join("");

    // println!("{}", pattern);

    (result, pattern)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day10_part1() {
        let file = "src/specs/day10";
        let result = parse_input(file);
        assert_eq!(result.0, 13140);
    }

    #[test]
    fn test_day10_part2() {
        let file = "src/specs/day10";
        let result = parse_input(file);
        let out = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######......";

        println!("{}", out);
        assert_eq!(result.1, out);
    }
}

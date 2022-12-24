use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_input(path: &str) -> Vec<(i64, bool)> {
    let file = File::open(path).unwrap();
    let mut i = 0;
    let mut result = io::BufReader::new(file)
        .lines()
        .map(|line| (line.unwrap().parse::<i64>().unwrap(), false))
        .collect::<Vec<(i64, bool)>>();

    result
}


pub fn part_1(path: &str) -> i64 {
    let mut result = parse_input(path);
    let mut pos = 0;

    while pos < result.len() {
        let idx = result.iter().position(|e| e.1 == false).unwrap();
        let value = result[idx].0;

        // println!("moving value: {}", value);
        let mut next_position = idx as i64 + value;
        if next_position > (result.len() - 1) as i64 {
            next_position = next_position % (result.len() - 1) as i64;
        }

        if next_position <= 0 && value != 0 {
            next_position = result.len() as i64 + (next_position % (result.len() - 1) as i64) - 1;
        }

        // println!("next position: {}", next_position);

        result.remove(idx);
        result.insert(next_position as usize, (value, true));
        pos += 1;

        // println!("result: {:?}", &result);
    }

    let zero_pos = result.iter().position(|e| e.0 == 0).unwrap();
    let pos_1000 = (zero_pos + 1000) % result.len();
    let pos_2000 = (zero_pos + 2000) % result.len();
    let pos_3000 = (zero_pos + 3000) % result.len();

    // println!("final result: {:?}", result);
    [pos_1000, pos_2000, pos_3000]
        .iter()
        .map(|p| result[*p].0)
        .sum()
}

const KEY: i64 = 811589153;

pub fn part_2(path: &str) -> i64 {
    let mut result = parse_input(path).iter().enumerate().map(|(index, v)| (v.0 * KEY, index)).collect::<Vec<(i64, usize)>>();

    for i in 0..10 {
        for i in 0..result.len() {
            let idx = result.iter().position(|e| e.1 == i).unwrap();
            let value = result[idx].0;

            // println!("moving value: {}", value);
            let mut next_position = idx as i64 + value;
            if next_position > (result.len() - 1) as i64 {
                next_position = next_position % (result.len() - 1) as i64;
            }

            if next_position <= 0 && value != 0 {
                next_position = result.len() as i64 + (next_position % (result.len() - 1) as i64) - 1;
            }

            // println!("next position: {}", next_position);

            result.remove(idx);
            result.insert(next_position as usize, (value, i));
        }

        // println!("");
        // println!("round {i}");
        // println!("result: {:?}", &result.iter().map(|r| r.0).collect::<Vec<_>>());

    }

    let zero_pos = result.iter().position(|e| e.0 == 0).unwrap();
    let pos_1000 = (zero_pos + 1000) % result.len();
    let pos_2000 = (zero_pos + 2000) % result.len();
    let pos_3000 = (zero_pos + 3000) % result.len();

    // println!("final result: {:?}", result);
    [pos_1000, pos_2000, pos_3000]
        .iter()
        .map(|p| result[*p].0)
        .sum()
}

#[cfg(test)]
#[test]
fn test_part_1() {
    let input = part_1("src/specs/day20");
    assert_eq!(input, 3);
}

#[test]
fn test_part_2() {
    let input = part_2("src/specs/day20");
    assert_eq!(input, 1623178306);
}

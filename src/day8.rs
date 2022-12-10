use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_input(path: &str) -> Vec<Vec<usize>> {
    let file = File::open(path).expect("Unable to open file");
    io::BufReader::new(file)
        .lines()
        .map(|s| {
            s.unwrap()
                .split("")
                .into_iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

pub fn get_score(forest: &Vec<Vec<usize>>, (i, j): (usize, usize)) -> usize {
    let place_for_the_house = forest[i][j];
    let width = forest[0].len();
    let height = forest.len();

    let mut ts = 0;
    let mut bs = 0;
    let mut ls = 0;
    let mut rs = 0;

    // top score
    let mut row = i as i32 - 1;

    loop {
        if row < 0 {
            break;
        }

        let current = forest[row as usize][j];
        if current < place_for_the_house {
            ts += 1;
        } else {
            ts += 1;
            break;
        }

        row -= 1;
    }

    println!("ts: {}", ts);

    row = i as i32 + 1;

    loop {
        if row > height as i32 - 1 {
            break;
        }

        let current = forest[row as usize][j];
        if current < place_for_the_house {
            bs += 1;
        } else {
            bs += 1;
            break;
        }

        row += 1;
    }

    println!("bs: {}", bs);

    // left score
    let mut col = j as i32 - 1;
    loop {
        if col < 0 {
            break;
        }

        println!(
            "col: {}, i: {}, current: {}",
            col, i, forest[i][col as usize]
        );

        let current = forest[i][col as usize];
        if current < place_for_the_house {
            ls += 1;
        } else {
            ls += 1;
            break;
        }

        col -= 1;
    }

    println!("ls: {}", ls);

    // right score

    col = j as i32 + 1;
    loop {
        if col > width as i32 - 1 {
            break;
        }

        let current = forest[i][col as usize];
        // println!("current: {}, i, col: ({}, {})", current, i, col);
        if current < place_for_the_house {
            rs += 1;
        } else {
            rs += 1;
            break;
        }

        col += 1;
    }

    println!("rs: {}", rs);

    ts * bs * ls * rs
}

pub fn count_edge_trees(path: &str) -> usize {
    let forest = parse_input(path);
    let mut visible_top = (0..forest[0].len()).map(|i| (0, i)).collect::<Vec<_>>();
    let mut visible_left = (0..forest.len()).map(|i| (i, 0)).collect::<Vec<_>>();
    let mut visible_right = (0..forest.len())
        .map(|i| (i, forest[0].len() - 1))
        .collect::<Vec<_>>();
    let mut visible_bottom = (0..forest[0].len())
        .map(|i| (forest.len() - 1, i))
        .collect::<Vec<_>>();

    let mut highest_top = forest[0].clone();
    let mut highest_left = forest.iter().map(|v| v[0]).collect::<Vec<_>>();
    let mut highest_right = forest
        .iter()
        .map(|v| v[forest[0].len() - 1])
        .collect::<Vec<_>>();
    let mut highest_bottom = forest.last().unwrap().clone();

    for i in 1..forest.len() - 1 {
        for j in 1..forest[0].len() - 1 {
            let current = forest[i][j];
            if current > highest_top[j] {
                visible_top.push((i, j));
                highest_top[j] = current
            }

            if current > highest_left[i] {
                visible_left.push((i, j));
                highest_left[i] = current;
            }
        }
    }

    for i in (1..forest.len() - 1).into_iter().rev() {
        for j in (1..forest[0].len() - 1).into_iter().rev() {
            let current = forest[i][j];
            if current > highest_bottom[j] {
                visible_bottom.push((i, j));

                highest_bottom[j] = current
            }

            if current > highest_right[i] {
                visible_right.push((i, j));
                highest_right[i] = current;
            }
        }
    }

    let mut visible = vec![];
    visible.append(&mut visible_top);
    visible.append(&mut visible_left);
    visible.append(&mut visible_right);
    visible.append(&mut visible_bottom);

    let result = HashSet::<(usize, usize)>::from_iter(visible);

    result.len()
}

pub fn get_scenic_score(path: &str) -> usize {
    let mut max = 0;
    let parsed = parse_input(path);
    for i in 1..parsed.len() - 1 {
        for j in 1..parsed[0].len() - 1 {
            let score = get_score(&parsed, (i, j));
            println!("({}, {}) score: {}", i, j, score);
            if score > max {
                max = score;
            }
        }
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day8_part1() {
        let file = "src/specs/day8";
        let result = count_edge_trees(file);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_get_score() {
        let file = "src/specs/day8";
        let result = parse_input(file);
        assert_eq!(get_score(&result, (1, 1)), 1);
        assert_eq!(get_score(&result, (1, 2)), 4);
        assert_eq!(get_score(&result, (3, 2)), 8);
        assert_eq!(get_score(&result, (0, 0)), 0);
    }

    #[test]
    fn test_day8_part2() {
        let file = "src/specs/day8";
        let result = get_scenic_score(file);
        assert_eq!(result, 8);
    }
}

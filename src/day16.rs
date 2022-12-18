use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{stdout, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: String, flow_rate: i32, tunnels: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            tunnels,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    valves: HashMap<String, bool>,
}

impl State {
    fn new(valves: &Vec<String>) -> Self {
        Self {
            valves: valves
                .iter()
                .map(|v| (v.clone(), false))
                .collect::<HashMap<String, bool>>(),
        }
    }

    fn open(&self, name: String) -> Self {
        let mut new_state = self.valves.clone();
        new_state.insert(name, true);
        Self { valves: new_state }
    }

    fn is_opened(&self, name: &str) -> bool {
        *self.valves.get(name).unwrap()
    }

    fn gas_for(&self, count: i32, maze: &Maze) -> i32 {
        let mut acc = 0;
        for (v, is_opened) in &self.valves {
            let flow = maze.get_flow(v.as_str());
            if *is_opened {
                acc += count * flow as i32;
            }
        }

        acc
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut acc = String::new();
        for (v, is_opened) in &self.valves {
            acc.push_str(&format!(
                "{v}:{}",
                if *is_opened { "O" } else { "X" },
                v = v
            ));
        }

        write!(f, "{}", acc)
    }
}

#[derive(Debug)]
pub struct Maze {
    data: HashMap<String, Valve>,
}

impl Maze {
    fn count_steps(&self, start: &str, end: &str) -> i32 {
        let BIIG = 100000;
        let mut unvisited: HashMap<&str, i32> =
            self.data.iter().map(|(k, v)| (k.as_str(), BIIG)).collect();
        let mut visited: HashMap<&str, i32> = HashMap::new();

        unvisited.insert(start, 0);

        loop {
            let copy = unvisited.clone();
            let min = copy.iter().min_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();

            if min.0 == &end {
                return *min.1;
            }

            unvisited.remove(&*min.0.to_string()).unwrap();
            visited.insert(min.0, *min.1);

            let edges = &self.data.get(&min.0.to_string()).unwrap().tunnels;

            for e in edges {
                if let Some(old_dist) = &unvisited.get(&*e.to_string()) {
                    let new_dist = min.1 + 1;
                    if &new_dist < old_dist {
                        unvisited.insert(e, new_dist);
                    }
                }
            }
        }

        BIIG
    }

    fn new(data: HashMap<String, Valve>) -> Self {
        Self { data }
    }

    fn get_flow(&self, name: &str) -> i32 {
        self.data.get(name).unwrap().flow_rate
    }

    pub fn get_best_score(&self) -> usize {
        let steps = 30;
        let keys: Vec<String> = self.data.keys().map(|k| k.clone()).collect();
        let mut initial_state = State::new(&keys);
        initial_state = initial_state.open("AA".to_string());

        let mut options: Vec<Vec<(String, i32, State)>> = Vec::with_capacity(steps);
        let first = keys
            .iter()
            .map(|k| {
                (
                    (*k).clone(),
                    if k == "AA" { 0 } else { -100000 },
                    initial_state.clone(),
                )
            })
            .collect::<Vec<_>>();
        options.push(first);

        for i in 1..steps {
            let mut next_options = vec![];

            for name in keys.iter() {
                let mut best_state = initial_state.clone();
                let mut best_name = String::new();
                let mut best = -100000;

                // println!("checking name {}", name);
                for (index, other_name) in keys.iter().enumerate() {
                    let distance = self.count_steps(name, other_name) as usize;
                    // println!("distance from {} to {} is {}", name, other_name, distance);

                    if (i as i32) - (distance as i32) - 1 < 0 {
                        continue;
                    }

                    // println!("distance from {} to {} is {}", name, other_name, distance);

                    let prev_cell = options[(i - distance - 1) as usize][index].clone();
                    let (delta, additional_steps) = if prev_cell.2.is_opened(name) {
                        (0, 0)
                    } else {
                        (self.get_flow(name), 1)
                    };

                    let score = prev_cell.1
                        + prev_cell
                            .2
                            .gas_for((distance + additional_steps) as i32, self)
                        + delta as i32;
                    let new_state = prev_cell.2.open(name.clone());

                    // println!("score for {} is {}", other_name, score);

                    if score > best {
                        best = score;
                        best_state = new_state;
                        best_name = other_name.clone();
                    }
                }

                // println!("picking {} for {}", best_name, name);

                next_options.push((name.clone(), best, best_state));
            }

            options.push(next_options);
        }

        options
            .last()
            .unwrap()
            .iter()
            .max_by(|(_, s1, _), (_, s2, _)| s1.cmp(s2))
            .unwrap()
            .1 as usize
    }

    pub fn get_best_score_for_both(&self) -> usize {
        let steps = 26;
        let keys: Vec<String> = self.data.keys().map(|k| k.clone()).collect();

        let key_pairs_set: HashSet<(String, String)> = keys
            .iter()
            .flat_map(|k| self.data.keys().map(|k1| (k.clone(), k1.clone())))
            .collect();

        let key_pairs: Vec<(String, String)> = key_pairs_set.iter().map(|v| (*v).clone()).collect();

        let mut initial_state = State::new(&keys);
        initial_state = initial_state.open("AA".to_string());

        let mut options: Vec<Vec<((String, String), i32, State)>> = Vec::with_capacity(steps);
        let first = key_pairs
            .iter()
            .map(|(k1, k2)| {
                (
                    ((*k1).clone(), (*k2).clone()),
                    if (k1, k2) == (&"AA".to_string(), &"AA".to_string()) {
                        0
                    } else {
                        -100000
                    },
                    initial_state.clone(),
                )
            })
            .collect::<Vec<_>>();

        options.push(first);

        for i in 1..steps {
            // println!("step {}", i);
            let mut next_options = vec![];

            let mut best_state = initial_state.clone();
            let mut best_score = -100000;
            for (name1, name2) in key_pairs.iter() {
                // println!("checking name {}", name);
                for (index, (other_name1, other_name2)) in key_pairs.iter().enumerate() {
                    // human loop
                    let distance1 = self.count_steps(name1, other_name1) as usize;

                    if (i as i32) - (distance1 as i32) - 1 < 0 {
                        continue;
                    }

                    let distance2 = self.count_steps(name2, other_name2) as usize;

                    if (i as i32) - (distance2 as i32) - 1 < 0 {
                        continue;
                    }

                    // println!("distance from {} to {} is {}", name, other_name, distance);

                    let prev_cell = options[(i - distance1 - 1) as usize][index].clone();

                    let (delta1, additional_steps1) = if prev_cell.2.is_opened(name1) {
                        (0, 0)
                    } else {
                        (self.get_flow(name1), 1)
                    };

                    let (delta2, additional_steps2) = if prev_cell.2.is_opened(name2) {
                        (0, 0)
                    } else {
                        (self.get_flow(name2), 1)
                    };

                    let score = prev_cell.1 + prev_cell.2.gas_for(distance1 as i32, self);

                    let mut new_state = prev_cell.2.open(name1.clone());
                    new_state = new_state.open(name2.clone());

                    if score > best_score {
                        best_score = score;
                        best_state = new_state;
                    }
                }

                // println!("picking {} for {}", best_name, name);

                next_options.push((
                    (name1.clone(), name2.clone()),
                    best_score,
                    best_state.clone(),
                ));
            }

            options.push(next_options);
        }

        options
            .last()
            .unwrap()
            .iter()
            .max_by(|(_, s1, _), (_, s2, _)| s1.cmp(s2))
            .unwrap()
            .1 as usize
    }
}

pub fn parse_input(path: &str) -> Maze {
    let file = File::open(path).expect("Unable to open file");

    let graph = io::BufReader::new(file)
        .lines()
        .filter_map(|i| i.ok())
        .filter_map(|l| {
            let regexp = Regex::new(
                r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((:?\w\w(:?, )?)+)",
            )
            .unwrap();
            let captures = regexp.captures(&l).map(|cap| {
                let mut name = cap.get(1).unwrap().as_str().to_string();
                let mut flow_rate = cap
                    .get(2)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let mut tunnels = cap
                    .get(3)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                return (name.clone(), Valve::new(name, flow_rate, tunnels));
            });
            captures
        })
        .collect::<HashMap<String, Valve>>();

    Maze::new(graph)
}

pub fn part_1(path: &str) -> usize {
    let maze = parse_input(path);
    maze.get_best_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze() {
        let res = parse_input("src/specs/day16");
        assert_eq!(res.count_steps("BB", "AA"), 1);
        assert_eq!(res.count_steps("AA", "JJ"), 2);
        assert_eq!(res.count_steps("DD", "HH"), 4);
    }

    #[test]
    fn test_best_score() {
        let res = parse_input("src/specs/day16");
        let score = res.get_best_score();

        assert_eq!(score, 1651);
    }

    // #[test]
    // fn test_best_score_for_both() {
    //     let res = parse_input("src/specs/day16");
    //     let score = res.get_best_score_for_both();

    //     assert_eq!(score, 1651);
    // }
}

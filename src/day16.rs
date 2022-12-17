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

#[derive(Debug)]
struct Maze {
    data: HashMap<String, Valve>,
}

impl Maze {
    fn count_steps(&self, start: &str, end: &str) -> usize {
        let mut queue: Vec<(&str, usize)> = Vec::new();
        queue.push((end, 0));

        loop {
            let (el, count) = queue.pop().unwrap();
            if el == start {
                return count;
            }

            println!("El: {}", el);
            println!("Count: {}", count);
            println!("data {:?}", self.data);

            let edges = &self.data.get(&el.to_string()).unwrap().tunnels;
            for e in edges {
                queue.push((e.as_str(), count + 1))
            }
        }

        100000
    }

    fn new(data: HashMap<String, Valve>) -> Self {
        Self { data }
    }
}

fn parse_input(path: &str) -> Maze {
    let file = File::open(path).expect("Unable to open file");

    let graph = io::BufReader::new(file)
        .lines()
        .filter_map(|i| i.ok())
        .filter_map(|l| {
            println!("Line: {}", l);
            let regexp = Regex::new(
                r"Valve (\w\w) has flow rate=(\d+); tunnels lead to valves ((:?\w\w(:?, )?)+)",
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

fn open_valve(valve: HashMap<String, Valve>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze() {
        let res = parse_input("src/specs/day16");
        assert_eq!(res.count_steps("AA", "BB"), 1);
        assert_eq!(res.count_steps("AA", "JJ"), 2);
    }
}

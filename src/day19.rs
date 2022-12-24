use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Blueprint {
    title: i32,
    robots: HashMap<Robots, Stock>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Robots {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Resources {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Stock {
    hash: HashMap<Resources, i32>,
}

impl Stock {
    pub fn new(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Stock {
        Stock {
            hash: HashMap::from_iter(vec![
                (Resources::Ore, ore),
                (Resources::Clay, clay),
                (Resources::Obsidian, obsidian),
                (Resources::Geode, geode),
            ]),
        }
    }

    pub fn add(&self, stock: Stock) -> Stock {
        let mut new_stock = stock.clone();
        for (resource, value) in self.hash.iter() {
            let in_new_stock = new_stock.hash.get(resource).unwrap();
            new_stock
                .hash
                .insert(resource.clone(), in_new_stock + value);
        }

        new_stock
    }

    pub fn get_ore(&self) -> i32 {
        self.hash.get(&Resources::Ore).unwrap().clone()
    }

    pub fn get_clay(&self) -> i32 {
        self.hash.get(&Resources::Clay).unwrap().clone()
    }

    pub fn get_obsidian(&self) -> i32 {
        self.hash.get(&Resources::Obsidian).unwrap().clone()
    }

    pub fn get_geode(&self) -> i32 {
        self.hash.get(&Resources::Geode).unwrap().clone()
    }

    pub fn sub(&self, stock: Stock) -> Option<Stock> {
        let mut new_stock = stock.clone();
        for (resource, value) in self.hash.iter() {
            let in_new_stock = new_stock.hash.get(resource).unwrap();
            if in_new_stock > value {
                return None;
            }
            new_stock
                .hash
                .insert(resource.clone(), value - in_new_stock);
        }

        Some(new_stock)
    }
}

const PRIORITY: [Resources; 4] = [
    Resources::Geode,
    Resources::Obsidian,
    Resources::Clay,
    Resources::Ore,
];
const ROBOTS_PRIORITY: [Robots; 4] = [Robots::Geode, Robots::Obsidian, Robots::Clay, Robots::Ore];

impl PartialOrd for Stock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for res in PRIORITY.iter() {
            let self_value = self.hash.get(res).unwrap();
            let other_value = other.hash.get(res).unwrap();
            if self_value > other_value {
                println!("{} > {}, {:?}", self_value, other_value, res);
                return Some(Ordering::Greater);
            } else if self_value < other_value {
                return Some(Ordering::Less);
            }
        }

        return Some(Ordering::Equal);
    }
}

impl Ord for Stock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scenario {
    resources: Stock,
    steps: i32,
    robots: HashMap<Robots, i32>,
    blueprint: Blueprint,
    history: Vec<(i32, Robots)>,
}

impl Scenario {
    pub fn new(blueprint: Blueprint) -> Scenario {
        Scenario {
            resources: Stock::new(0, 0, 0, 0),
            steps: 0,
            robots: HashMap::from_iter(vec![
                (Robots::Clay, 0),
                (Robots::Obsidian, 0),
                (Robots::Geode, 0),
                (Robots::Ore, 1),
            ]),
            blueprint,
            history: Vec::new(),
        }
    }

    pub fn collect(&mut self) {
        for (robot, count) in self.robots.iter() {
            match robot {
                Robots::Ore => {
                    self.resources = self.resources.add(Stock::new(*count, 0, 0, 0));
                }
                Robots::Clay => {
                    self.resources = self.resources.add(Stock::new(0, *count, 0, 0));
                }
                Robots::Obsidian => {
                    self.resources = self.resources.add(Stock::new(0, 0, *count, 0));
                }
                Robots::Geode => {
                    self.resources = self.resources.add(Stock::new(0, 0, 0, *count));
                }
            }
        }
    }

    pub fn get_output_for_configuration(configuration: &HashMap<Robots, i32>, steps: i32) -> Stock {
        let mut stock = Stock::new(0, 0, 0, 0);

        for (robot, count) in configuration {
            match robot {
                Robots::Ore => {
                    stock = stock.add(Stock::new(*count * steps, 0, 0, 0));
                }
                Robots::Clay => {
                    stock = stock.add(Stock::new(0, *count * steps, 0, 0));
                }
                Robots::Obsidian => {
                    stock = stock.add(Stock::new(0, 0, *count * steps, 0));
                }
                Robots::Geode => {
                    stock = stock.add(Stock::new(0, 0, 0, *count * steps));
                }
            }
        }

        stock
    }

    pub fn available_options(&self) -> Vec<(Robots, Stock)> {
        let mut options = vec![];
        for robot in ROBOTS_PRIORITY.iter() {
            let cost = self.blueprint.robots.get(robot).unwrap();
            let new_stock = self.resources.clone().sub(cost.clone());
            if new_stock.is_some() {
                options.push((robot.clone(), new_stock.unwrap().clone()));
            }
        }
        options
    }

    pub fn step(&mut self) -> Vec<Scenario> {
        self.steps += 1;
        let available_options = self.available_options();
        self.collect();
        available_options
            .iter()
            .map(|(robot, stock)| {
                let mut history = self.history.clone();
                let mut scenario = Scenario {
                    resources: stock.clone(),
                    steps: self.steps,
                    robots: self.robots.clone(),
                    blueprint: self.blueprint.clone(),
                    history,
                };

                scenario.collect();
                scenario.history.push((self.steps, robot.clone()));
                scenario
                    .robots
                    .insert(robot.clone(), scenario.robots.get(robot).unwrap() + 1);
                scenario
            })
            .collect()
    }
}

pub fn parse_input(path: &str) -> Vec<Blueprint> {
    let file = File::open(path).expect("Unable to open file");
    let lines = io::BufReader::new(file).lines().into_iter();

    let mut parsed: Vec<Blueprint> = vec![];

    for l in lines {
        let line = l.unwrap();

        let regex = Regex::new(r"Blueprint (\d+): .*");

        let caps = regex.unwrap().captures(&line).unwrap();
        let title = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

        let ore = Regex::new(r"Each ore robot costs (\d+) ore");
        let clay = Regex::new(r"Each clay robot costs (\d+) ore");
        let obsidian = Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay");
        let geode = Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian");

        let ore = ore.unwrap().captures(&line).unwrap();
        let clay = clay.unwrap().captures(&line).unwrap();
        let obsidian = obsidian.unwrap().captures(&line).unwrap();
        let geode = geode.unwrap().captures(&line).unwrap();

        let ore_for_ore = ore.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let ore_for_clay = clay.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let ore_for_obsidian = obsidian.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let clay_for_obsidian = obsidian.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let ore_for_geode = geode.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let obsidian_for_geode = geode.get(2).unwrap().as_str().parse::<i32>().unwrap();

        let blueprint = Blueprint {
            title,
            robots: vec![
                (Robots::Ore, Stock::new(ore_for_ore, 0, 0, 0)),
                (Robots::Clay, Stock::new(ore_for_clay, 0, 0, 0)),
                (
                    Robots::Obsidian,
                    Stock::new(ore_for_obsidian, clay_for_obsidian, 0, 0),
                ),
                (
                    Robots::Geode,
                    Stock::new(ore_for_geode, 0, obsidian_for_geode, 0),
                ),
            ]
            .into_iter()
            .collect(),
        };

        parsed.push(blueprint);
    }

    parsed
}

fn test_blueprint(blueprint: Blueprint) -> usize {
    let scenario = Scenario::new(blueprint);
    let mut scenarios: Vec<Scenario> = vec![scenario];
    let mut step = 0;

    while step < 24 {
        let has_clay = &scenarios
            .iter()
            .find(|s| s.robots.get(&Robots::Clay) != Some(&0));

        if has_clay.is_some() {
            scenarios = scenarios
                .iter()
                .filter(|s| s.robots.get(&Robots::Clay) != Some(&0))
                .map(|s| s.clone())
                .collect();
            println!("scenarios {:?}", &scenarios.len());
        }


        let has_obsidian = &scenarios
            .iter()
            .find(|s| s.robots.get(&Robots::Obsidian) != Some(&0));

        if has_obsidian.is_some() {
            scenarios = scenarios
                .iter()
                .filter(|s| s.robots.get(&Robots::Obsidian) != Some(&0))
                .map(|s| s.clone())
                .collect();
            println!("scenarios {:?}", &scenarios.len());
        }

        let has_geode = &scenarios
            .iter()
            .find(|s| s.robots.get(&Robots::Geode) != Some(&0));

        // if has_geode.is_some() {
        //     println!("has geode on step {}", step);
        //     println!("scenario {:?}", has_geode.unwrap());
        //     scenarios = scenarios
        //         .iter()
        //         .filter(|s| s.robots.get(&Robots::Geode) != Some(&0))
        //         .map(|s| s.clone())
        //         .collect();
        //     println!("scenarios {:?}", scenarios.len());
        // }

        println!("");

        println!("step {}", step);
        let mut scenarios_on_step: Vec<Scenario> = vec![];
        scenarios.iter_mut().for_each(|s| {
            let new_scenarios = s.step();
            scenarios_on_step.extend(new_scenarios);
        });

        scenarios.extend(scenarios_on_step);
        step += 1;
    }

    scenarios
        .iter()
        .map(|s| s.resources.get_geode())
        .max()
        .unwrap() as usize
}

#[cfg(test)]

mod tests {
    use super::*;

    // #[test]
    // fn test_scenario() {
    //     let input = parse_input("src/specs/day19");

    //     // assert_eq!(test_blueprint(input[0].clone()), 9);
    //     assert_eq!(test_blueprint(input[1].clone()), 12);
    // }

    #[test]
    fn test_stock() {
        let s1 = Stock::new(1, 2, 3, 4);
        let s2 = Stock::new(1, 2, 3, 1);
        assert_eq!(s1.add(s2), Stock::new(2, 4, 6, 5));

        let s1 = Stock::new(1, 2, 3, 4);
        let s2 = Stock::new(1, 4, 3, 1);
        assert_eq!(s1.sub(s2), None);

        let s1 = Stock::new(4, 2, 3, 4);
        let s2 = Stock::new(1, 2, 3, 1);
        assert_eq!(s1.sub(s2), Some(Stock::new(3, 0, 0, 3)));
    }

    #[test]
    fn test_stock_compare() {
        let s1 = Stock::new(1, 2, 3, 4);
        let s2 = Stock::new(1, 10, 0, 1);
        assert_eq!(s1.cmp(&s2), Ordering::Greater);

        let s1 = Stock::new(1, 10, 3, 1);
        let s2 = Stock::new(1, 4, 8, 1);
        assert_eq!(s1.cmp(&s2), Ordering::Less);

        let s1 = Stock::new(1, 2, 3, 4);
        let s2 = Stock::new(1, 2, 3, 4);
        assert_eq!(s1.cmp(&s2), Ordering::Equal);
    }
}

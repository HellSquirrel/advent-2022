use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn parse_input(path: &str) -> HashMap<String, Monkey> {
    let file = File::open(path).unwrap();
    let mut i = 0;
    let mut result = io::BufReader::new(file)
        .lines()
        .map(|s| {
            let regex = regex::Regex::new(r"(\w+): (\w+) (\+|-|/|\*) (\w+)").unwrap();
            let value_regex = regex::Regex::new(r"(\w+): (\d+)").unwrap();

            let line = s.unwrap();

            if let Some(x) = regex.captures(&line) {
                let name = x[1].to_string();
                let op = match x[3].to_string().as_str() {
                    "+" => Op::Add(x[2].to_string(), x[4].to_string()),
                    "-" => Op::Sub(x[2].to_string(), x[4].to_string()),
                    "*" => Op::Mul(x[2].to_string(), x[4].to_string()),
                    "/" => Op::Div(x[2].to_string(), x[4].to_string()),
                    _ => panic!("unknown op"),
                };

                return (name.clone(), Monkey { op, name });
            } else {
                let x = value_regex.captures(&line).unwrap();
                let name = x[1].to_string();
                let value = x[2].parse::<i32>().unwrap();

                return (name.clone(), Monkey { op: Op::Value(value), name });
            }
        })
        .collect::<HashMap<String, Monkey>>();
        // println!("{:?}", result);
    result
}

pub(crate) fn part_1(path: &str) -> i32 {
    let input = parse_input(path);

    let mut ops: Vec<String> = Vec::with_capacity(input.len());
    let mut result: HashMap<String, i32> = HashMap::new();
    ops.push("root".to_string());

    loop {
        let last = ops.last().unwrap();
        let monkey = input.get(last).unwrap();
        let current_result = monkey.try_eval(&mut result);

        // println!("ops: {:?}, result: {:?}", ops, result);
        match current_result {
            Some(v) => {
                if last == "root" {
                    return v;
                }
                ops.pop();
            }

            None => {
                match &monkey.op {
                    Op::Add(a, b) => {
                        ops.push(a.clone());
                        ops.push(b.clone());
                    }
                    Op::Sub(a, b) => {
                        ops.push(a.clone());
                        ops.push(b.clone());
                    },
                    Op::Mul(a, b) => {
                        ops.push(a.clone());
                        ops.push(b.clone());
                    },
                    Op::Div(a, b) => {
                        ops.push(a.clone());
                        ops.push(b.clone());
                    },
                    Op::Value(_) => {},
                };
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Op {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Value(i32),
}

#[derive(Debug, Clone)]
pub(crate) struct Monkey {
    op: Op,
    name: String,
}

impl Monkey {
    fn try_eval(&self, result: &mut HashMap<String, i32>) -> Option<i32> {
        match &self.op {
            Op::Add(a, b) => {
                let a = result.get(a);
                let b = result.get(b);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        let val = a + b;
                        result.insert(self.name.clone(),val);
                        Some(val)
                    },
                    _ => None,
                }
            }
            Op::Sub(a, b) => {
                let a = result.get(a);
                let b = result.get(b);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        let val = a - b;
                        result.insert(self.name.clone(),val);
                        Some(val)
                    },
                    _ =>  None,
                }
            }
            Op::Mul(a, b) => {
                let a = result.get(a);
                let b = result.get(b);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        let val = a * b;
                        result.insert(self.name.clone(),val);
                        Some(val)
                    },
                    _ =>  None,
                }
            }
            Op::Div(a, b) => {
                let a = result.get(a);
                let b = result.get(b);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        let val = a / b;
                        result.insert(self.name.clone(),val);
                        Some(val)
                    },
                    _ =>  None,
                }
            }
            Op::Value(x) => {
                result.insert(self.name.clone(),*x);
                Some(*x)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/specs/day21"), 152);
    }
}
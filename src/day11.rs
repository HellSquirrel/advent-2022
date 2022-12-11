use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_input(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file");
    let mut monkey_strings = vec![String::new()];

    for i in io::BufReader::new(file).lines() {
        let line = i.unwrap();

        if line.len() < 3 {
            monkey_strings.push(String::new());
        } else {
            let result = monkey_strings.pop().unwrap();

            monkey_strings.push(format!("{}{}", result, line));
        }
    }

    monkey_strings
}

pub fn monkeys_to_string(path: &str, rounds: usize) -> usize {
    let input = parse_input(path);
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let regexp =
        Regex::new(r"Monkey (\d+):.*Starting items:(.*)Operation:.*new = old (.) (\d+|old).*Test: divisible by (\d+).*If true: throw to monkey (\d+).*If false: throw to monkey (\d+)")
            .unwrap();

    for monkey_s in input {
        let captures = regexp.captures(&monkey_s).unwrap();
        let monkey_id = captures.get(1).unwrap().as_str();
        let items = captures
            .get(2)
            .unwrap()
            .as_str()
            .trim()
            .split(", ")
            .map(|i| i.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();

        let op = captures.get(3).unwrap().as_str();
        let op_value = captures.get(4).unwrap().as_str();
        let div_by = captures.get(5).unwrap().as_str().parse::<f32>().unwrap();
        let monkey_id_true = captures.get(6).unwrap().as_str();
        let monkey_id_false = captures.get(7).unwrap().as_str();

        let monkey = Monkey::new_from_figures(
            String::from(monkey_id),
            items,
            div_by,
            String::from(op),
            String::from(op_value),
            String::from(monkey_id_true),
            String::from(monkey_id_false),
        );

        monkeys.insert(String::from(monkey_id), monkey);
    }

    monkey_loop(monkeys, rounds)
}

struct Monkey {
    name: String,
    items: Vec<f32>,
    op: Box<dyn Fn(f32) -> f32>,
    test: Box<dyn Fn(f32) -> String>,
    counter: usize,
}

impl Monkey {
    fn new(
        name: String,
        items: Vec<f32>,
        op: Box<dyn Fn(f32) -> f32>,
        test: Box<dyn Fn(f32) -> String>,
    ) -> Monkey {
        let len = items.len();
        Monkey {
            name,
            items,
            op,
            test,
            counter: 0,
        }
    }

    fn new_from_figures(
        name: String,
        items: Vec<f32>,
        div_by: f32,
        op: String,
        op_value: String,
        monkey_id_true: String,
        monkey_id_false: String,
    ) -> Monkey {
        let len = items.len();
        let test = Box::new(move |i: f32| {
            if i.round() % div_by == 0.0 {
                monkey_id_true.clone()
            } else {
                monkey_id_false.clone()
            }
        });

        let op = Box::new(move |i| {
            let op_colone = op.clone();
            let value = if op_value == "old" {
                i
            } else {
                op_value.parse::<f32>().unwrap()
            };

            match op_colone.as_str() {
                "+" => (i + value).round(),
                "-" => (i - value).round(),
                "*" => (i * value).round(),
                "/" => (i / value).round(),
                _ => panic!("Unknown op"),
            }
        });
        Monkey {
            name,
            items,
            op,
            test,
            counter: 0,
        }
    }

    fn process_items(&self) -> Vec<(f32, String)> {
        let items = &self.items;
        let result: Vec<_> = items
            .iter()
            .map(|i| {
                let wl = ((self.op)(*i) / 3.0).floor();
                let next = (self.test)(wl);
                (wl, next)
            })
            .collect();

        // println!("{}: {:?}", self.name, result);
        result
    }

    fn throw_items(&mut self) -> Vec<(f32, String)> {
        let result = self.process_items();
        self.items = vec![];
        self.counter += result.len();
        result
    }

    fn get_item(&mut self, item: f32) {
        self.items.push(item);
    }
}

fn monkey_loop<'a>(monkeys: HashMap<String, Monkey>, rounds: usize) -> usize {
    let mut monkeys = monkeys;
    let mut keys = monkeys
        .keys()
        .map(|k| k.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    keys.sort();

    for i in 0..rounds {
        let mut result = vec![];
        for k in keys.iter() {
            result = monkeys
                .get_mut(&String::from(format!("{}", k)))
                .unwrap()
                .throw_items();
            for (item, next) in result {
                monkeys.get_mut(&next).unwrap().get_item(item);
            }
        }

        // for k in &keys {
        //     println!(
        //         "round ---> {} {}",
        //         i,
        //         monkeys.get_mut(&String::from(format!("{}", k))).unwrap()
        //     );
        // }
    }

    let mut two_most_active: Vec<_> = monkeys.iter().map(|(_, monkey)| monkey.counter).collect();
    // two_most_active.iter().sort();
    two_most_active.sort();
    two_most_active.reverse();
    two_most_active[0] * two_most_active[1]
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nMonkey: {}, Items: {:?} Counter: {}\n",
            self.name, self.items, self.counter
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey() {
        let mut monkey = Monkey::new(
            String::from("monkey 1"),
            vec![1.0, 2.0, 3.0],
            Box::new(|i| i * 3.0),
            Box::new(|i| {
                if (i as i32) % 2 == 0 {
                    String::from("monkey 3")
                } else {
                    String::from("monkey 4")
                }
            }),
        );

        let result = monkey.throw_items();
        assert_eq!(
            result,
            vec![
                (1.0, String::from("monkey 4")),
                (2.0, String::from("monkey 3")),
                (3.0, String::from("monkey 4"))
            ]
        );

        assert_eq!(monkey.items, vec![]);
        assert_eq!(monkey.counter, 3);
    }

    #[test]
    fn test_monkey_get_item() {
        let mut monkey = Monkey::new(
            String::from("monkey 1"),
            vec![1.0, 2.0, 3.0],
            Box::new(|i| i * 3.0),
            Box::new(|i| {
                if (i as i32) % 2 == 0 {
                    String::from("monkey 3")
                } else {
                    String::from("monkey 4")
                }
            }),
        );

        monkey.get_item(30.0);
        assert_eq!(monkey.items, vec![1.0, 2.0, 3.0, 30.0]);
        assert_eq!(monkey.counter, 0);
    }

    #[test]
    fn test_monkeys_to_stirng() {
        let result = monkeys_to_string("src/specs/day11", 20);
        assert_eq!(result, 10605);
    }
}

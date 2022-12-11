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
    let mut parsed_monkey_data: Vec<(String, Vec<i32>, Op, i32, String, String)> = vec![];
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
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let op = captures.get(3).unwrap().as_str();
        let op_value = captures.get(4).unwrap().as_str();
        let op = if op_value == "old" {
            Op::Square
        } else {
            match op {
                "+" => Op::Plus(op_value.parse::<i32>().unwrap()),
                "*" => Op::Mul(op_value.parse::<i32>().unwrap()),
                _ => panic!("Unknown operation"),
            }
        };

        let div_by = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();
        let true_monkey = captures.get(6).unwrap().as_str();
        let false_monkey = captures.get(7).unwrap().as_str();

        parsed_monkey_data.push((
            monkey_id.to_string(),
            items,
            op,
            div_by,
            true_monkey.to_string(),
            false_monkey.to_string(),
        ));
    }

    let divisors = parsed_monkey_data
        .iter()
        .map(|(_, _, _, div_by, _, _)| *div_by)
        .collect::<Vec<_>>();

    for (monkey_id, items, op, div_by, true_monkey, false_monkey) in parsed_monkey_data {
        let monkey = Monkey {
            name: monkey_id.clone(),
            items: items
                .iter()
                .map(|i| Reminders::new(*i, divisors.clone()))
                .collect(),
            op,
            divisor: div_by,
            counter: 0,
            true_monkey,
            false_monkey,
        };

        monkeys.insert(monkey_id, monkey);
    }

    monkey_loop(monkeys, rounds)
}

#[derive(Debug, Clone)]
struct Reminder {
    value: i32,
    divisor: i32,
}

impl Reminder {
    fn new(value: i32, divisor: i32) -> Reminder {
        Reminder { value, divisor }
    }

    fn plus(&self, other: i32) -> Reminder {
        Reminder {
            value: (self.value + other) % self.divisor,
            divisor: self.divisor,
        }
    }

    fn mul(&self, other: i32) -> Reminder {
        Reminder {
            value: (self.value * other) % self.divisor,
            divisor: self.divisor,
        }
    }

    fn square(&self) -> Reminder {
        Reminder {
            value: (self.value * self.value) % self.divisor,
            divisor: self.divisor,
        }
    }

    fn apply(&self, op: &Op) -> Reminder {
        match op {
            Op::Plus(i) => self.plus(*i),
            Op::Mul(i) => self.mul(*i),
            Op::Square => self.square(),
        }
    }

    fn is_divisible(&self) -> bool {
        self.value == 0
    }
}

#[derive(Debug, Clone)]
struct Reminders {
    items: HashMap<i32, Reminder>,
}

impl Reminders {
    fn new(raw_val: i32, divisors: Vec<i32>) -> Reminders {
        let items: HashMap<_, _> = divisors
            .iter()
            .map(|d| (*d, Reminder::new(raw_val % d, *d)))
            .collect();
        Reminders { items }
    }

    fn apply(&self, op: &Op) -> Reminders {
        let items: HashMap<_, _> = self
            .items
            .iter()
            .map(|(v, rem)| (*v, rem.apply(op)))
            .collect();

        Reminders { items }
    }

    fn is_divisible_by(&self, divisor: i32) -> bool {
        self.items.get(&divisor).unwrap().is_divisible()
    }
}

#[derive(Debug, Clone)]
enum Op {
    Plus(i32),
    Mul(i32),
    Square,
}

struct Monkey {
    name: String,
    items: Vec<Reminders>,
    op: Op,
    divisor: i32,
    counter: usize,
    true_monkey: String,
    false_monkey: String,
}

impl Monkey {
    fn new(
        name: String,
        items: Vec<i32>,
        op: Op,
        divisor: i32,
        divisors: Vec<i32>,
        counter: usize,
        true_monkey: String,
        false_monkey: String,
    ) -> Monkey {
        let len = items.len();
        Monkey {
            name,
            items: items
                .iter()
                .map(|i| Reminders::new(*i, divisors.clone()))
                .collect(),
            op,
            divisor,
            counter: 0,
            true_monkey,
            false_monkey,
        }
    }

    fn process_items(&self) -> Vec<(String, Reminders)> {
        let items = &self.items;
        let result: Vec<_> = items
            .iter()
            .map(|i| {
                let wl = i.apply(&self.op);
                if wl.is_divisible_by(self.divisor) {
                    (self.true_monkey.clone(), wl)
                } else {
                    (self.false_monkey.clone(), wl)
                }
            })
            .collect();
        result
    }

    fn throw_items(&mut self) -> Vec<(String, Reminders)> {
        let result = self.process_items();
        self.items = vec![];
        self.counter += result.len();
        result
    }

    fn get_item(&mut self, item: Reminders) {
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
            for (next, item) in result {
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
    fn test_plus() {
        let reminder = Reminder {
            value: 3,
            divisor: 3,
        };
        assert_eq!(reminder.plus(2).value, 2);
        assert_eq!(reminder.plus(2).divisor, 3);
    }

    #[test]
    fn test_mul() {
        let reminder = Reminder {
            value: 10,
            divisor: 3,
        };
        assert_eq!(reminder.mul(2).value, 2);
        assert_eq!(reminder.mul(2).divisor, 3);
    }

    #[test]
    fn test_monkeys_to_stirng() {
        let result = monkeys_to_string("src/specs/day11", 10000);
        assert_eq!(result, 2713310158);
    }
}

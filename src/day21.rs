use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Value(f32),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Compare(Box<Expression>, Box<Expression>),
}

use Expression::*;

pub(crate) fn parse_input(path: &str) -> HashMap<String, Expression> {
    let file = File::open(path).unwrap();
    let mut i = 0;
    let mut result = io::BufReader::new(file)
        .lines()
        .map(|s| {
            let regex = regex::Regex::new(r"(\w+): (\w+) (\+|-|/|\*) (\w+)").unwrap();
            let value_regex = regex::Regex::new(r"(\w+): (\d+)").unwrap();

            let line = s.unwrap();

            if let Some(x) = regex.captures(&line) {
                let v1 = Box::new(Variable(x[2].to_string()));
                let v2 = Box::new(Variable(x[4].to_string()));
                let name = x[1].to_string();
                if name == "root" {
                    return (name.clone(), Compare(v1, v2));
                }
                let op = match x[3].to_string().as_str() {
                    "+" => Add(v1, v2),
                    "-" => Sub(v1, v2),
                    "*" => Mul(v1, v2),
                    "/" => Div(v1, v2),
                    _ => panic!("unknown op"),
                };

                return (name.clone(), op);
            } else {
                let x = value_regex.captures(&line).unwrap();
                let name = x[1].to_string();
                let value = x[2].parse::<f32>().unwrap();

                return (name.clone(), Value(value));
            }
        })
        .collect::<HashMap<String, Expression>>();
    result.remove("humn");
    let mut rootOp = result.remove("root").unwrap();

    let mut prev = result.clone();

    loop {
        reduce(&mut result);

        if prev != result {
            prev = result.clone();
        }

        if prev == result {
            break;
        }
    }

    rootOp = substitute_variable(rootOp, &result, &vec!["humn".to_string()]);

    let (left, right) = match rootOp {
        Compare(a, b) => (a, b),
        _ => panic!("root op is not compare"),
    };

    println!("{:#?}, {:#?}", left, right);

    let mut hm = HashMap::new();
    hm.insert("humn".to_string(), Value(10.0));

    let unboxed = *left;

    let unboxed = substitute_variable(unboxed, &hm, &vec![]);
    unboxed.reduce();

    // println!("{:#?}", result);

    println!("unboxed: {:#?}", unboxed);
    // println!("{:#?}", result);
    result
}

fn replace_variables(hash: &mut HashMap<String, Expression>) {
    let clone = hash.clone();
    for (_, v) in hash.iter_mut() {
        *v = substitute_variable(v.clone(), &clone, &vec!["humn".to_string()]);
    }
}

fn reduce(hash: &mut HashMap<String, Expression>) {
    replace_variables(hash);
    for (_, v) in hash.iter_mut() {
        *v = v.reduce();
        // println!("reduced {:#?}", v.clone())
    }
}

fn substitute_variable(e: Expression, hash: &HashMap<String, Expression>, ignore_keys: &Vec<String>) -> Expression {
    match e {
        Variable(v) => {
            if ignore_keys.contains(&v) {
                return Variable(v);
            }
            substitute_variable(hash.get(&v).unwrap().clone(), hash, ignore_keys)
        }

        Add(a, b) => {
            let a = substitute_variable(*a, hash, ignore_keys);
            let b = substitute_variable(*b, hash, ignore_keys);
            Add(Box::new(a), Box::new(b))
        }

        Compare(a, b) => {
            let a = substitute_variable(*a, hash, ignore_keys);
            let b = substitute_variable(*b, hash, ignore_keys);
            Compare(Box::new(a), Box::new(b))
        }

        Sub(a, b) => {
            let a = substitute_variable(*a, hash, ignore_keys);
            let b = substitute_variable(*b, hash, ignore_keys);
            Sub(Box::new(a), Box::new(b))
        }

        Mul(a, b) => {
            let a = substitute_variable(*a, hash, ignore_keys);
            let b = substitute_variable(*b, hash, ignore_keys);
            Mul(Box::new(a), Box::new(b))
        }

        Div(a, b) => {
            let a = substitute_variable(*a, hash, ignore_keys);
            let b = substitute_variable(*b, hash, ignore_keys);
            Div(Box::new(a), Box::new(b))
        }

        _ => e.clone(),
    }
}

impl Expression {
    fn reduce(&self) -> Expression {
        match self {
            Sub(a, b) => match (a.reduce(), b.reduce()) {
                (Value(a), Value(b)) => Value(a - b),
                (a, b) => Sub(Box::new(a), Box::new(b)),
            },

            Mul(a, b) => match (a.reduce(), b.reduce()) {
                (Value(a), Value(b)) => Value(a * b),
                (a, b) => Mul(Box::new(a), Box::new(b)),
            },

            Div(a, b) => match (a.reduce(), b.reduce()) {
                (Value(a), Value(b)) => Value(a / b),
                (a, b) => Div(Box::new(a), Box::new(b)),
            },

            Add(a, b) => {
                let a = a.reduce();
                let b = b.reduce();
                match (a, b) {
                    (Value(a), Value(b)) => Value(a + b),
                    (Add(c, d), Value(b)) => {
                        let c = c.reduce();
                        let d = d.reduce();
                        match (&c, &d) {
                            (Value(c), _) => Add(Box::new(Value(c + b)), Box::new(d)),
                            (_, Value(d)) => Add(Box::new(c), Box::new(Value(d + b))),
                            _ => self.clone(),
                        }
                    }

                    (Sub(c, d), Value(b)) => {
                        let c = c.reduce();
                        let d = d.reduce();
                        match (&c, &d) {
                            (Value(c), _) => Sub(Box::new(Value(c + b)), Box::new(d)),
                            (_, Value(d)) => Sub(Box::new(c), Box::new(Value(d - b))),
                            _ => self.clone(),
                        }
                    }

                    (Value(a), Sub(c, d)) => {
                        let c = c.reduce();
                        let d = d.reduce();
                        match (&c, &d) {
                            (Value(c), _) => Sub(Box::new(Value(c + a)), Box::new(d)),
                            (_, Value(d)) => Sub(Box::new(c), Box::new(Value(d - a))),
                            _ => self.clone(),
                        }
                    }

                    (Value(a), Add(c, d)) => {
                        let c = c.reduce();
                        let d = d.reduce();
                        match (&c, &d) {
                            (Value(c), _) => Add(Box::new(Value(c + a)), Box::new(d)),
                            (_, Value(d)) => Add(Box::new(c), Box::new(Value(d + a))),
                            _ => self.clone(),
                        }
                    }

                    (a, b) => Add(Box::new(a), Box::new(b)),
                }
            }

            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expression::*;
    use super::*;

    #[test]
    fn test_op() {
        let mut a = Expression::Add(
            Box::new(Expression::Value(1.0)),
            Box::new(Expression::Value(2.0)),
        );
        a = a.reduce();
        assert_eq!(a, Expression::Value(3.0));

        assert_eq!(
            Expression::Add(
                Box::new(Expression::Value(1.0)),
                Box::new(Expression::Add(
                    Box::new(Expression::Value(2.0)),
                    Box::new(Expression::Value(3.0))
                ))
            )
            .reduce(),
            Expression::Value(6.0)
        );

        assert_eq!(
            Expression::Add(
                Box::new(Expression::Add(
                    Box::new(Expression::Variable("b".to_string())),
                    Box::new(Expression::Value(2.0))
                )),
                Box::new(Expression::Value(3.0))
            )
            .reduce(),
            Add(Box::new(Variable("b".to_string())), Box::new(Value(5.0)))
        );

        assert_eq!(
            Expression::Add(
                Box::new(Expression::Sub(
                    Box::new(Expression::Variable("b".to_string())),
                    Box::new(Expression::Value(2.0))
                )),
                Box::new(Expression::Value(3.0))
            )
            .reduce(),
            Sub(Box::new(Variable("b".to_string())), Box::new(Value(-1.0)))
        )
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input("src/specs/day21");
        assert_eq!(2, 4);
    }
}

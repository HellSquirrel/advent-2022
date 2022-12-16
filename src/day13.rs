use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq, Clone)]
enum RecList {
    List(Vec<Box<RecList>>),
    Value(i32),
}

impl RecList {
    fn new() -> Self {
        Self::List(Vec::new())
    }

    fn push(&mut self, item: Self) {
        match self {
            Self::List(l) => l.push(Box::new(item)),
            Self::Value(_) => panic!("Cannot push to a value"),
        }
    }

    fn get(&self, index: usize) -> &Self {
        match self {
            Self::List(l) => &l[index],
            Self::Value(_) => self,
        }
    }

    fn get_mut(&mut self, index: usize) -> &mut Self {
        match self {
            Self::List(l) => &mut l[index],
            Self::Value(_) => self,
        }
    }

    fn from(i: i32) -> Self {
        Self::Value(i)
    }

    fn from_vec(vec: Vec<i32>) -> Self {
        Self::List(vec.iter().map(|i| Box::new(Self::from(*i))).collect())
    }

    fn len(&self) -> usize {
        match self {
            Self::List(l) => l.len(),
            Self::Value(_) => 1,
        }
    }

    fn to_list(val: RecList) -> Self {
        match val {
            Self::List(_) => val,
            Self::Value(val) => Self::List(vec![Box::new(Self::Value(val))]),
        }
    }

    fn from_string(source: &str) -> Self {
        let mut stack: Vec<RecList> = Vec::with_capacity(source.len());
        let mut sub_str: Vec<String> = Vec::new();
        for i in source.chars() {
            match i {
                '[' => {
                    for el in &sub_str {
                        if el == "" {
                            continue;
                        }
                        let val = el.parse::<i32>().unwrap();
                        stack.last_mut().unwrap().push(Self::from(val));
                    }

                    stack.push(Self::new());

                    sub_str = Vec::new();
                }
                ',' => sub_str.push(String::from("")),
                ']' => {
                    let mut last = stack.pop().unwrap();

                    for el in sub_str.iter_mut() {
                        let val = el.parse::<i32>().unwrap();
                        last.push(Self::from(val));
                    }

                    sub_str = Vec::new();

                    if stack.is_empty() {
                        return last;
                    } else {
                        stack.last_mut().unwrap().push(last);
                    }
                }
                _ => {
                    if sub_str.is_empty() {
                        sub_str.push(String::from(""));
                    }
                    sub_str.last_mut().unwrap().push(i);
                }
            }
        }

        Self::Value(0)
    }
}

impl std::fmt::Display for RecList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(l) => {
                let mut s = String::from("[");
                for i in l {
                    s.push_str(&format!("{},", i));
                }

                while s.ends_with(",") {
                    s.pop();
                }
                s.push(']');
                write!(f, "{}", s)
            }
            Self::Value(v) => write!(f, "{}", v),
        }
    }
}

impl PartialOrd for RecList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut deep = String::from(" ");
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a.partial_cmp(b),
            (a, b) => {
                deep.push(' ');
                let list_a = Self::to_list(a.clone());
                let list_b = Self::to_list(b.clone());
                let l1 = list_a.len();
                let l2 = list_b.len();

                let mut i = 0;
                let mut results: Vec<Option<Ordering>> = vec![];
                for i in 0..*([l1, l2]).iter().min().unwrap() as usize {
                    let a = list_a.get(i);
                    let b = list_b.get(i);
                    let la = list_a.len();
                    let lb = list_b.len();

                    match a.partial_cmp(b) {
                        Some(Ordering::Less) => {
                            results.push(Some(Ordering::Less));
                            break;
                        }
                        Some(Ordering::Equal) => {
                            results.push(Some(Ordering::Equal));
                            continue;
                        }
                        Some(Ordering::Greater) => {
                            results.push(Some(Ordering::Greater));
                            break;
                        }
                        None => {
                            results.push(Some(Ordering::Equal));
                            continue;
                        }
                    }
                }

                let is_gt = results.iter().any(|r| match r {
                    Some(Ordering::Greater) => true,
                    _ => false,
                });

                let is_lt = results.iter().any(|r| match r {
                    Some(Ordering::Less) => true,
                    _ => false,
                });

                if is_gt {
                    return Some(Ordering::Greater);
                }

                if is_lt {
                    return Some(Ordering::Less);
                }

                if l1 > l2 {
                    Some(Ordering::Greater)
                } else if l1 < l2 {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

impl Ord for RecList {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Equal) => Ordering::Equal,
            Some(Ordering::Greater) => Ordering::Greater,
            None => Ordering::Equal,
        }
    }
}

pub fn parse_input(path: &str) -> (usize, usize) {
    let file = File::open(path).expect("Unable to open file");

    let mut contents = io::BufReader::new(file)
        .lines()
        .filter_map(|s| match s {
            Ok(s) => {
                if s.len() < 2 {
                    None
                } else {
                    Some(s)
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<String>>();

    let right_order: &usize = &mut contents
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| {
            let a = RecList::from_string(&chunk[0]);
            let b = RecList::from_string(&chunk[1]);

            if let Some(Ordering::Less) = a.partial_cmp(&b) {
                return i + 1;
            }

            0
        })
        .sum();

    let sorted = &mut contents
        .iter()
        .map(|s| RecList::from_string(s))
        .collect::<Vec<RecList>>();

    let marker1 = RecList::from_string("[[2]]");
    let marker2 = RecList::from_string("[[6]]");

    sorted.push(marker1.clone());
    sorted.push(marker2.clone());

    sorted.sort();

    let i1 = sorted.iter().position(|s| s == &marker1).unwrap();
    let i2 = sorted.iter().position(|s| s == &marker2).unwrap();

    (*right_order, (i1 + 1) * (i2 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rec_list() {
        let list = RecList::new();
        assert_eq!(list.len(), 0);

        let list2 = RecList::from(1);
        assert_eq!(list2.len(), 1);

        let list3 = RecList::from_vec(vec![1, 2, 3]);
        assert_eq!(list3.len(), 3);
    }

    #[test]
    fn test_rec_fns() {
        let list = RecList::from(5);
        assert_eq!(list.get(0), &RecList::from(5));

        let list2 = RecList::from_vec(vec![1, 2, 3]);
        assert_eq!(list2.get(2), &RecList::from(3));

        let mut list3 = RecList::from_vec(vec![1, 2, 3]);
        list3.push(RecList::from(4));

        assert_eq!(list3.len(), 4);
        assert_eq!(list3.get(3), &RecList::from(4));

        let list4 = RecList::from_vec(vec![1, 2, 3]);
        let mut list5 = RecList::new();
        list5.push(RecList::from(4));
        list5.push(list4);

        assert_eq!(list5.len(), 2);

        let sub_list = list5.get_mut(1);
        assert_eq!(sub_list.len(), 3);
        sub_list.push(RecList::from(5));
        assert_eq!(sub_list.len(), 4);
    }

    #[test]
    fn test_to_list() {
        let list = RecList::from(5);
        let len = list.len();
        assert_eq!(len, 1);
        let result = RecList::to_list(list);

        assert_eq!(result, RecList::from_vec(vec![5]));
    }

    #[test]
    fn test_ord_values() {
        let l1 = RecList::from(1);
        let l2 = RecList::from(2);
        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
    }

    #[test]
    fn test_ord_values_2() {
        let l1 = RecList::from(1);
        let mut l2 = RecList::new();
        l2.push(RecList::from(2));

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
    }

    #[test]
    fn test_ord_values_3() {
        let l1 = RecList::from(4);
        let mut l2 = RecList::new();
        l2.push(RecList::from(3));

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Greater));
    }

    #[test]
    fn test_ord_values_4() {
        let l1 = RecList::from_vec(vec![1, 2, 3]);
        let l2 = RecList::from_vec(vec![4, 5, 6]);

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
    }

    #[test]
    fn test_ord_values_5() {
        let l1 = RecList::from_vec(vec![1, 2, 3]);
        let l2 = RecList::from_vec(vec![1, 5, 6]);

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
    }

    #[test]
    fn test_ord_values_6() {
        let l1 = RecList::from_vec(vec![1, 2]);
        let l2 = RecList::from_vec(vec![4, 5, 0]);

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
    }

    #[test]

    fn test_ord_values_7() {
        let l1 = RecList::from_vec(vec![1, 2, 0]);
        let l2 = RecList::from_vec(vec![4, 5]);

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
    }

    #[test]
    fn test_from_string() {
        let result = RecList::from_string("[1,1,3,1,1]");
        assert_eq!(result, RecList::from_vec(vec![1, 1, 3, 1, 1]));
    }

    #[test]
    fn test_from_string_complex() {
        let result = RecList::from_string("[1,[2,[3,[4,[5,6,7]]]],8,9]");

        assert_eq!(result.get(0), &RecList::from(1));
        assert_eq!(
            result.get(1).get(1).get(1).get(1),
            &RecList::from_vec(vec![5, 6, 7])
        );
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input("src/specs/day13");
        assert_eq!(input.0, 13);
    }

    #[test]
    fn test_parse_input_2() {
        let input = parse_input("src/specs/day13");
        assert_eq!(input.1, 140);
    }
}

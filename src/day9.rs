use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Rope {
    head: Point,
    tail: Point,
    tail_path: Vec<Point>,
}

impl Rope {
    fn new(start: Point) -> Rope {
        Rope {
            head: start,
            tail: start,
            tail_path: vec![start],
        }
    }

    fn adjust_tail(&mut self) {
        let dx = self.head.x - self.tail.x;
        let dy = self.head.y - self.tail.y;

        println!("\n");
        println!("dx: {}", dx);
        println!("dy: {}", dy);
        println!("head {} { }", self.head.x, self.head.y);

        if dx.abs() > 1 || dy.abs() > 1 || dx.abs() + dy.abs() > 2 {
            println!("adjusting tail");
            let x_step = dx.signum();
            let y_step = dy.signum();

            self.tail.x += x_step;
            self.tail.y += y_step;

            println!("tail {} {}", self.tail.x, self.tail.y);
        }
    }

    fn log_tail(&mut self) {
        let last = self.tail_path.last().unwrap();
        if (last.x != self.tail.x) || (last.y != self.tail.y) {
            self.tail_path.push(self.tail);
        };
    }

    fn move_head(&mut self, (dx, dy): (i32, i32)) {
        let x_step = dx.signum();
        let y_step = dy.signum();
        let count_x = dx.abs();
        let count_y = dy.abs();
        let mut steps = 0;

        loop {
            if (steps == count_x) || count_x == 0 {
                break;
            }

            self.head.x += x_step;
            self.adjust_tail();
            self.log_tail();

            steps += 1;

            // println!("head: {:?}", self.head);
            // println!("tail: {:?}", self.tail);
        }

        steps = 0;

        loop {
            if steps == (count_y) || count_y == 0 {
                break;
            }

            self.head.y += y_step;
            self.adjust_tail();
            self.log_tail();

            steps += 1;

            // println!("head: {:?}", self.head);
            // println!("tail: {:?}", self.tail);
        }
    }
}

pub fn parse_input(path: &str) -> usize {
    let file = File::open(path).expect("Unable to open file");
    let mut rope = Rope::new(Point::new(0, 0));

    for l in io::BufReader::new(file).lines() {
        let line = l.unwrap();
        let line = line.split("").collect::<Vec<&str>>();
        let direction = line[1];
        let distance = line[3..].join("").trim().parse::<i32>().unwrap();

        // println!("direction: {}", direction);
        // println!("distance: {}", distance);

        let (dx, dy) = match direction {
            "U" => (0, distance),
            "D" => (0, -distance),
            "L" => (-distance, 0),
            "R" => (distance, 0),
            _ => panic!("Unknown direction"),
        };

        rope.move_head((dx, dy));
    }

    // println!("tail: {:?}", rope.tail_path);
    let positions = rope.tail_path.iter().collect::<HashSet<_>>();
    for i in (0..=100).rev() {
        for j in 0..=100 {
            if positions.contains(&Point::new(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("");
    }
    positions.len()
}

#[cfg(test)]
#[test]
fn test_move_up() {
    let mut rope = Rope::new(Point::new(0, 0));
    rope.move_head((0, 0));
    assert_eq!(rope.tail, Point::new(0, 0));
    assert_eq!(rope.tail_path, vec![Point::new(0, 0)]);

    rope.move_head((0, 1));
    assert_eq!(rope.tail, Point::new(0, 0));
    assert_eq!(rope.tail_path, vec![Point::new(0, 0)]);

    rope.move_head((0, 4));
    assert_eq!(rope.tail, Point::new(0, 4));
    assert_eq!(
        rope.tail_path,
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
            Point::new(0, 4),
        ]
    );
}

#[test]
fn test_move_left() {
    let mut rope = Rope::new(Point::new(1, 0));
    rope.move_head((4, 0));
    assert_eq!(rope.tail, Point::new(4, 0));
    assert_eq!(
        rope.tail_path,
        vec![
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0)
        ]
    );
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("src/specs/day9"), 13);
}

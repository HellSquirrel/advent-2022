use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Transform {
    tx: i64,
    ty: i64,
}

impl Transform {
    fn new(tx: i64, ty: i64) -> Self {
        Transform { tx, ty }
    }

    fn apply(&self, point: Point) -> Point {
        Point {
            x: point.x + self.tx,
            y: point.y + self.ty,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Bbox {
    coords: Point,
    width: i64,
    height: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Figure {
    points: HashSet<Point>,
}

impl Figure {
    fn new() -> Self {
        Figure {
            points: HashSet::new(),
        }
    }

    fn square() -> Self {
        let mut points = HashSet::new();
        points.insert(Point { x: 0, y: 0 });
        points.insert(Point { x: 1, y: 0 });
        points.insert(Point { x: 0, y: 1 });
        points.insert(Point { x: 1, y: 1 });
        Figure { points }
    }

    fn minus() -> Self {
        let mut points = HashSet::new();
        points.insert(Point { x: 0, y: 0 });
        points.insert(Point { x: 1, y: 0 });
        points.insert(Point { x: 2, y: 0 });
        points.insert(Point { x: 3, y: 0 });
        Figure { points }
    }

    fn pipe() -> Self {
        let mut points = HashSet::new();
        points.insert(Point { x: 0, y: 0 });
        points.insert(Point { x: 0, y: 1 });
        points.insert(Point { x: 0, y: 2 });
        points.insert(Point { x: 0, y: 3 });
        Figure { points }
    }

    fn plus() -> Self {
        let mut points = HashSet::new();
        points.insert(Point { x: 1, y: 0 });
        points.insert(Point { x: 1, y: 1 });
        points.insert(Point { x: 1, y: 2 });
        points.insert(Point { x: 0, y: 1 });
        points.insert(Point { x: 2, y: 1 });
        Figure { points }
    }

    fn l() -> Self {
        let mut points = HashSet::new();
        points.insert(Point { x: 0, y: 0 });
        points.insert(Point { x: 1, y: 0 });
        points.insert(Point { x: 2, y: 0 });
        points.insert(Point { x: 2, y: 1 });
        points.insert(Point { x: 2, y: 2 });

        Figure { points }
    }

    fn merge(&mut self, other: Figure) {
        self.points.extend(other.points);
    }

    fn intersect(&self, p: &HashSet<Point>) -> bool {
        self.points.intersection(p).count() > 0
    }

    fn intersect_x(&self, x: i64) -> bool {
        self.points.iter().any(|p| p.x == x)
    }

    fn bbox(&self) -> Bbox {
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;

        for i in self.points.iter() {
            if i.x < min_x {
                min_x = i.x;
            }
            if i.x > max_x {
                max_x = i.x;
            }
            if i.y < min_y {
                min_y = i.y;
            }
            if i.y > max_y {
                max_y = i.y;
            }
        }

        if self.points.is_empty() {
            return Bbox {
                coords: Point { x: 0, y: 0 },
                width: 0,
                height: 0,
            };
        }

        Bbox {
            coords: Point { x: min_x, y: min_y },
            width: max_x - min_x + 1,
            height: max_y - min_y + 1,
        }
    }

    fn apply_transform(&mut self, transform: Transform) {
        let transformed = self
            .points
            .iter()
            .map(|p| transform.apply(*p))
            .collect::<HashSet<Point>>();
        self.points = transformed;
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bbox = self.bbox();

        for y in (0..bbox.height).rev() {
            for x in 0..bbox.width {
                let point = Point {
                    x: bbox.coords.x + x,
                    y: bbox.coords.y + y,
                };
                if self.points.contains(&point) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    time: i64,
    stable_figure: Figure,
    moving_figure: Figure,
    width: i64,
    height: i64,
    wind_pattern: Vec<i64>,
    cycle: i64,
    figures: [Figure; 5],
    cycle_positions: Vec<(i64, i64, i64)>,
    loop_after: i32,
    loop_period: i32,
    loop_height: i64,
}

impl Field {
    fn new(width: i64, wind_pattern: Vec<i64>) -> Self {
        Field {
            time: -1,
            stable_figure: Figure::new(),
            moving_figure: Figure::new(),
            width,
            height: 0,
            wind_pattern,
            cycle: -1,
            figures: [
                Figure::minus(),
                Figure::plus(),
                Figure::l(),
                Figure::pipe(),
                Figure::square(),
            ],
            cycle_positions: Vec::new(),
            loop_after: -1,
            loop_height: 0,
            loop_period: 0,
        }
    }

    fn floor(&self) -> HashSet<Point> {
        (0..self.width).map(|x| Point { x, y: -1 }).collect()
    }

    fn get_figure_position_x(&self) -> Vec<Point> {
        let positions: Vec<i64> = Vec::with_capacity(self.wind_pattern.len());

        self.stable_figure
            .points
            .union(&self.moving_figure.points)
            .cloned()
            .collect()
    }

    fn add_figure(&mut self, figure: Figure) {
        let transform = Transform {
            tx: 2,
            ty: self.height + 3,
        };

        let mut mutable_figure = figure;
        mutable_figure.apply_transform(transform);

        self.moving_figure = mutable_figure;

        self.height = self.moving_figure.bbox().coords.y;
    }

    fn start_new_cycle(&mut self) {
        // before merge

        self.cycle += 1;
        self.add_second();

        // after merge

        self.stable_figure.merge(self.moving_figure.clone());
        self.height = self.stable_figure.bbox().height;

        if self.loop_after == -1 {
            let next = (
                self.time,
                self.moving_figure.bbox().coords.x,
                self.cycle % 5,
            );

            let index = self.cycle_positions.iter().position(|p| p == &next);

            if index.is_some() {
                self.loop_after = index.unwrap() as i32;
                self.loop_period = (self.cycle_positions.len() - self.loop_after as usize) as i32;
                self.loop_height = self.height;
            }

            self.cycle_positions.push(next);
        }

        self.add_figure(self.figures[self.cycle as usize % 5].clone());
    }

    fn get_current_wind(&self) -> i64 {
        self.wind_pattern[(self.time as usize % self.wind_pattern.len())]
    }

    fn apply_wind(&mut self, figure: Figure) -> Figure {
        let mut figure = figure.clone();
        figure.apply_transform(Transform {
            tx: self.get_current_wind(),
            ty: 0,
        });

        if figure.intersect_x(-1) {
            figure.apply_transform(Transform { tx: 1, ty: 0 });
        }

        if figure.intersect_x(self.width) {
            figure.apply_transform(Transform { tx: -1, ty: 0 });
        }

        if figure.intersect(&self.stable_figure.points) {
            figure.apply_transform(Transform {
                tx: -1 * self.get_current_wind(),
                ty: 0,
            });
        }

        figure
    }

    fn tick(&mut self) {
        self.moving_figure = self.apply_wind(self.moving_figure.clone());

        let mut next = self.moving_figure.clone();

        next.apply_transform(Transform { tx: 0, ty: -1 });

        let should_stop =
            next.intersect(&self.floor()) || next.intersect(&self.stable_figure.points);

        if should_stop {
            self.start_new_cycle();
        } else {
            self.moving_figure = next;
            self.add_second();
        }
    }

    fn add_second(&mut self) {
        self.time = (self.time + 1) % self.wind_pattern.len() as i64;
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "cycle: {}", self.cycle);
        for y in (-1..self.height).rev() {
            write!(f, "{:2} ", y);
            for x in -1..=self.width {
                let point = Point { x, y };
                if x == -1 {
                    write!(f, "|")?;
                } else if x == self.width {
                    write!(f, "|")?;
                } else if y == -1 {
                    write!(f, "_")?;
                } else {
                    if self.moving_figure.points.contains(&point) {
                        write!(f, "@")?;
                    } else if self.stable_figure.points.contains(&point) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_1(path: &str) -> usize {
    let content = read_to_string(path).unwrap();
    let pattern: Vec<i64> = content
        .trim()
        .split("")
        .filter(|c| !c.is_empty())
        .map(|c| match c {
            ">" => 1,
            "<" => -1,
            _ => 0,
        })
        .collect();

    // let mut field = Field::new(7, pattern.clone());
    // field.start_new_cycle();

    // while field.loop_after == i32::MAX {
    //     field.tick();
    // }

    // println!("Loop after {} cycles", field.loop_after);
    // println!("Loop height {}", field.loop_height);

    let mut field_2 = Field::new(7, pattern);

    // let iterations = 2022;
    // let remaining_iterations = iterations % (field.loop_after + 1) as usize;
    // let multiplyer = iterations / (field.loop_after + 1) as usize;

    // assert_eq!(
    //     remaining_iterations + multiplyer * (field.loop_after + 1) as usize,
    //     iterations
    // );

    // let additional_height = steps * field.loop_height as usize;

    let iter_count = 2022;
    field_2.start_new_cycle();
    while field_2.loop_after == -1 {
        field_2.tick();
    }

    for _ in 0..=field_2.loop_period {
        field_2.tick();
    }

    let height_2 = field_2.stable_figure.bbox().height as usize;
    let cycle_2 = field_2.cycle;

    // for _ in 0..=field_2.loop_period {
    //     field_2.tick();
    // }

    for _ in 0..20 {
        let prev = field_2.stable_figure.bbox().height as usize;
        let prev_cycle = field_2.cycle;

        for _ in 0..3 * (field_2.loop_period) {
            field_2.tick();
        }

        let next = field_2.stable_figure.bbox().height as usize;
        let next_cycle = field_2.cycle;

        println!("{} {}", next - prev, next_cycle - prev_cycle)
    }

    // let height_3 = field_2.stable_figure.bbox().height as usize;
    // let cycle_3 = field_2.cycle;

    // let delta_h = height_3 - height_2;
    // let delta_c = cycle_3 - cycle_2;

    // let remaining = cycle_3 + (iter_count - cycle_3) % delta_c;

    // while field_2.cycle <= remaining + 1 {
    //     field_2.tick();
    // }

    // field_2.stable_figure.bbox().height as usize
    //     + delta_h * ((iter_count - cycle_3) / delta_c) as usize
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figure() {
        let box_figure = Figure::square();
        let minus = Figure::minus();

        let box_str = format!("{}", box_figure);
        let minus_str = format!("{}", minus);

        assert_eq!(box_str, "##\n##\n");
        assert_eq!(minus_str, "####\n");
    }

    // #[test]
    // fn test_part_1() {
    //     let result = part_1("src/specs/day17");

    //     assert_eq!(result, 3068);
    // }
}

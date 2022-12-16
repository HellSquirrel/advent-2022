use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Sand,
    Rock,
    Empty,
    Source,
    Abyss,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Cell::Sand => write!(f, "o"),
            Cell::Rock => write!(f, "#"),
            Cell::Empty => write!(f, "."),
            Cell::Source => write!(f, "+"),
            Cell::Abyss => write!(f, "~"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct SandGrid {
    grid: HashMap<Point, Cell>,
    start: i32,
    end: i32,
    height: i32,
    source: Point,
}

impl SandGrid {
    fn new() -> SandGrid {
        let source = Point { x: 500, y: 0 };
        let mut grid = SandGrid {
            grid: HashMap::new(),
            start: source.x,
            end: source.x,
            height: source.y,
            source,
        };

        grid.set(grid.source, Cell::Source);
        grid.update_dims(grid.source);
        grid
    }

    fn update_dims(&mut self, point: Point) {
        let Point { x, y } = point;
        if y > self.height {
            self.height = y;
        }

        if x < self.start {
            self.start = x;
        }

        if x > self.end {
            self.end = x;
        }
    }

    fn add_rock_path(&mut self, from: Point, to: Point) {
        let Point { x: xf, y: yf } = from;
        let Point { x: xt, y: yt } = to;

        self.update_dims(from);
        self.update_dims(to);

        let delta_x = xt - xf;
        let delta_y = yt - yf;

        for j in 0..=delta_x.abs() {
            self.set(
                Point {
                    x: xf + j * delta_x.signum(),
                    y: yf,
                },
                Cell::Rock,
            );
        }

        for i in 0..=delta_y.abs() {
            self.set(
                Point {
                    y: yf + i * delta_y.signum(),
                    x: xf,
                },
                Cell::Rock,
            );
        }
    }

    fn drop_pebble(&mut self) -> (Point, Cell) {
        let mut current_point = self.source;

        loop {
            let down_point = Point {
                x: current_point.x,
                y: current_point.y + 1,
            };

            if self.get(down_point) == Cell::Empty {
                current_point = down_point;
                continue;
            }

            if self.get(down_point) == Cell::Abyss {
                return (current_point, Cell::Abyss);
            }

            let left_point = Point {
                x: current_point.x - 1,
                y: current_point.y + 1,
            };

            if self.get(left_point) == Cell::Empty {
                current_point = left_point;
                continue;
            }

            if self.get(left_point) == Cell::Abyss {
                return (current_point, Cell::Abyss);
            }

            let right_point = Point {
                x: current_point.x + 1,
                y: current_point.y + 1,
            };

            if self.get(right_point) == Cell::Empty {
                current_point = right_point;
                continue;
            }

            if self.get(right_point) == Cell::Abyss {
                return (current_point, Cell::Abyss);
            }

            self.set(current_point, Cell::Sand);
            return (current_point, self.get(down_point));
        }
    }

    fn drop_sand(&mut self) -> usize {
        let mut pebbles = 0;
        let mut drop = self.drop_pebble();
        while (drop.1 != Cell::Abyss) && !(drop.1 == Cell::Sand && drop.0 == self.source) {
            pebbles += 1;
            drop = self.drop_pebble();
        }

        pebbles
    }

    fn outside(&self, point: Point) -> bool {
        let Point { x, y } = point;

        x < self.start || x > self.end || y < 0 || y > self.height
    }

    fn get(&self, point: Point) -> Cell {
        let Point { x, y } = point;
        if self.outside(point) {
            return Cell::Abyss;
        }

        *self
            .grid
            .get(&Point { x, y })
            .or(Some(&Cell::Empty))
            .unwrap()
    }

    fn set(&mut self, point: Point, cell: Cell) {
        let Point { x, y } = point;

        if self.outside(point) {
            return;
        }

        self.grid.insert(Point { x, y }, cell);
    }
}

impl Display for SandGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..=self.height {
            for x in self.start..=self.end {
                write!(f, "{}", self.get(Point { x, y }))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input(path: &str) -> (usize, usize) {
    let file = File::open(path).expect("Unable to open file");
    let mut grid = SandGrid::new();
    io::BufReader::new(file).lines().for_each(|s| {
        let str = s.unwrap();
        let mut items = str.split(" -> ").collect::<Vec<_>>();
        let first = items[0];
        let last = items.pop().unwrap();

        let middle = items[1..].iter().flat_map(|s| [s, s]).collect::<Vec<_>>();
        let mut result = vec![first];
        result.extend(middle);
        result.push(last);
        let it = result.chunks(2).for_each(|c| {
            let p1 = c[0].split(",").collect::<Vec<_>>();
            let p2 = c[1].split(",").collect::<Vec<_>>();
            let from = Point {
                x: p1[0].parse().unwrap(),
                y: p1[1].parse().unwrap(),
            };

            let to = Point {
                x: p2[0].parse().unwrap(),
                y: p2[1].parse().unwrap(),
            };

            grid.add_rock_path(from, to);
        });
    });

    let mut grid2 = grid.clone();
    grid2.add_rock_path(
        Point {
            x: -10000,
            y: grid2.height + 2,
        },
        Point {
            x: 10000,
            y: grid2.height + 2,
        },
    );

    return (grid.drop_sand(), grid2.drop_sand());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sand_grid() {
        let mut grid = SandGrid::new();
        grid.add_rock_path(Point { x: 498, y: 4 }, Point { x: 498, y: 6 });
        grid.add_rock_path(Point { x: 498, y: 6 }, Point { x: 496, y: 6 });
        grid.add_rock_path(Point { x: 503, y: 4 }, Point { x: 502, y: 4 });
        grid.add_rock_path(Point { x: 502, y: 4 }, Point { x: 502, y: 9 });
        grid.add_rock_path(Point { x: 502, y: 9 }, Point { x: 494, y: 9 });

        for i in 0..=24 {
            grid.drop_pebble();
        }

        let display_str = read_to_string("src/specs/day14_img").unwrap();
        assert_eq!(grid.to_string(), display_str)
    }

    #[test]
    fn test_drop_all_sand() {
        let mut grid = SandGrid::new();
        grid.add_rock_path(Point { x: 498, y: 4 }, Point { x: 498, y: 6 });
        grid.add_rock_path(Point { x: 498, y: 6 }, Point { x: 496, y: 6 });
        grid.add_rock_path(Point { x: 503, y: 4 }, Point { x: 502, y: 4 });
        grid.add_rock_path(Point { x: 502, y: 4 }, Point { x: 502, y: 9 });
        grid.add_rock_path(Point { x: 502, y: 9 }, Point { x: 494, y: 9 });

        let count = grid.drop_sand();

        assert_eq!(count, 24)
    }

    #[test]
    fn test_parse_input() {
        let count = parse_input("src/specs/day14");
        assert_eq!(count, (24, 92))
    }
}

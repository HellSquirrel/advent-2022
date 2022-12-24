use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_input(path: &str) -> HashSet<(i32, i32, i32)> {
    let file = File::open(path).expect("Unable to open file");
    io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|s| {
            let vec = s
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (vec[0], vec[1], vec[2])
        })
        .collect::<HashSet<(i32, i32, i32)>>()
}

pub fn part_1(path: &str) -> usize {
    let result = parse_input(path);
    let mut count = 0;

    for (x, y, z) in result.iter() {
        let left = (*x - 1, *y, *z);
        let right = (*x + 1, *y, *z);
        let top = (*x, *y - 1, *z);
        let bottom = (*x, *y + 1, *z);
        let front = (*x, *y, *z - 1);
        let back = (*x, *y, *z + 1);

        let mut count_per_cube = 6;

        for (x, y, z) in vec![left, right, top, bottom, front, back] {
            if result.contains(&(x, y, z)) {
                count_per_cube -= 1;
            }
        }

        count += count_per_cube;
    }

    count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Face {
    direction: Direction,
    position: (i32, i32, i32),
}

impl Face {
    fn new(direction: Direction, position: (i32, i32, i32)) -> Self {
        Face {
            direction,
            position,
        }
    }

    fn get_neighbors(&self) -> HashSet<Face> {
        let (x, y, z) = self.position;
        match self.direction {
            Direction::Z => {
                let mut adjacet_faces = HashSet::from([
                    Face::new(Direction::Z, (x, y + 1, z)),
                    Face::new(Direction::Z, (x, y - 1, z)),
                    Face::new(Direction::Z, (x + 1, y, z)),
                    Face::new(Direction::Z, (x - 1, y, z)),
                ]);

                let Cube = Cube::new((x, y, z));
                let mut cube_faces = Cube.get_faces();
                cube_faces.remove(self);
                cube_faces.remove(&Face::new(Direction::Z, (x, y, z - 1)));

                adjacet_faces.extend(cube_faces);

                let mut cube_faces_2 = Cube::new((x, y, z + 1)).get_faces();
                cube_faces_2.remove(&Face::new(Direction::Z, (x, y, z + 1)));
                cube_faces_2.remove(self);

                adjacet_faces.extend(cube_faces_2);

                adjacet_faces
            }

            Direction::Y => {
                let mut adjacet_faces = HashSet::from([
                    Face::new(Direction::Y, (x, y, z + 1)),
                    Face::new(Direction::Y, (x, y, z - 1)),
                    Face::new(Direction::Y, (x + 1, y, z)),
                    Face::new(Direction::Y, (x - 1, y, z)),
                ]);

                let Cube = Cube::new((x, y, z));
                let mut cube_faces = Cube.get_faces();
                cube_faces.remove(self);
                cube_faces.remove(&Face::new(Direction::Y, (x, y - 1, z)));

                adjacet_faces.extend(cube_faces);

                let mut cube_faces_2 = Cube::new((x, y + 1, z)).get_faces();
                cube_faces_2.remove(&Face::new(Direction::Y, (x, y + 1, z)));
                cube_faces_2.remove(self);

                adjacet_faces.extend(cube_faces_2);

                adjacet_faces
            }

            Direction::X => {
                let mut adjacet_faces = HashSet::from([
                    Face::new(Direction::X, (x, y, z + 1)),
                    Face::new(Direction::X, (x, y, z - 1)),
                    Face::new(Direction::X, (x, y + 1, z)),
                    Face::new(Direction::X, (x, y - 1, z)),
                ]);

                let Cube = Cube::new((x, y, z));
                let mut cube_faces = Cube.get_faces();
                cube_faces.remove(self);
                cube_faces.remove(&Face::new(Direction::X, (x - 1, y, z)));

                adjacet_faces.extend(cube_faces);

                let mut cube_faces_2 = Cube::new((x + 1, y, z)).get_faces();
                cube_faces_2.remove(&Face::new(Direction::X, (x + 1, y, z)));
                cube_faces_2.remove(self);

                adjacet_faces.extend(cube_faces_2);

                adjacet_faces
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    coords: (i32, i32, i32),
}

impl Cube {
    fn new((x, y, z): (i32, i32, i32)) -> Self {
        Cube { coords: (x, y, z) }
    }

    fn get_faces(&self) -> HashSet<Face> {
        let mut faces = HashSet::new();
        let (x, y, z) = self.coords;
        faces.insert(Face {
            direction: Direction::X,
            position: self.coords,
        });

        faces.insert(Face {
            direction: Direction::X,
            position: (x - 1, y, z),
        });

        faces.insert(Face {
            direction: Direction::Y,
            position: (x, y, z),
        });

        faces.insert(Face {
            direction: Direction::Y,
            position: (x, y - 1, z),
        });

        faces.insert(Face {
            direction: Direction::Z,
            position: (x, y, z),
        });

        faces.insert(Face {
            direction: Direction::Z,
            position: (x, y, z - 1),
        });

        faces
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let (x, y, z) = self.coords;
        write!(f, "{}, {}, {}", x, y, z)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cluster {
    cubes: HashSet<Cube>,
    faces: HashSet<Face>,
}

impl Cluster {
    fn new() -> Self {
        Self {
            cubes: HashSet::new(),
            faces: HashSet::new(),
        }
    }

    fn from(c: Cube) -> Self {
        let mut cluster = Self::new();
        let faces = c.get_faces();
        cluster.push(c);
        cluster.faces.extend(faces);
        cluster
    }

    fn total_faces(&self) -> usize {
        self.faces.len()
    }

    fn push(&mut self, c: Cube) {
        for f in c.get_faces() {
            let contains = self.faces.remove(&f);
            if !contains {
                self.faces.insert(f);
            }
        }

        self.cubes.insert(c);
    }

    fn can_push(&mut self, c: Cube) -> bool {
        let faces = c.get_faces();
        for c in faces {
            if self.faces.contains(&c) {
                return true;
            }
        }

        false
    }

    fn can_merge(&self, other: &Cluster) -> bool {
        self.faces.intersection(&other.faces).count() > 0
    }

    fn merge(&mut self, other: Cluster) {
        for c in other.cubes {
            self.push(c);
        }
    }

    fn get_outer_faces(&self) -> HashSet<Face> {
        let mut outer = HashSet::new();
        let mut to_visit: Vec<Face> = Vec::new();

        let mut current_faces = self.faces.clone();
        let min_x_face = current_faces
            .iter()
            .filter(|x| x.direction == Direction::X)
            .min_by(|x, y| x.position.0.cmp(&y.position.0))
            .unwrap()
            .clone();

        current_faces.remove(&min_x_face);

        to_visit.push(min_x_face);

        while !to_visit.is_empty() {
            let current = to_visit.pop().unwrap();

            let neighbors = current.get_neighbors();
            for a in neighbors {
                // println!("Neighbor: {:?}", a);
                if current_faces.contains(&a) && !outer.contains(&a) {
                    current_faces.remove(&a);
                    outer.insert(a.clone());
                    to_visit.push(a);
                }
            }
        }

        outer
    }
}

impl Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Cluster: ")?;

        for c in self.cubes.iter() {
            write!(f, "({}), ", c)?;
        }

        Ok(())
    }
}

pub fn part_2(path: &str) -> usize {
    let result = parse_input(path);

    let mut initial_clusters = result
        .iter()
        .map(|x| Cluster::from(Cube::new(*x)))
        .collect::<Vec<_>>();

    'out: loop {
        let clusters = initial_clusters.clone();
        for c in clusters.iter() {
            for d in clusters.iter() {
                if c != d && c.can_merge(d) {
                    let mut new_cluster = c.clone();
                    new_cluster.merge(d.clone());
                    initial_clusters.retain(|x| x != d && x != c);
                    initial_clusters.push(new_cluster);
                    continue 'out;
                }
            }
        }

        break;
    }

    for c in &initial_clusters {
        let outer_edges = c.get_outer_faces();
        println!(
            "inner edges: {:?}",
            c.faces.difference(&c.get_outer_faces())
        );
    }

    initial_clusters
        .iter()
        .map(|x| x.get_outer_faces().iter().count())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = part_1("src/specs/day18");
        assert_eq!(input, 64);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = part_2("src/specs/day18");
    //     assert_eq!(input, 51);
    // }

    #[test]
    fn cluster() {
        let c1 = Cube::new((0, 0, 0));
        let c2 = Cube::new((1, 0, 0));

        let mut cluster = Cluster::from(c1);
        cluster.push(c2);

        assert_eq!(cluster.total_faces(), 10);

        assert_eq!(cluster.can_push(Cube::new((4, 3, 1))), false);
        assert_eq!(cluster.can_push(Cube::new((2, 2, 1))), false);
        assert_eq!(cluster.can_push(Cube::new((2, 2, 1))), false);
        assert_eq!(cluster.can_push(Cube::new((2, 0, 0))), true);
    }
}

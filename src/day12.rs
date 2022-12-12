use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct GridCell {
    x: usize,
    y: usize,
}

impl GridCell {
    fn new(x: usize, y: usize) -> GridCell {
        GridCell { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct GridCellData {
    elevation: usize,
    distance: usize,
}

impl GridCellData {
    fn new(elevation: usize, distance: usize) -> GridCellData {
        GridCellData {
            elevation,
            distance,
        }
    }
}

static BIIIG: usize = 1000;

pub fn parse_input(
    path: &str,
) -> (
    HashMap<GridCell, GridCellData>,
    GridCell,
    GridCell,
    usize,
    usize,
    Vec<GridCell>,
) {
    let file = File::open(path).expect("Unable to open file");
    let mut elevations = ('a'..='z').zip(1..).collect::<HashMap<_, _>>();
    let mut vertices: HashMap<GridCell, GridCellData> = HashMap::new();
    let mut from = GridCell::new(0, 0);
    let mut to = GridCell::new(0, 0);
    elevations.insert('E', *elevations.get(&'z').unwrap());
    elevations.insert('S', *elevations.get(&'a').unwrap());
    let mut cols = 0;
    let mut rows = 0;
    let mut starts = Vec::new();

    for (row, line) in io::BufReader::new(file).lines().enumerate() {
        if row + 1 >= row {
            rows = row + 1;
        }
        for (col, c) in line.unwrap().chars().enumerate() {
            if col + 1 >= cols {
                cols = col + 1;
            }

            vertices.insert(
                GridCell::new(col, row),
                GridCellData {
                    elevation: *elevations.get(&c).unwrap(),
                    distance: BIIIG,
                },
            );

            if c == 'S' {
                from = GridCell::new(col, row);
                starts.push(GridCell::new(col, row));
            }

            if c == 'E' {
                to = GridCell::new(col, row);
            }

            if c == 'a' {
                starts.push(GridCell::new(col, row));
            }
        }
    }

    (vertices, from, to, cols, rows, starts)
}

pub fn get_reachable_cells(
    from: GridCell,
    grid: &HashMap<GridCell, GridCellData>,
    elevation: usize,
    width: usize,
    height: usize,
) -> Vec<(GridCell, GridCellData)> {
    let mut reachable_cells: Vec<GridCell> = Vec::with_capacity(4);

    if from.x > 0 {
        reachable_cells.push(GridCell::new(from.x - 1, from.y));
    }

    if from.x < width - 1 {
        reachable_cells.push(GridCell::new(from.x + 1, from.y));
    }

    if from.y > 0 {
        reachable_cells.push(GridCell::new(from.x, from.y - 1));
    }

    if from.y < height - 1 {
        reachable_cells.push(GridCell::new(from.x, from.y + 1));
    }

    reachable_cells
        .iter()
        .flat_map(|cell| {
            grid.get(&cell).and_then(|cell_data| {
                if cell_data.elevation <= elevation + 1 {
                    Some((*cell, *cell_data))
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn relax(
    cell: GridCell,
    grid: &mut HashMap<GridCell, GridCellData>,
    width: usize,
    height: usize,
) -> (GridCell, GridCellData) {
    let data = grid.remove(&cell).unwrap();

    let reachable_cells = get_reachable_cells(cell, grid, data.elevation, width, height);
    // println!(
    //     "reachable cells {:#?} dist: {} for cell {:#?}",
    //     reachable_cells, data.distance, cell
    // );

    for (reachable_cell, reachable_data) in reachable_cells {
        let distance = data.distance + 1;
        if distance < reachable_data.distance {
            let new_data = GridCellData {
                elevation: reachable_data.elevation,
                distance,
            };
            grid.insert(reachable_cell, new_data);

            // println!("updating cell --> {:#?} {:#?}", reachable_cell, new_data);
        }
    }

    (cell, data)
}

pub fn extract_min(grid: &mut HashMap<GridCell, GridCellData>) -> Option<(GridCell, GridCellData)> {
    let item = grid
        .iter()
        .min_by(|(_, a), (_, b)| a.distance.cmp(&b.distance))
        .map(|(cell, data)| (*cell, *data));

    match item {
        Some((cell, data)) => Some((cell, data)),
        None => None,
    }
}

pub fn get_path(path: &str) -> usize {
    let (mut input, from, to, cols, rows, _) = parse_input(path);
    // println!("{:#?} {:#?}", cols, rows);
    let el = input.get_mut(&from).unwrap();
    el.distance = 0;

    // println!("{:?} {:?}", from, to);

    loop {
        let (cell, data) = extract_min(&mut input).unwrap();
        // println!("{:?} {:?}", cell, data);
        if cell == to {
            return data.distance;
        }

        relax(cell, &mut input, cols, rows);
    }

    BIIIG
}

// its better to revert requirements and start from the end :)
// but I'm too lazy to do it

pub fn get_path_part2(path: &str) -> usize {
    let (mut input, from, to, cols, rows, starts) = parse_input(path);
    // println!("{:#?} {:#?}", cols, rows);

    // println!("{:?} {:?}", from, to);
    let mut distances: Vec<usize> = Vec::with_capacity(starts.len());
    let mut i = 0;
    let len = starts.len();

    for start in starts {
        i += 1;
        println!("{i} of {}", len);
        let mut input = input.clone();
        let el = input.get_mut(&start).unwrap();
        el.distance = 0;

        loop {
            let (cell, data) = extract_min(&mut input).unwrap();
            // println!("{:?} {:?}", cell, data);
            if cell == to {
                distances.push(data.distance);
                break;
            }

            relax(cell, &mut input, cols, rows);
        }
    }

    distances.iter().min().unwrap().clone()
}

#[cfg(test)]
#[test]
fn test_reachable_cells() {
    let input = parse_input("src/specs/day12");
    let from = GridCell::new(2, 3);
    let elevation = input.0.get(&from).unwrap().elevation;
    let reachable_cells = get_reachable_cells(from, &input.0, elevation, input.3, input.4);
    assert_eq!(
        reachable_cells,
        vec![
            (GridCell::new(1, 3), GridCellData::new(3, BIIIG)),
            (GridCell::new(2, 2), GridCellData::new(3, BIIIG)),
            (GridCell::new(2, 4), GridCellData::new(4, BIIIG)),
        ]
    );
}

#[test]
fn test_relax() {
    let mut input = parse_input("src/specs/day12");
    let from = GridCell::new(0, 0);
    input.0.insert(from, GridCellData::new(19, 0));
    let (cell, data) = relax(from, &mut input.0, input.3, input.4);

    assert_eq!(cell, from);
    assert_eq!(data, GridCellData::new(19, 0));
    assert_eq!(
        input.0.get(&GridCell::new(1, 0)),
        Some(&GridCellData::new(1, 1))
    );

    assert_eq!(
        input.0.get(&GridCell::new(0, 1)),
        Some(&GridCellData::new(1, 1))
    );
}

#[test]
fn test_extract_min() {
    let mut input = parse_input("src/specs/day12");
    let from = GridCell::new(0, 0);
    input.0.insert(from, GridCellData::new(19, 0));
    let result = extract_min(&mut input.0).unwrap();

    assert_eq!(result.0, from);
}

#[test]
fn test_get_path() {
    let result = get_path("src/specs/day12");
    assert_eq!(result, 31);
}

#[test]
fn test_get_path_2() {
    let result = get_path_part2("src/specs/day12");
    assert_eq!(result, 29);
}

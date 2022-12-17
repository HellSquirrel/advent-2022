use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{stdout, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct BeaconAndSensor {
    beacon: Point,
    sensor: Point,
}

impl BeaconAndSensor {
    fn new(beacon: Point, sensor: Point) -> BeaconAndSensor {
        BeaconAndSensor { beacon, sensor }
    }

    fn max_distance(&self) -> i32 {
        self.sensor.distance(&self.beacon)
    }

    fn is_inside_scan_area(&self, point: &Point) -> bool {
        let distance = self.sensor.distance(point);
        distance <= self.max_distance()
    }

    fn should_be_empty(&self, point: &Point) -> bool {
        if self.is_inside_scan_area(point) && *point != self.beacon && *point != self.sensor {
            return true;
        }

        false
    }

    fn bounds(&self) -> (Point, Point) {
        let min_x = self.sensor.x - self.max_distance();
        let min_y = self.sensor.y - self.max_distance();
        let max_x = self.sensor.x + self.max_distance();
        let max_y = self.sensor.y + self.max_distance();

        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    fn row_range(&self, y: i32, x_min: i32, x_max: i32) -> Vec<(i32, i32)> {
        let mut v = Vec::with_capacity(1);
        let distance = self.max_distance() - (self.sensor.y - y).abs();
        let x = self.sensor.x;

        if distance < 0 {
            return v;
        } else {
            let p1 = x - distance;
            let p2 = x + distance;

            v.push((p1.max(x_min), p2.min(x_max)));
        }

        v
    }
}

fn get_unreachable_cells(beacons_and_sensors: &Vec<BeaconAndSensor>, y_pos: i32) -> HashSet<Point> {
    let mut unreachable_cells: HashSet<Point> = HashSet::new();

    for beacon_and_sensor in beacons_and_sensors {
        let distance = beacon_and_sensor.max_distance();
        let dy = (beacon_and_sensor.sensor.y - y_pos).abs();
        let start = beacon_and_sensor.sensor.x - distance + dy;
        let end = beacon_and_sensor.sensor.x + distance - dy;

        // println!(
        //     "Sensor: {:?} Beacon: {:?} Distance: {} Start: {} End: {}",
        //     beacon_and_sensor.sensor, beacon_and_sensor.beacon, distance, start, end
        // );

        for x in start..=end {
            let point = Point::new(x, y_pos);
            if beacon_and_sensor.should_be_empty(&point) {
                // println!(
                //     "Adding point: {:?} because of {:?}",
                //     point, beacon_and_sensor
                // );
                unreachable_cells.insert(point);
            }

            if (point == beacon_and_sensor.sensor) {
                unreachable_cells.insert(point);
            }
        }
    }

    for beacon_and_sensor in beacons_and_sensors {
        let beacon = beacon_and_sensor.beacon;
        unreachable_cells.remove(&beacon);
    }

    unreachable_cells
}

fn find_distress_beacon(
    beacons_and_sensors: &Vec<BeaconAndSensor>,
    p_min: Point,
    p_max: Point,
) -> Option<Point> {
    let x_min = p_min.x;
    let x_max = p_max.x;
    let y_min = p_min.y;
    let y_max = p_max.y;

    for y in y_min..=y_max {
        // print!("\r y: {}", y);
        // stdout().flush().unwrap();
        let ranges = get_all_ranges(beacons_and_sensors, y, x_min, x_max);
        if ranges.len() > 1 {
            // println!("result: {:?}", ranges);
            for e in ranges {
                return Some(Point::new(e.1 + 1, y));
            }
        }
    }

    None
}

fn parse_input(path: &str) -> Vec<BeaconAndSensor> {
    let file = File::open(path).expect("Unable to open file");
    let mut field_start = Point::new(0, 0);
    let mut field_end = Point::new(0, 0);

    io::BufReader::new(file)
        .lines()
        .filter_map(|i| i.ok())
        .filter_map(|l| {
            let regexp = Regex::new(r".*x=(-?\d+), y=(-?\d+):.*x=(-?\d+), y=(-?\d+)").unwrap();
            let captures = regexp.captures(&l).map(|cap| {
                let sx = cap[1].parse::<i32>().unwrap();
                let sy = cap[2].parse::<i32>().unwrap();

                let bx = cap[3].parse::<i32>().unwrap();
                let by = cap[4].parse::<i32>().unwrap();

                BeaconAndSensor::new(Point::new(bx, by), Point::new(sx, sy))
            });
            captures
        })
        .collect::<Vec<_>>()
}

pub fn part_1(path: &str) -> usize {
    let beacons_and_sensors = parse_input(path);
    get_unreachable_cells(&beacons_and_sensors, 2000000)
        .iter()
        .map(|p| p.x)
        .len()
}

fn merge_ranges(r1: (i32, i32), r2: (i32, i32)) -> Vec<(i32, i32)> {
    let (x_s1, x_e1) = r1;
    let (x_s2, x_e2) = r2;
    let mut ranges = Vec::with_capacity(2);

    let (first_range, second_range) = if x_s1 < x_s2 { (r1, r2) } else { (r2, r1) };

    if first_range.1 < second_range.0 {
        ranges.push(first_range);
        ranges.push(second_range);
    } else if first_range.1 <= second_range.1 {
        ranges.push((first_range.0, second_range.1));
    } else {
        ranges.push(first_range);
    }

    ranges
}

fn get_all_ranges(
    beacons_and_sensors: &Vec<BeaconAndSensor>,
    y: i32,
    x_min: i32,
    x_max: i32,
) -> HashSet<(i32, i32)> {
    let mut ranges: HashSet<(i32, i32)> = HashSet::new();

    for bas in beacons_and_sensors {
        let r = bas.row_range(y, x_min, x_max);
        if r.is_empty() {
            continue;
        }
        if r[0] == (x_min, x_max) {
            ranges.insert(r[0]);
            return ranges;
        }

        ranges.insert(r[0]);
    }

    '_out: loop {
        let clone = ranges.clone();
        for r in &clone {
            for k in &clone {
                if r != k {
                    let merged_ranges = merge_ranges(*r, *k);
                    if merged_ranges.len() == 1 {
                        ranges.remove(r);
                        ranges.remove(k);
                        ranges.insert(merged_ranges[0]);
                        continue '_out;
                    }
                }
            }
        }

        break;
    }

    ranges
}

pub fn part_2(path: &str, min: Point, max: Point) -> usize {
    let beacons_and_sensors = parse_input(path);
    let Point { x, y } = find_distress_beacon(&beacons_and_sensors, min, max).unwrap();

    (4000000 * x + y) as usize
}

#[cfg(test)]
#[test]
fn test_beacon_and_sensor_distance() {
    let sensor = Point::new(8, 7);
    let beacon = Point::new(2, 10);

    let bas = BeaconAndSensor::new(beacon, sensor);

    assert_eq!(bas.is_inside_scan_area(&sensor), true);
    assert_eq!(bas.is_inside_scan_area(&beacon), true);
    assert_eq!(bas.is_inside_scan_area(&Point::new(5, 7)), true);
    assert_eq!(bas.is_inside_scan_area(&Point::new(0, 11)), false);
}

#[test]
fn test_beacon_and_sensor_should_be_empty() {
    let sensor = Point::new(8, 7);
    let beacon = Point::new(2, 10);

    let bas = BeaconAndSensor::new(beacon, sensor);

    assert_eq!(bas.should_be_empty(&sensor), false);
    assert_eq!(bas.should_be_empty(&beacon), false);
    assert_eq!(bas.should_be_empty(&Point::new(8, 9)), true);
}

#[test]
fn test_bounds() {
    let sensor = Point::new(8, 7);
    let beacon = Point::new(2, 10);

    let bas = BeaconAndSensor::new(beacon, sensor);
    assert_eq!(bas.bounds(), (Point::new(-1, -2), Point::new(17, 16)));
}

#[test]
fn test_row_range() {
    let sensor = Point::new(8, 7);
    let beacon = Point::new(2, 10);

    let bas = BeaconAndSensor::new(beacon, sensor);

    assert_eq!(bas.row_range(17, 0, 20), vec![]);
    assert_eq!(bas.row_range(16, 0, 20), vec![(8, 8)]);
    assert_eq!(bas.row_range(12, 0, 20), vec![(4, 12)]);
    assert_eq!(bas.row_range(7, 0, 20), vec![(0, 17)]);
    assert_eq!(bas.row_range(11, -4, 20), vec![(3, 13)]);
}

#[test]
fn test_get_all_ranges() {
    let beacons_and_sensors = parse_input("src/specs/day15");
    let result2 = get_all_ranges(&beacons_and_sensors, 11, -4, 20);
    assert_eq!(result2, HashSet::from_iter(vec![(-3, 13), (15, 20)]));
}

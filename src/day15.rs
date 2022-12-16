use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
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

fn find_distress_beacon(beacons_and_sensors: &Vec<BeaconAndSensor>) -> Option<Point> {
    for current in beacons_and_sensors {
        for other in beacons_and_sensors {
            if current.beacon == other.beacon {
                continue;
            }

            if current.should_be_empty(&other.beacon) {
                return Some(current.beacon);
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

pub fn part1(path: &str) -> usize {
    let beacons_and_sensors = parse_input(path);
    get_unreachable_cells(&beacons_and_sensors, 2000000)
        .iter()
        .map(|p| p.x)
        .len()
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
fn test_find_distress_beacon() {
    let beacons_and_sensors = parse_input("src/specs/day15");
    let result = find_distress_beacon(&beacons_and_sensors);

    assert_eq!(result, Some(Point::new(5, 8)));
}

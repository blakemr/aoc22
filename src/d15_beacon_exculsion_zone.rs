use std::{collections::BTreeSet, ops::Range, str::FromStr};

type Position = (i32, i32);

impl Range<i32> {}

#[derive(Debug)]
pub struct ParseSensorError;

#[derive(Debug, Clone)]
pub struct Sensor {
    position: Position,
    range: i32,
    beacons: Vec<Position>,
}

impl FromStr for Sensor {
    type Err = ParseSensorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s.split_once(':').unwrap();

        // Sensor parse
        let mut sensor = sensor.strip_prefix("Sensor at x=").unwrap().split(',');
        let x: i32 = sensor.next().unwrap().parse().unwrap();
        let y: i32 = sensor
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();
        let position = (x, y);

        // Beacon parse
        let mut beacon = beacon
            .strip_prefix(" closest beacon is at x=")
            .unwrap()
            .split(',');
        let x: i32 = beacon.next().unwrap().parse().unwrap();
        let y: i32 = beacon
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();
        let beacon = (x, y);
        let beacons = vec![beacon];

        let range = (position.0 - beacon.0).abs() + (position.1 - beacon.1).abs();

        Ok(Sensor {
            position,
            range,
            beacons,
        })
    }
}

impl Sensor {
    fn coverage_y(self, row: i32) -> Option<Range<i32>> {
        let distance = (self.position.1 - row).abs();

        match (self.range as i32).cmp(&distance) {
            std::cmp::Ordering::Less => None,
            std::cmp::Ordering::Equal => Some(Range {
                start: self.position.0,
                end: self.position.0 + 1,
            }),
            std::cmp::Ordering::Greater => Some(Range {
                start: self.position.0 - (self.range - distance),
                end: self.position.0 + (self.range - distance) + 1,
            }),
        }
    }

    fn uncovered_y(self, row: i32, range: Range<i32>) -> Option<Vec<Range<i32>>> {
        let Some(covered) = self.coverage_y(row) else { return Some(vec![range]) };

        if covered.contains(&range.start) && covered.contains(&range.end) {
            return None;
        } else if range.contains(&covered.start) && range.contains(&covered.end) {
            return Some(vec![
                [range.start..covered.start],
                [covered.end + 1..range.end],
            ]);
        } else if covered.contains(&range.start) {
            return Some(vec![covered.end + 1..range.end]);
        } else if covered.contains(&range.end) {
            return Some(vec![range.start..covered.start]);
        } else {
            return Some(vec![range]);
        }
    }

    fn in_range(self, point: Position) -> bool {
        self.position.0.abs_diff(point.0) + self.position.1.abs_diff(point.1) <= self.range as u32
    }
}

fn no_coverage_y(
    sensors: Vec<Sensor>,
    beacons: BTreeSet<Position>,
    row_region: BTreeSet<i32>,
    row: i32,
) -> Option<Range<i32>> {
    let mut undetected = row_region;
    sensors.iter().for_each(|sensor| {
        undetected
            .splice(sensor.coverage_y(row), Vec::new())
            .collect()
    })
}

fn coverage_y(sensors: Vec<Sensor>, beacons: BTreeSet<Position>, row: i32) -> usize {
    let coverage: BTreeSet<i32> = sensors
        .iter()
        .flat_map(|s| {
            s.clone()
                .coverage_y(row)
                .unwrap_or(Range { start: 0, end: 0 })
        })
        .collect();

    let row_beacons: BTreeSet<i32> = beacons
        .iter()
        .filter(|(_, y)| y == &row)
        .map(|(x, _)| *x)
        .collect();

    coverage
        .difference(&row_beacons)
        .collect::<BTreeSet<&i32>>()
        .len()
}

pub fn part_1(input: &str, row: i32) -> usize {
    let sensors = parse(input);
    let beacons: BTreeSet<Position> = sensors.iter().flat_map(|s| s.beacons.clone()).collect();
    coverage_y(sensors, beacons, row)
}

pub fn part_2(input: &str, min: i32, max: i32) -> u32 {
    let sensors = parse(input);
    let beacons: BTreeSet<Position> = sensors.iter().flat_map(|s| s.beacons.clone()).collect();
}

pub fn parse(input: &str) -> Vec<Sensor> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d15_beacon_exculsion_zone.test";
    const INPUT: &str = "src\\d15_beacon_exculsion_zone.input";

    #[test]
    fn check_parse() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        dbg!(parse(&input));
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_1(&input, 10), 26)
    }

    #[test]
    fn run_part_1() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_1(&input, 2_000_000))
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_2(&input, 0, 10), 0)
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_2(&input, 0, 4_000_000))
    }
}

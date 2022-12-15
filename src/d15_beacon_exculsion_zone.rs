use std::{collections::BTreeSet, ops::Range, str::FromStr};

type Position = (i64, i64);

enum RangeDifference<T> {
    Reduce(Range<T>),
    Split((Range<T>, Range<T>)),
    NoDiff,
    None,
}

trait RangeExt<T> {
    fn join(&self, other: &Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn difference(&self, other: &Self) -> RangeDifference<T>;
}

impl<T: Copy> RangeExt<T> for Range<T> {
    fn join(&self, other: &Self) -> Option<Self> {
        if self.start.max(other.start) >= self.end.max(other.end) {
            Some(Range {
                start: self.start.min(other.start),
                end: self.start.max(other.start),
            })
        } else {
            None
        }
    }

    fn difference(&self, other: &Self) -> RangeDifference<T> {
        if other.contains(&self.start) && other.contains(&self.end) {
            RangeDifference::None
        } else if self.contains(&other.start)
            && self.contains(&other.end)
            && self.start != other.start
        {
            RangeDifference::Split((
                Range {
                    start: self.start,
                    end: other.start,
                },
                Range {
                    start: other.end,
                    end: self.end,
                },
            ))
        } else if other.contains(&self.start) {
            RangeDifference::Reduce(Range {
                start: other.end,
                end: self.end,
            })
        } else if other.contains(&(self.end - 1)) {
            RangeDifference::Reduce(Range {
                start: self.start,
                end: other.start,
            })
        } else {
            RangeDifference::NoDiff
        }
    }
}

#[derive(Debug)]
pub struct ParseSensorError;

#[derive(Debug, Clone)]
pub struct Sensor {
    position: Position,
    range: i64,
    beacons: Vec<Position>,
}

impl FromStr for Sensor {
    type Err = ParseSensorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s.split_once(':').unwrap();

        // Sensor parse
        let mut sensor = sensor.strip_prefix("Sensor at x=").unwrap().split(',');
        let x: i64 = sensor.next().unwrap().parse().unwrap();
        let y: i64 = sensor
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
        let x: i64 = beacon.next().unwrap().parse().unwrap();
        let y: i64 = beacon
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
    fn coverage_y(self, row: i64) -> Option<Range<i64>> {
        let distance = (self.position.1 - row).abs();

        match (self.range as i64).cmp(&distance) {
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

    fn uncovered_y(&self, row: i64, range: &Range<i64>) -> RangeDifference<Range<i64>> {
        let Some(covered) = self.coverage_y(row) else { return RangeDifference::NoDiff };
        range.difference(&covered)
    }
}

fn no_coverage_y(sensors: &[Sensor], row_region: &Range<i64>, row: i64) -> Option<(i64, i64)> {
    let mut undetected = vec![row_region];
    sensors.iter().for_each(|sensor| {
        // dbg!(sensor.clone().coverage_y(row));
        let mut new_segments = Vec::new();
        for segment in undetected {
            match sensor.uncovered_y(row, segment) {
                RangeDifference::Reduce(x) => new_segments.push(x),
                RangeDifference::Split(x) => {
                    new_segments.push(x.0);
                    new_segments.push(x.1);
                }
                RangeDifference::NoDiff => new_segments.push(segment),
                RangeDifference::None => {}
            }
        }
        undetected = new_segments;
    });

    Some((undetected.first()?.start, row))
}

fn coverage_y(sensors: Vec<Sensor>, beacons: BTreeSet<Position>, row: i64) -> usize {
    let coverage: BTreeSet<i64> = sensors
        .iter()
        .flat_map(|s| {
            s.clone()
                .coverage_y(row)
                .unwrap_or(Range { start: 0, end: 0 })
        })
        .collect();

    let row_beacons: BTreeSet<i64> = beacons
        .iter()
        .filter(|(_, y)| y == &row)
        .map(|(x, _)| *x)
        .collect();

    coverage
        .difference(&row_beacons)
        .collect::<BTreeSet<&i64>>()
        .len()
}

pub fn part_1(input: &str, row: i64) -> usize {
    let sensors = parse(input);
    let beacons: BTreeSet<Position> = sensors.iter().flat_map(|s| s.beacons.clone()).collect();
    coverage_y(sensors, beacons, row)
}

pub fn part_2(input: &str, min: i64, max: i64) -> Option<i64> {
    let sensors = parse(input);

    for row in min..=max {
        if let Some((x, y)) = no_coverage_y(&sensors, &(min..max), row) {
            return Some(x * 4_000_000 + y);
        }
    }
    None
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

        assert_eq!(part_2(&input, 0, 20), Some(56000011))
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_2(&input, 0, 4_000_000))
    }
}

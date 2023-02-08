use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct ParseNodeError;

#[derive(Debug)]
pub struct Valve {
    flow: u32,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let flow = s
            .split(';')
            .next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let tunnels = s
            .split("valves")
            .nth(1)
            .unwrap_or(s.split("valve").nth(1).unwrap())
            .split_whitespace()
            .map(|tun| tun.trim_end_matches(',').to_string())
            .collect();

        Ok(Valve { flow, tunnels })
    }
}

pub fn part_1(input: &str) -> u32 {
    let start = "AA";
    let mut time = 30;
    let valve_time = 1;
    let travel_time = 1;

    let valves = parse(input);

    let flow = 0;

    while time > 0 {
        todo!()
    }

    flow
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

pub fn parse(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|line| {
            (
                line.split_whitespace().nth(1).unwrap().to_string(),
                line.parse::<Valve>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d16_proboscidea_volcanium.test";
    const INPUT: &str = "src\\d16_proboscidea_volcanium.input";

    #[test]
    fn check_parse() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        dbg!(parse(&input));
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_1(&input), 13)
    }

    #[test]
    fn run_part_1() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_1(&input))
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_2(&input), 0)
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_2(&input))
    }
}

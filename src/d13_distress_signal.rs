use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
enum SignalPacket {
    List(Vec<SignalPacket>),
    Int(u32),
}

impl FromStr for SignalPacket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split into [] sets, then recurse
        match s.chars().into_iter().next() {
            Some('[') => {
                if let (list, other) = s.rsplit_once(']') {
                    todo!()
                }
            }
            Some(_) => {
                let ns = s.split(',');

                let packets = ns
                    .map(|p| match u32::try_into(p) {
                        Some(n) => SignalPacket::Int(n),
                        None => p::<SignalPacket>.parse(),
                    })
                    .collect();

                Ok(SignalPacket::List(todo!()))
            }
            None => panic!("No chars!"),
        }
    }
}

#[derive(Debug)]
pub struct Signal {
    left: SignalPacket,
    right: SignalPacket,
}

impl FromStr for Signal {
    type Err = ParseError;

    // s is a single pair of lines.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (left, right) = (
            lines.next().unwrap().parse().unwrap(),
            lines.next().unwrap().parse().unwrap(),
        );

        Ok(Signal { left, right })
    }
}

pub fn part_1(input: &str) -> u32 {
    let signals = parse(input);

    todo!()
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

pub fn parse(input: &str) -> Vec<Signal> {
    let signals = input.split("\n\r\n");

    signals
        .into_iter()
        .map(|pair| pair.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d13_distress_signal.test";
    const INPUT: &str = "src\\d13_distress_signal.input";

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

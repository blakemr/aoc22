use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
struct ParseSignalError;

#[derive(Debug, Clone, PartialEq)]
enum SignalPacket {
    List(Vec<SignalPacket>),
    Int(u32),
}

impl FromStr for SignalPacket {
    type Err = ParseSignalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut packet_stack = Vec::new();
        let ch_iter = s.chars();
        let mut pay = String::new();

        for ch in ch_iter {
            match ch {
                '[' => packet_stack.push(SignalPacket::List(Vec::new())),
                ']' => {
                    if !pay.is_empty() {
                        let parent = packet_stack.pop().unwrap();
                        if let SignalPacket::List(mut par) = parent {
                            par.push(SignalPacket::Int(pay.parse().unwrap()));
                            packet_stack.push(SignalPacket::List(par));
                            pay.clear();
                        } else {
                            panic!("Non-List Signal in stack after ','!");
                        }
                    }

                    let elem = packet_stack.pop().unwrap();
                    if let Some(parent) = packet_stack.pop() {
                        if let SignalPacket::List(mut par) = parent {
                            par.push(elem);
                            packet_stack.push(SignalPacket::List(par));
                        } else {
                            panic!("Non-List Signal in stack after ']'!");
                        }
                    } else {
                        return Ok(elem);
                    }
                }
                ',' => {
                    if !pay.is_empty() {
                        let parent = packet_stack.pop().unwrap();
                        if let SignalPacket::List(mut par) = parent {
                            par.push(SignalPacket::Int(pay.parse().unwrap()));
                            packet_stack.push(SignalPacket::List(par));
                            pay.clear();
                        } else {
                            panic!("Non-List Signal in stack after ','!");
                        }
                    }
                }
                _ => pay.push(ch),
            }
        }

        panic!("Unclosed bracket detected!");
    }
}

impl PartialOrd for SignalPacket {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SignalPacket::Int(x), SignalPacket::Int(y)) => x.partial_cmp(y),
            (SignalPacket::List(x), SignalPacket::List(y)) => x.partial_cmp(y),
            (SignalPacket::List(x), SignalPacket::Int(y)) => SignalPacket::List(x.clone())
                .partial_cmp(&SignalPacket::List(vec![SignalPacket::Int(*y)])),
            (SignalPacket::Int(x), SignalPacket::List(y)) => {
                SignalPacket::List(vec![SignalPacket::Int(*x)])
                    .partial_cmp(&SignalPacket::List(y.clone()))
            }
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

pub fn part_1(input: &str) -> usize {
    let signals = parse(input);

    let mut ind = 0;
    for (i, signal) in signals.iter().enumerate() {
        if signal.left < signal.right {
            ind += i + 1;
        }
    }

    ind
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

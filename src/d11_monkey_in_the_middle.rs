use std::{collections::VecDeque, str::FromStr, string::ParseError};

#[derive(Debug)]
enum InspectOp {
    Add(u64),
    Mul(u64),
    Squ,
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    op: InspectOp,
    throw_n: usize,
    throw_true: usize,
    throw_false: usize,
    inpections: u64,
}

impl Monkey {
    fn throw(&mut self) -> Option<(usize, u64)> {
        if let Some(item) = self.items.pop_front() {
            if item % self.throw_n as u64 == 0 {
                Some((self.throw_true, item))
            } else {
                Some((self.throw_false, item))
            }
        } else {
            None
        }
    }

    fn inspect(&mut self, relief: u64) {
        if let Some(item) = self.items.front_mut() {
            match self.op {
                InspectOp::Add(x) => *item += x,
                InspectOp::Mul(x) => *item *= x,
                InspectOp::Squ => *item = item.pow(2),
            }

            *item %= relief;
            self.inpections += 1;
        }
    }

    fn relief(&mut self, amount: u64) {
        if let Some(item) = self.items.front_mut() {
            *item /= amount;
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    // FORMAT
    // ---------------------------------------------------
    // Monkey X:
    //   Starting items: 0, 1, ...
    //   Operation: new = old * x OR old + x OR old * old
    //   Test: divisible by N
    //     If true: throw to monkey Y
    //     If false: throw to monkey Z

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.lines();
        input.next(); // Drop: Monkey X:

        //   Starting items: 0, 1, ...
        let items: VecDeque<u64> = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        //   Operation: new = old * x OR old + x OR old * old
        let mut op_split = input
            .next()
            .unwrap()
            .split_once("new = old ")
            .unwrap()
            .1
            .split_whitespace();
        let op = match op_split.next().unwrap() {
            "+" => InspectOp::Add(op_split.next().unwrap().parse::<u64>().unwrap()),
            "*" => match op_split.next() {
                Some("old") => InspectOp::Squ,
                Some(n) => InspectOp::Mul(n.parse::<u64>().unwrap()),
                _ => panic!(),
            },
            _ => panic!(),
        };

        //   Test: divisible by N
        let throw_n: usize = input
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        //     If true: throw to monkey Y
        let throw_true: usize = input
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        //     If false: throw to monkey Z
        let throw_false: usize = input
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            op,
            throw_n,
            throw_true,
            throw_false,
            inpections: 0,
        })
    }
}

pub fn part_1(input: &str, rounds: usize, relief: u64) -> u64 {
    let mut monkeys = parse(input);

    for _ in 0..rounds {
        // Round
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                monkeys[i].inspect(relief);
                monkeys[i].relief(3);
                let toss = monkeys[i].throw().unwrap();
                monkeys[toss.0 as usize].items.push_back(toss.1);
            }
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|i| i.inpections).collect();

    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

pub fn part_2(input: &str, rounds: usize, relief: u64) -> u64 {
    let mut monkeys = parse(input);

    for _ in 0..rounds {
        // Round
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                monkeys[i].inspect(relief);
                let toss = monkeys[i].throw().unwrap();
                monkeys[toss.0 as usize].items.push_back(toss.1);
            }
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|i| i.inpections).collect();

    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

pub fn parse(input: &str) -> Vec<Monkey> {
    let monkeys = input.split("\n\r\n");

    monkeys
        .into_iter()
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d11_monkey_in_the_middle.test";
    const INPUT: &str = "src\\d11_monkey_in_the_middle.input";

    const TEST_MAGIC_BS: u64 = 23 * 19 * 13 * 17;
    const MAGIC_BS: u64 = 17 * 13 * 19 * 7 * 11 * 3 * 2 * 5;

    #[test]
    fn check_parse() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        dbg!(parse(&input));
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_1(&input, 20, TEST_MAGIC_BS), 10605)
    }

    #[test]
    fn run_part_1() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_1(&input, 20, MAGIC_BS))
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_2(&input, 10_000, TEST_MAGIC_BS), 2713310158)
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_2(&input, 10_000, MAGIC_BS))
    }
}

use std::str::FromStr;

pub struct ParseNodeError;

pub struct Valve<'a> {
    flow: u64,
    connections: Vec<&'a Self>,
}

impl<'a> FromStr for Valve<'a> {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub fn part_1(input: &str) -> u32 {
    todo!()
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

pub fn parse(input: &str) -> () {
    todo!()
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

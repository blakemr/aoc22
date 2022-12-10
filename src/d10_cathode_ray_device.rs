pub fn part_1(input: &str) -> i32 {
    let commands = parse(input);
    let mut output: i32 = 0;
    let mut sum: i32 = 1;
    let imp_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    for (i, cmd) in commands.iter().enumerate() {
        let cycle: i32 = i as i32 + 1; // The cycles start at 1
        if imp_cycles.contains(&cycle) {
            output += sum * cycle;
        }
        match cmd {
            Some(x) => sum += x,
            None => {}
        }
    }

    output
}

pub fn part_2(input: &str) -> String {
    let commands = parse(input);
    let mut output: String = String::new();
    let mut sum: i32 = 1;

    for (i, cmd) in commands.iter().enumerate() {
        if (i as i32 % 40 - sum).abs() <= 1 {
            output.push('#')
        } else {
            output.push('.')
        }

        match cmd {
            Some(x) => sum += x,
            None => {}
        }
    }

    output
}

pub fn parse(input: &str) -> Vec<Option<i32>> {
    input
        .lines()
        .into_iter()
        .flat_map(|line| {
            let mut split = line.split_whitespace();
            match split.next() {
                Some("addx") => vec![None, Some(split.next().unwrap().parse().unwrap())],
                Some("noop") => vec![None],
                _ => panic!("unknown command!"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d10_cathode_ray_device.test";
    const INPUT: &str = "src\\d10_cathode_ray_device.input";

    #[test]
    fn check_parse() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        dbg!(parse(&input));
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_1(&input), 13140)
    }

    #[test]
    fn run_part_1() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_1(&input))
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        let mut output = part_2(&input);

        while !output.is_empty() {
            let remainder = output.split_off(40);
            println!("{}", output);
            output = remainder;
        }
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        let mut output = part_2(&input);

        while !output.is_empty() {
            let remainder = output.split_off(40);
            println!("{}", output);
            output = remainder;
        }
    }
}

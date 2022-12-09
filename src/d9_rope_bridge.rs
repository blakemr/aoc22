use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    tail_history: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        let mut tail_history = HashSet::new();
        tail_history.insert((0, 0));

        Rope {
            head: (0, 0),
            tail: (0, 0),
            tail_history,
        }
    }

    fn move_head(&mut self, dir: Direction, len: i32) {
        match dir {
            Direction::Down => self.head.1 -= len,
            Direction::Up => self.head.1 += len,
            Direction::Left => self.head.0 -= len,
            Direction::Right => self.head.0 += len,
        }

        self.move_tail();
    }

    fn move_tail(&mut self) {
        let mut dx = self.head.0 - self.tail.0;
        let mut dy = self.head.1 - self.tail.1;

        while dx.abs() > 1 {
            self.tail.0 += dx.signum();
            self.tail.1 += dy;

            dx = self.head.0 - self.tail.0;
            dy = self.head.1 - self.tail.1;
            self.tail_history.insert(self.tail);
        }

        while dy.abs() > 1 {
            self.tail.0 += dx;
            self.tail.1 += dy.signum();

            dx = self.head.0 - self.tail.0;
            dy = self.head.1 - self.tail.1;
            self.tail_history.insert(self.tail);
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let commands = parse(input);
    let mut rope = Rope::new();

    for command in commands {
        rope.move_head(command.0, command.1);
    }

    rope.tail_history.len()
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

fn parse(input: &str) -> Vec<(Direction, i32)> {
    let mut commands = Vec::<(Direction, i32)>::new();
    for line in input.lines() {
        let mut split_line = line.split_whitespace();

        let direction = match split_line.next() {
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            _ => panic!("Unknown command!"),
        };

        let amount: i32 = split_line.next().unwrap().parse().unwrap();

        commands.push((direction, amount));
    }

    commands
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d9_rope_bridge.test";
    const INPUT: &str = "src\\d9_rope_bridge.txt";

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

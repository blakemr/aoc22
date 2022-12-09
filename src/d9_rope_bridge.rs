use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Rope {
    segments: Vec<RopeSegment>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut segments = Vec::new();
        segments.resize(length, RopeSegment::new());
        Rope { segments }
    }

    fn get_tail_history(&self) -> BTreeSet<(i32, i32)> {
        self.segments.last().unwrap().tail_history.clone()
    }

    fn move_head(&mut self, dir: Direction, len: i32) {
        let head = self.segments.first().unwrap();

        let mut x = match dir {
            Direction::Up => head.head.0,
            Direction::Down => head.head.0,
            Direction::Left => head.head.0 - len,
            Direction::Right => head.head.0 + len,
        };

        let mut y = match dir {
            Direction::Up => head.head.1 + len,
            Direction::Down => head.head.1 - len,
            Direction::Left => head.head.1,
            Direction::Right => head.head.1,
        };

        self.segments.iter_mut().for_each(|segmnent| {
            segmnent.put_head(x, y);
            x = segmnent.tail.0;
            y = segmnent.tail.1;
        });
    }
}

#[derive(Debug, Clone)]
struct RopeSegment {
    head: (i32, i32),
    tail: (i32, i32),
    tail_history: BTreeSet<(i32, i32)>,
}

impl RopeSegment {
    fn new() -> Self {
        let mut tail_history = BTreeSet::new();
        tail_history.insert((0, 0));

        RopeSegment {
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

    fn put_head(&mut self, x: i32, y: i32) {
        self.head = (x, y);
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
    let mut rope = RopeSegment::new();

    for command in commands {
        rope.move_head(command.0, command.1);
    }

    rope.tail_history.len()
}

pub fn part_2(input: &str, length: usize) -> usize {
    let commands = parse(input);
    let mut rope = Rope::new(length);

    for command in commands {
        rope.move_head(command.0, command.1);
    }

    rope.get_tail_history().len()
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
    const TEST_INPUT_2: &str = "src\\d9_rope_bridge.test2";
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
    fn test_rope() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_2(&input, 1), 13)
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();
        assert_eq!(part_2(&input, 9), 1);
    }

    #[test]
    fn test_part_2_large_input() {
        let input = fs::read_to_string(TEST_INPUT_2).unwrap();
        assert_eq!(part_2(&input, 9), 36);
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_2(&input, 9))
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MOVES: &str = "src\\d5_supply_stacks_moves.txt";
const START: &str = "src\\d5_supply_stacks_start.csv";

#[derive(Debug)]
struct MoveOrder {
    amount: usize,
    start: usize,
    end: usize,
}

impl MoveOrder {
    fn move_crate(self, crates: &mut [Vec<char>]) {
        for _ in 0..self.amount {
            let Some(c) = crates[self.start].pop() else {panic!("No crate to move!")};
            crates[self.end].push(c);
        }
    }

    fn move_multple_crates(self, crates: &mut [Vec<char>]) {
        let mut c = crates[self.start].split_off(crates[self.start].len() - self.amount);
        crates[self.end].append(&mut c);
    }
}

pub fn find_top_crates() -> String {
    let moves = import_moves();
    let mut crates = import_start();

    for order in moves {
        order.move_crate(&mut crates);
    }

    let mut top_crates = String::new();
    for mut stack in crates {
        let Some(top) = stack.pop() else {panic!("Empty stack!")};
        top_crates.push(top);
    }

    top_crates
}

pub fn find_new_top_crates() -> String {
    let moves = import_moves();
    let mut crates = import_start();

    for order in moves {
        order.move_multple_crates(&mut crates);
    }

    let mut top_crates = String::new();
    for mut stack in crates {
        let Some(top) = stack.pop() else {panic!("Empty stack!")};
        top_crates.push(top);
    }

    top_crates
}

fn import_moves() -> Vec<MoveOrder> {
    let file = File::open(MOVES).expect("Error opening file.");
    let reader = BufReader::new(file);

    let mut orders = Vec::<MoveOrder>::new();
    for line in reader.lines() {
        let lin = line.expect("Error reading line.");

        let split_line: Vec<&str> = lin.split(' ').collect();

        #[allow(dead_code)]
        enum LineSegments {
            Move,
            MoveAmount,
            From,
            StartStack,
            To,
            EndStack,
        }

        let amount: usize = split_line[LineSegments::MoveAmount as usize]
            .parse()
            .unwrap();
        let mut start: usize = split_line[LineSegments::StartStack as usize]
            .parse()
            .unwrap();
        let mut end: usize = split_line[LineSegments::EndStack as usize].parse().unwrap();

        start -= 1;
        end -= 1;

        orders.push(MoveOrder { amount, start, end });
    }

    orders
}

fn import_start() -> Vec<Vec<char>> {
    let file = File::open(START).expect("Error opening file.");
    let reader = BufReader::new(file);

    let mut start_position = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        let crates: Vec<char> = line
            .expect("unable to read line.")
            .replace(',', "")
            .chars()
            .collect();

        start_position.push(crates);
    }

    start_position
}

#[cfg(test)]
mod tests {
    use super::{find_new_top_crates, find_top_crates, import_moves, import_start, MoveOrder};

    #[test]
    fn check_moves_import() {
        println!("{:?}", import_moves());
    }

    #[test]
    fn check_start_import() {
        println!("{:?}", import_start());
    }

    #[test]
    fn test_move_crate() {
        let mut start = vec![vec!['a', 'b', 'c'], vec!['x', 'y', 'z']];
        let order = MoveOrder {
            amount: 2,
            start: 1 - 1,
            end: 2 - 1,
        };

        order.move_crate(&mut start);

        assert_eq!(start[1], vec!['x', 'y', 'z', 'c', 'b']);
    }

    #[test]
    fn check_top_crates() {
        println!("{}", find_top_crates());
    }

    #[test]
    fn check_new_top_crates() {
        println!("{}", find_new_top_crates());
    }
}

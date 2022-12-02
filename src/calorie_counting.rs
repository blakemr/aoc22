// https://adventofcode.com/2022/day/1
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn result() {
    let inv = read_inventory();
    let max_inv = count_calories(inv);
    print!("{}", max_inv);
}

fn count_calories(inventory: Vec<Vec<usize>>) -> usize {
    let mut max_inv: usize = 0;
    for inv in inventory {
        max_inv = max(inv.iter().sum(), max_inv);
    }

    max_inv
}

fn read_inventory() -> Vec<Vec<usize>> {
    let filepath = "src\\calorie_counting_input.txt";
    let file = File::open(filepath).expect("Error opening file.");
    let reader = BufReader::new(file);

    let mut inv_vec = Vec::<Vec<usize>>::new();
    let mut current_inv = Vec::<usize>::new();
    for line in reader.lines() {
        match line {
            Ok(line) => match line.parse::<usize>() {
                Ok(value) => current_inv.push(value),
                Err(_) => {
                    inv_vec.push(current_inv.clone());
                    current_inv.clear();
                }
            },
            Err(error) => panic!("{}", error),
        }
    }

    inv_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_entry() {
        let inv = vec![vec![100, 200]];
        assert_eq!(count_calories(inv), 300);
    }

    #[test]
    fn two_entries() {
        let inv = vec![vec![100, 200], vec![200, 600]];
        assert_eq!(count_calories(inv), 800);
    }

    #[test]
    fn read_bytes() {
        for line in read_inventory() {
            print!("{:?}\n", line);
        }
    }

    #[test]
    fn get_result() {
        result();
    }
}

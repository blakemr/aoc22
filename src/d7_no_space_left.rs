use std::collections::HashMap;

pub fn sum_small_files(data: &str) -> u32 {
    let mut filesystem: HashMap<&str, u32> = HashMap::new();
    let mut dirstack: Vec<&str> = Vec::new();

    for line in data.lines() {
        let mut line_parts = line.split_whitespace();

        match line_parts.next() {
            Some("$") => match line_parts.next() {
                Some("cd") => match line_parts.next() {
                    Some("..") => {
                        let dir = dirstack.pop().expect("Empty stack on cd!");
                        let dir_size = *filesystem.get(dir).unwrap_or(&0);

                        filesystem
                            .entry(dirstack.last().expect("Stack Empty!"))
                            .and_modify(|n| *n += dir_size)
                            .or_insert(dir_size);
                    }
                    Some(dir) => dirstack.push(dir),
                    None => panic!("'cd' missing dir!"),
                },
                Some(_) => {}
                None => panic!("'$' missing command!"),
            },
            Some("dir") => {}
            Some(i) => {
                let size = i.parse::<u32>().expect("Failed to parse into number!");
                filesystem
                    .entry(dirstack.last().expect("Stack Empty!"))
                    .and_modify(|n| *n += size)
                    .or_insert(size);
            }
            None => panic!("Missing line!"),
        }
    }

    filesystem.values().filter(|&n| *n <= 100_000).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    const TEST_INPUT: &str = "src\\d7_no_space_left.test";
    const INPUT: &str = "src\\d7_no_space_left.txt";

    #[test]
    fn check_test_solution() {
        let mut input = String::new();
        File::open(TEST_INPUT)
            .expect("Failed to open!")
            .read_to_string(&mut input)
            .expect("Failed to read file!");

        assert_eq!(sum_small_files(&input), 95437);
    }

    #[test]
    fn check_part_1() {
        let mut input = String::new();
        File::open(INPUT)
            .expect("Failed to open!")
            .read_to_string(&mut input)
            .expect("Failed to read file!");

        println!("{:?}", sum_small_files(&input));
    }
}

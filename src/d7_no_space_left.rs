use std::collections::BTreeSet;

pub fn sum_small_files(data: &str, max_size: u32) -> u32 {
    // I need a stack. When the value gets popped off the stack, add it to the top of the stack. if that value is small, add it to my total
    let mut total = 0;
    let mut dirstack = Vec::<u32>::new();

    for line in data.lines() {
        let mut line_parts = line.split_whitespace();

        match line_parts.next() {
            Some("$") => match line_parts.next() {
                Some("cd") => match line_parts.next() {
                    Some("..") => {
                        let v = dirstack.pop().unwrap();
                        if v <= max_size {
                            total += v;
                        }
                        if !dirstack.is_empty() {
                            let i = dirstack.len() - 1;
                            dirstack[i] += v;
                        }
                    }
                    Some(_) => dirstack.push(0),
                    None => panic!("'cd' missing dir!"),
                },
                Some(_) => {} // don't care about ls
                None => panic!("'$' missing command!"),
            },
            Some("dir") => {} // don't care about dir in ls
            Some(i) => {
                let size = i.parse::<u32>().expect("Failed to parse into number!");
                let i = dirstack.len() - 1;
                dirstack[i] += size;
            }
            None => panic!("Missing line!"),
        }
    }

    while let Some(dir) = dirstack.pop() {
        if dir <= max_size {
            total += dir;
        }
    }
    total
}

pub fn find_smallest_useful_dir(data: &str, total_space: u32, needed_space: u32) -> u32 {
    let sizes = list_directory_sizes(data);
    let total_used: u32 = *sizes.iter().last().unwrap();

    for size in sizes {
        if total_used - size <= total_space - needed_space {
            return size;
        }
    }
    panic!()
}

fn list_directory_sizes(data: &str) -> BTreeSet<u32> {
    let mut dirtree = BTreeSet::<u32>::new();
    let mut dirstack = Vec::<u32>::new();

    for line in data.lines() {
        let mut line_parts = line.split_whitespace();

        match line_parts.next() {
            Some("$") => match line_parts.next() {
                Some("cd") => match line_parts.next() {
                    Some("..") => {
                        let v = dirstack.pop().unwrap();

                        if !dirstack.is_empty() {
                            let i = dirstack.len() - 1;
                            dirstack[i] += v;
                        }
                        dirtree.insert(v);
                    }
                    Some(_) => dirstack.push(0),
                    None => panic!("'cd' missing dir!"),
                },
                Some(_) => {} // don't care about ls
                None => panic!("'$' missing command!"),
            },
            Some("dir") => {} // don't care about dir in ls
            Some(i) => {
                let size = i.parse::<u32>().expect("Failed to parse into number!");
                let i = dirstack.len() - 1;
                dirstack[i] += size;
            }
            None => panic!("Missing line!"),
        }
    }

    while let Some(dir) = dirstack.pop() {
        if !dirstack.is_empty() {
            let i = dirstack.len() - 1;
            dirstack[i] += dir;
        }
        dirtree.insert(dir);
    }

    dirtree
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

        assert_eq!(sum_small_files(&input, 100_000), 95437);
    }

    #[test]
    fn check_part_1() {
        let mut input = String::new();
        File::open(INPUT)
            .expect("Failed to open!")
            .read_to_string(&mut input)
            .expect("Failed to read file!");

        println!("{:?}", sum_small_files(&input, 100_000));
    }

    #[test]
    fn check_test_solution_2() {
        let mut input = String::new();
        File::open(TEST_INPUT)
            .expect("Failed to open!")
            .read_to_string(&mut input)
            .expect("Failed to read file!");

        assert_eq!(
            find_smallest_useful_dir(&input, 70_000_000, 30_000_000),
            24933642
        );
    }

    #[test]
    fn check_part_2() {
        let mut input = String::new();
        File::open(INPUT)
            .expect("Failed to open!")
            .read_to_string(&mut input)
            .expect("Failed to read file!");

        println!(
            "{:?}",
            find_smallest_useful_dir(&input, 70_000_000, 30_000_000)
        );
    }
}

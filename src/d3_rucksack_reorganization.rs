use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn reorganize() -> u32 {
    let rucks = get_input();

    let mut prio = 0;
    for ruck in rucks {
        let split = ruck.len() / 2;
        let s1 = &ruck[..split];
        let s2 = &ruck[split..];

        let common = get_common_char(s1, s2);
        prio += score_char(common);
    }

    prio
}

pub fn reorganize_triples() -> u32 {
    let rucks = ruck_sets(get_input(), 3);

    let mut prio = 0;
    for ruck_set in rucks {
        let common = get_triple_common_char(&ruck_set[0], &ruck_set[1], &ruck_set[2]);
        prio += score_char(common);
    }

    prio
}

fn ruck_sets(rucks: Vec<String>, size: usize) -> Vec<Vec<String>> {
    let mut paired_rucks = Vec::new();
    let mut current_set = Vec::new();

    for ruck in rucks {
        current_set.push(ruck.clone());

        if current_set.len() >= size {
            paired_rucks.push(current_set.clone());
            current_set.clear();
        }
    }

    paired_rucks
}

fn get_triple_common_char(s1: &str, s2: &str, s3: &str) -> char {
    let first_common = s1.chars().filter(|c| s2.contains(*c));
    let mut trip_common = first_common.filter(|c| s3.contains(*c));

    match trip_common.next() {
        Some(c) => c,
        None => char::REPLACEMENT_CHARACTER,
    }
}

fn get_common_char(s1: &str, s2: &str) -> char {
    let mut common = s1.chars().filter(|c| s2.contains(*c));

    match common.next() {
        Some(c) => c,
        None => char::REPLACEMENT_CHARACTER,
    }
}

fn score_char(c: char) -> u32 {
    if c.is_lowercase() {
        u32::from(c) - 96
    } else {
        u32::from(c) - 64 + 26
    }
}

fn get_input() -> Vec<String> {
    let fp = "src\\d3_rucksack_reorganization.txt";
    let file = File::open(fp).expect("Error opening file.");
    let reader = BufReader::new(file);

    let mut vec_lines = Vec::<String>::new();
    for line in reader.lines() {
        vec_lines.push(line.expect("Error reading line"));
    }

    vec_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = get_input();
        println!("{:?}", input);
    }

    #[test]
    fn test_scoring() {
        assert_eq!(score_char('c'), 3);
        assert_eq!(score_char('z'), 26);
        assert_eq!(score_char('A'), 27);
        assert_eq!(score_char('Z'), 52);
    }

    #[test]
    fn test_bad_score() {
        assert_eq!(score_char(char::REPLACEMENT_CHARACTER), 65533 - 64 + 26);
    }

    #[test]
    fn test_common() {
        assert_eq!(get_common_char("abc", "ATa"), 'a');
        assert_eq!(get_common_char("abc", "def"), char::REPLACEMENT_CHARACTER);
    }

    #[test]
    fn test_common_triple() {
        assert_eq!(get_triple_common_char("abc", "ATa", "BaB"), 'a');
        assert_eq!(
            get_triple_common_char("abc", "def", "UwU"),
            char::REPLACEMENT_CHARACTER
        );
    }

    #[test]
    fn check_prio() {
        println!("{}", reorganize());
    }

    #[test]
    fn check_prio_2() {
        println!("{}", reorganize_triples());
    }
}

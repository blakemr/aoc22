use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    fn contains(self, compare: Assignment) -> bool {
        self.start <= compare.start && self.end >= compare.end
    }

    fn overlaps(self, compare: Assignment) -> bool {
        (self.start <= compare.end && self.end >= compare.end)
            || (self.start <= compare.start && self.end >= compare.start)
            || (compare.start <= self.end && compare.end >= self.end)
            || (compare.start <= self.start && compare.end >= self.start)
    }
}

pub fn count_contained() -> usize {
    let assignments = get_input();

    assignments
        .iter()
        .filter(|pair| pair[0].contains(pair[1]) || pair[1].contains(pair[0]))
        .count()
}
pub fn count_overlaps() -> usize {
    let assignments = get_input();

    assignments
        .iter()
        .filter(|pair| pair[0].overlaps(pair[1]))
        .count()
}

fn get_input() -> Vec<Vec<Assignment>> {
    let fp = "src\\d4_camp_clenup.txt";
    let file = File::open(fp).expect("Error opening file.");
    let reader = BufReader::new(file);

    let mut vec_lines = Vec::<Vec<Assignment>>::new();
    for line in reader.lines() {
        let pair = line
            .expect("Error reading line.")
            .split(',')
            .map(|p| {
                let nums: Vec<usize> = p
                    .split('-')
                    .map(|n| n.parse::<usize>().expect("Unable to parse number."))
                    .collect();

                Assignment {
                    start: nums[0],
                    end: nums[1],
                }
            })
            .collect();

        vec_lines.push(pair);
    }

    vec_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_containing() {
        println!("{}", count_contained());
    }
    #[test]
    fn test_overlaps() {
        println!("{}", count_overlaps());
    }

    #[test]
    fn check_input() {
        println!("{:?}", get_input());
    }
}

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Error},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
struct ParseCaveError;

#[derive(Debug)]
struct Cave {
    rocks: BTreeMap<usize, BTreeSet<usize>>, //column, row
    sand: BTreeSet<(usize, usize)>,
}

impl FromStr for Cave {
    type Err = ParseCaveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let endpoints: Vec<Vec<(usize, usize)>> = s
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|tuple| {
                        tuple
                            .split(',')
                            .map(|t| t.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        let mut rocks: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

        for rock_ends in endpoints.iter() {
            for eps in rock_ends.windows(2) {
                if eps[0].0 == eps[1].0 {
                    let start = eps[0].1;
                    let end = eps[1].1;

                    // Can append, but not really worth bothering for now.
                    for row in start..=end {
                        rocks
                            .entry(eps[0].0)
                            .and_modify(|r| {
                                r.insert(row);
                            })
                            .or_insert_with(|| BTreeSet::from([row]));
                    }
                } else if eps[0].1 == eps[1].1 {
                    let start = eps[0].0;
                    let end = eps[1].0;

                    let mut dir = [start, end];
                    dir.sort();
                    for column in dir[0]..=dir[1] {
                        rocks
                            .entry(column)
                            .and_modify(|r| {
                                r.insert(eps[0].1);
                            })
                            .or_insert_with(|| BTreeSet::from([eps[0].1]));
                    }
                } else {
                    panic!("Diagonal rock?");
                }
            }
        }

        Ok(Cave {
            rocks,
            sand: BTreeSet::new(),
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_column = self.rocks.keys().next().unwrap();
        let width = self.rocks.keys().last().unwrap() - self.rocks.keys().next().unwrap();
        let height = self
            .rocks
            .iter()
            .map(|(_, column)| column.iter().max())
            .max()
            .unwrap()
            .unwrap();

        let mut output = String::new();
        for row in 0..=*height {
            for column in *first_column..=(first_column + width) {
                if (column, row) == (500, 0) {
                    output.push('+');
                } else if self.sand.contains(&(column, row)) {
                    output.push('o');
                } else if self.rocks.contains_key(&column)
                    && self.rocks.get(&column).unwrap().contains(&row)
                {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl Cave {
    fn add_sand(&mut self, start: (usize, usize)) -> Option<(usize, usize)> {
        let mut sand_pos = start;

        while let Some(column) = self.rocks.get(&sand_pos.0) {
            // while there are rocks in this column,
            if let Some(ground) = column.range(sand_pos.1..).next() {
                // move the sand to the nearest ground
                if ground - 1 > sand_pos.1 {
                    sand_pos.1 = ground - 1;
                } else if self
                    // If down-left is open, move there
                    .rocks
                    .get(&(sand_pos.0 - 1))?
                    .range(sand_pos.1 + 1..)
                    .next()?
                    != &(sand_pos.1 + 1)
                {
                    sand_pos.0 -= 1;
                    sand_pos.1 += 1;
                } else if self
                    // else, if down-right is open, move there
                    .rocks
                    .get(&(sand_pos.0 + 1))?
                    .range(sand_pos.1 + 1..)
                    .next()?
                    != &(sand_pos.1 + 1)
                {
                    sand_pos.0 += 1;
                    sand_pos.1 += 1;
                } else {
                    // otherwise, rest here
                    return Some(sand_pos);
                }
            } else {
                return None;
            }
        }

        None
    }

    fn fill_with_sand(&mut self, start: (usize, usize)) -> Result<usize, Error> {
        let mut sand = 0;
        while let Some(pos) = self.add_sand(start) {
            sand += 1;

            if !self.sand.insert(pos) {
                panic!("Added sand to filled location (sand)! pos: {:?}", pos);
            }

            self.rocks.entry(pos.0).and_modify(|p| {
                if !p.insert(pos.1) {
                    panic!("Added sand to filled location (rock)! pos: {:?}", pos);
                }
            });
        }

        Ok(sand) // Fall off map; empty column
    }
}

pub fn part_1(input: &str) -> usize {
    let mut cave = input.parse::<Cave>().unwrap();
    let sand = cave.fill_with_sand((500, 0)).unwrap();
    println!("{}", cave);
    sand
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d14_regolith_resrvoir.test";
    const INPUT: &str = "src\\d14_regolith_resrvoir.input";

    #[test]
    fn check_parse() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        dbg!(input.parse::<Cave>().unwrap());
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_1(&input), 24)
    }

    #[test]
    fn run_part_1() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{}", part_1(&input))
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

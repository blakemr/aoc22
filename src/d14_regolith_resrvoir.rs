use std::{
    collections::{BTreeMap, BTreeSet},
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

impl Cave {
    fn fill_with_sand(&mut self, start: (usize, usize)) -> usize {
        let mut sand = 0;
        let mut sand_pos = start;
        while let Some(column) = self.rocks.get(&sand_pos.0) {
            if let Some(next_rock) = column.range(sand_pos.1..).next() {
                if next_rock - sand_pos.1 > 1 {
                    sand_pos.1 = next_rock - 1;
                }

                let left_column = self.rocks.get(&(sand_pos.0 - 1));
                let right_column = self.rocks.get(&(sand_pos.0 + 1));

                // check left
                if left_column.is_none()
                    || left_column
                        .unwrap()
                        .range((sand_pos.1 + 1)..)
                        .next()
                        .is_none()
                {
                    // Left void
                    return sand;
                } else if left_column
                    .unwrap()
                    .range((sand_pos.1 + 1)..)
                    .next()
                    .unwrap()
                    == &(sand_pos.1 + 1)
                {
                    // Check Right
                    if right_column.is_none()
                        || right_column
                            .unwrap()
                            .range((sand_pos.1 + 1)..)
                            .next()
                            .is_none()
                    {
                        // Right void
                        return sand;
                    } else if right_column
                        .unwrap()
                        .range((sand_pos.1 + 1)..)
                        .next()
                        .unwrap()
                        == &(sand_pos.1 + 1)
                    {
                        // Set sand
                        self.rocks.entry(sand_pos.0).and_modify(|rows| {
                            rows.insert(sand_pos.1);
                        });
                        self.sand.insert(sand_pos);
                        sand += 1;
                        sand_pos = start;
                    } else {
                        // Can go right
                        sand_pos.0 += 1;
                    }
                } else {
                    // Can go left
                    sand_pos.0 -= 1;
                }
            } else {
                // there are rocks above, but none below
                break;
            }
        }

        sand // Fall off map; empty column
    }
}

pub fn part_1(input: &str) -> usize {
    let mut cave = input.parse::<Cave>().unwrap();
    let sand = cave.fill_with_sand((500, 0));
    dbg!(cave.sand);
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

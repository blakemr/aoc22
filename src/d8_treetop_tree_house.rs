use std::collections::BTreeSet;

pub fn count_visible_trees(input: &str) -> u32 {
    let trees = parse_trees(input);
    let mut visible_trees: BTreeSet<(usize, usize)> = BTreeSet::new();

    //horizontal
    for (y, row) in trees.iter().enumerate() {
        let sighted = row.iter().enumerate().filter(|(i, v)| {
            row[..*i].iter().all(|e| e < v) || row[*i + 1..].iter().all(|e| e < v)
        });

        for (index, _value) in sighted {
            visible_trees.insert((index, y));
        }
    }

    //vertical
    for x in 0..trees[0].len() {
        let column: Vec<u32> = trees.iter().map(|v| v[x]).collect();

        let sighted = column.iter().enumerate().filter(|(i, v)| {
            column[..*i].iter().all(|e| e < v) || column[*i + 1..].iter().all(|e| e < v)
        });

        for (index, _value) in sighted {
            visible_trees.insert((x, index));
        }
    }

    visible_trees.len() as u32
}

pub fn find_max_scenic_score(input: &str) -> u32 {
    let trees = parse_trees(input);
    let mut max_score: u32 = 0;

    let width: usize = trees[0].len();
    for (i, height) in trees.iter().flatten().enumerate() {
        let x = i % width;
        let y = i / width;

        let row = trees[y].clone();
        let column: Vec<u32> = trees.iter().map(|v| v[x]).collect();

        let (north, south) = column.split_at(y);
        let mut north = north.to_vec();
        north.reverse();
        let south = south.to_vec()[1..].to_vec();
        let (east, west) = row.split_at(x);
        let mut east = east.to_vec();
        east.reverse();
        let west = west.to_vec()[1..].to_vec();

        let mut score: usize = 1;
        for direction in [&north, &south, &east, &west] {
            let subscore = direction
                .split_inclusive(|v| v >= height)
                .next()
                .unwrap_or_default()
                .len();

            score *= subscore;
        }

        max_score = max_score.max(score as u32);
    }

    max_score
}

fn parse_trees(input: &str) -> Vec<Vec<u32>> {
    let mut output = Vec::new();
    for line in input.lines() {
        output.push(Vec::from_iter(
            line.chars().map(|n| n.to_digit(10).unwrap()),
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_tree_parse() {
        let input = fs::read_to_string("src\\d8_treetop_tree_house.test").unwrap();

        dbg!(parse_trees(&input));
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("src\\d8_treetop_tree_house.test").unwrap();

        assert_eq!(count_visible_trees(&input), 21)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("src\\d8_treetop_tree_house.txt").unwrap();

        println!("{:?}", count_visible_trees(&input))
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("src\\d8_treetop_tree_house.test").unwrap();

        assert_eq!(find_max_scenic_score(&input), 8)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("src\\d8_treetop_tree_house.txt").unwrap();

        println!("{:?}", find_max_scenic_score(&input))
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn sum_score() -> u32 {
    let rounds = txt_input();
    let mut score: u32 = 0;
    for round in rounds {
        score += round_score(round);
    }

    score
}

pub fn sum_round_2() -> u32 {
    let rounds = txt_input();
    let mut score: u32 = 0;
    for round in rounds {
        score += round_2_score(round);
    }

    score
}

#[allow(clippy::identity_op)]
fn round_score(round: String) -> u32 {
    match &*round {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

#[allow(clippy::identity_op)]
fn round_2_score(round: String) -> u32 {
    match &*round {
        "A X" => 0 + 3, // Lose v rock: scissors
        "A Y" => 3 + 1, // tie v rock: rock
        "A Z" => 6 + 2, // win v rock: paper
        "B X" => 0 + 1, // lose v paper: rock
        "B Y" => 3 + 2, // tie: paper
        "B Z" => 6 + 3, // win v paper: scissors
        "C X" => 0 + 2, // lose v scissors: paper
        "C Y" => 3 + 3, // tie: scissors
        "C Z" => 6 + 1, // win v scissors: rock
        _ => 0,
    }
}

fn txt_input() -> Vec<String> {
    let fp = "src\\rock_paper_scissors.txt";
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
    use crate::rock_paper_scissors::sum_round_2;

    use super::{round_score, sum_score, txt_input};

    #[test]
    fn check_input() {
        let inp = txt_input();
        for line in inp {
            println!("{}", line);
        }
    }

    #[test]
    fn test_scoring() {
        assert_eq!(round_score("C X".to_string()), 6 + 1);
        assert_eq!(round_score("C Y".to_string()), 2);
        assert_eq!(round_score("C Z".to_string()), 3 + 3);
    }

    #[test]
    fn result_1() {
        println!("{}", sum_score());
    }

    #[test]
    fn result_2() {
        println!("{}", sum_round_2());
    }
}

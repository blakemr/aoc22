use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
mod tests {}

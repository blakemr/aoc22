use std::collections::HashSet;

#[allow(dead_code)]
const INPUT_DATA: &str = "src\\d6_tuning_trouble.txt";
#[allow(dead_code)]
const INPUT_DATA_TEST: &str = "src\\d6_tuning_trouble.test";

pub fn find_marker_index(data: &str, marker_size: usize) -> usize {
    for i in 0..data.len() {
        let sample: &HashSet<char> = &data[i..i + marker_size].chars().collect();
        if sample.len() == marker_size {
            return i + marker_size;
        }
    }

    panic!("Marker not found!");
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn test_find_marker() {
        assert_eq!(find_marker_index("data", 2), 2);
        assert_eq!(find_marker_index("aata", 2), 3);
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_data() {
        let mut test_data = String::new();
        File::open(INPUT_DATA_TEST)
            .expect("File can't open.")
            .read_to_string(&mut test_data)
            .expect("Can't read to string");

        assert_eq!(find_marker_index(&test_data, 4), 7);
    }

    #[test]
    fn find_packet_marker() {
        let mut data = String::new();
        File::open(INPUT_DATA)
            .expect("File can't open.")
            .read_to_string(&mut data)
            .expect("Can't read to string");

        println!("{:?}", find_marker_index(&data, 4))
    }

    #[test]
    fn test_find_message() {
        assert_eq!(find_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }

    #[test]
    fn find_message_marker() {
        let mut data = String::new();
        File::open(INPUT_DATA)
            .expect("File can't open.")
            .read_to_string(&mut data)
            .expect("Can't read to string");

        println!("{:?}", find_marker_index(&data, 14))
    }
}

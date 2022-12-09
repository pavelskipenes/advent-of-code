use std::collections::HashSet;

#[must_use]
pub fn find_start_of_packet(input: &'static str) -> usize {
    let (_, result) = find_distinct_chars(input, 4);
    result
}

#[must_use]
pub fn find_start_of_message(input: &'static str) -> usize {
    let (_, result) = find_distinct_chars(input, 14);
    result
}

fn find_distinct_chars(input: &'static str, length: usize) -> (&'static str, usize) {
    let mut offset = 0;
    loop {
        let mut set: HashSet<char> = HashSet::new();
        input[offset..offset + length].chars().for_each(|ch| {
            set.insert(ch);
        });
        if set.len() == length {
            break;
        }
        offset += 1;
    }
    (&input[offset..offset + length], offset + length)
}

#[cfg(test)]
mod tests {
    use crate::day_6::{find_start_of_message, find_start_of_packet};

    const INPUT: &str = include_str!("../puzzle_input/day_6.txt");
    const EXAMPLE_INPUT: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    const ANSWER: [usize; 2] = [1702, 3559];
    const EXAMPLE_ANSWER: [[usize; 2]; 5] = [[7, 19], [5, 23], [6, 23], [10, 29], [11, 26]];

    #[test]
    fn example() {
        for (input, expected) in EXAMPLE_INPUT.iter().zip(EXAMPLE_ANSWER.iter()) {
            assert_eq!(find_start_of_packet(input), expected[0]);
            assert_eq!(find_start_of_message(input), expected[1]);
        }
    }

    #[test]
    fn problem() {
        assert_eq!(find_start_of_packet(INPUT), ANSWER[0]);
        assert_eq!(find_start_of_message(INPUT), ANSWER[1]);
    }
}

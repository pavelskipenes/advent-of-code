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
    use super::*;

    // (input, expected_problem_1, expected_problem2)
    const EXPECTED: (&str, usize, usize) = (include_str!("../puzzle_input/day_6.txt"), 1702, 3559);
    const EXAMPLE_EXPECTED: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn example() {
        for (input, expected1, expected2) in &EXAMPLE_EXPECTED {
            assert_eq!(find_start_of_packet(input), *expected1);
            assert_eq!(find_start_of_message(input), *expected2);
        }
    }

    #[test]
    fn problem() {
        let (input, expected1, expected2) = EXPECTED;
        assert_eq!(find_start_of_packet(input), expected1);
        assert_eq!(find_start_of_message(input), expected2);
    }
}

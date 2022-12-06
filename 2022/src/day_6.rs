use std::collections::HashSet;

/// returns start of packet signature and first offset on where data is
fn find_start_of_packet(input: &'static str) -> (&'static str, usize) {
    find_distinct_chars(input, 4)
}

fn find_start_of_message(input: &'static str) -> (&'static str, usize) {
    find_distinct_chars(input, 14)
}

fn find_distinct_chars(input: &'static str, count: usize) -> (&'static str, usize) {
    let mut offset = 0;
    loop {
        let view = &input[offset..offset + count];
        let mut set: HashSet<char> = HashSet::new();
        for character in view.chars() {
            set.insert(character);
        }
        if set.len() == count {
            break;
        }
        offset += 1;
    }
    (&input[offset..offset + count], offset + count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn get_input() -> &'static str {
        include_str!("../puzzle_input/day_6.txt")
    }

    const fn get_example_input_1() -> &'static str {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    }
    const fn get_example_input_2() -> &'static str {
        "bvwbjplbgvbhsrlpgdmjqwftvncz"
    }
    const fn get_example_input_3() -> &'static str {
        "nppdvjthqldpwncqszvftbrmjlhg"
    }
    const fn get_example_input_4() -> &'static str {
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
    }
    const fn get_example_input_5() -> &'static str {
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
    }

    #[test]
    fn test_example_1() {
        const INPUT: &str = get_example_input_1();
        const ANSWER1: usize = 7;
        const MARKER1: &str = "jpqm";
        assert_eq!(find_start_of_packet(INPUT), (MARKER1, ANSWER1));
    }
    #[test]
    fn test_example_2() {
        const INPUT: &str = get_example_input_2();
        const ANSWER2: usize = 5;
        const MARKER2: &str = "vwbj";
        assert_eq!(find_start_of_packet(INPUT), (MARKER2, ANSWER2));
    }
    #[test]
    fn test_example_3() {
        const INPUT: &str = get_example_input_3();
        const ANSWER3: usize = 6;
        const MARKER3: &str = "pdvj";
        assert_eq!(find_start_of_packet(INPUT), (MARKER3, ANSWER3));
    }
    #[test]
    fn test_example_4() {
        const INPUT: &str = get_example_input_4();
        const ANSWER4: usize = 10;
        const MARKER4: &str = "rfnt";
        assert_eq!(find_start_of_packet(INPUT), (MARKER4, ANSWER4));
    }
    #[test]
    fn test_example_5() {
        const INPUT: &str = get_example_input_5();
        const ANSWER5: usize = 11;
        const MARKER5: &str = "zqfr";
        assert_eq!(find_start_of_packet(INPUT), (MARKER5, ANSWER5));
    }

    #[test]
    fn test_example_6() {
        const INPUT: &str = get_example_input_1();
        const ANSWER1: usize = 19;
        const MARKER1: &str = "qmgbljsphdztnv";
        assert_eq!(find_start_of_message(INPUT), (MARKER1, ANSWER1));
    }
    #[test]
    fn test_example_7() {
        const INPUT: &str = get_example_input_2();
        const ANSWER2: usize = 23;
        const MARKER2: &str = "vbhsrlpgdmjqwf";
        assert_eq!(find_start_of_message(INPUT), (MARKER2, ANSWER2));
    }
    #[test]
    fn test_example_8() {
        const INPUT: &str = get_example_input_3();
        const ANSWER3: usize = 23;
        const MARKER3: &str = "ldpwncqszvftbr";
        assert_eq!(find_start_of_message(INPUT), (MARKER3, ANSWER3));
    }
    #[test]
    fn test_example_9() {
        const INPUT: &str = get_example_input_4();
        const ANSWER4: usize = 29;
        const MARKER4: &str = "wmzdfjlvtqnbhc";
        assert_eq!(find_start_of_message(INPUT), (MARKER4, ANSWER4));
    }
    #[test]
    fn test_example_10() {
        const INPUT: &str = get_example_input_5();
        const ANSWER5: usize = 26;
        const MARKER5: &str = "jwzlrfnpqdbhtm";
        assert_eq!(find_start_of_message(INPUT), (MARKER5, ANSWER5));
    }

    #[test]
    fn test_problem_1() {
        let input = get_input();
        const ANSWER: &str = "rpbc";
        const OFFSET: usize = 1702;
        assert_eq!(find_start_of_packet(input), (ANSWER, OFFSET));
    }
    #[test]
    fn test_problem_2() {
        let input = get_input();

        const ANSWER: &str = "lnvbtqgjsdzmhc";
        const OFFSET: usize = 3559;
        assert_eq!(find_start_of_message(input), (ANSWER, OFFSET));
    }
}

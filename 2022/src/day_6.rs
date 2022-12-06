use std::collections::HashSet;

fn run_problem_1(input: &'static str) -> (&'static str, usize) {
    let mut offset = 0;
    loop {
        let view = &input[offset..offset + 4];
        let mut set: HashSet<char> = HashSet::new();
        for character in view.chars() {
            set.insert(character);
        }
        dbg!(&input, &set, &input[offset..], &offset);
        if set.len() == 4 {
            set.clear();
            break;
        }
        offset += 1;
    }
    (&input[offset..offset + 4], offset + 4)
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
        assert_eq!(run_problem_1(INPUT), (MARKER1, ANSWER1));
    }
    #[test]
    fn test_example_2() {
        const INPUT: &str = get_example_input_2();
        const ANSWER2: usize = 5;
        const MARKER2: &str = "vwbj";
        assert_eq!(run_problem_1(INPUT), (MARKER2, ANSWER2));
    }
    #[test]
    fn test_example_3() {
        const INPUT: &str = get_example_input_3();
        const ANSWER3: usize = 6;
        const MARKER3: &str = "pdvj";
        assert_eq!(run_problem_1(INPUT), (MARKER3, ANSWER3));
    }
    #[test]
    fn test_example_4() {
        const INPUT: &str = get_example_input_4();
        const ANSWER4: usize = 10;
        const MARKER4: &str = "rfnt";
        assert_eq!(run_problem_1(INPUT), (MARKER4, ANSWER4));
    }
    #[test]
    fn test_example_5() {
        const INPUT: &str = get_example_input_5();
        const ANSWER5: usize = 11;
        const MARKER5: &str = "zqfr";
        assert_eq!(run_problem_1(INPUT), (MARKER5, ANSWER5));
    }

    #[test]
    fn test_problem_1() {
        let input = get_input();
        const ANSWER: &str = "rpbc";
        const OFFSET: usize = 1702;
        assert_eq!(run_problem_1(input), (ANSWER, OFFSET));
    }
    #[test]
    #[ignore]
    fn test_problem_2() {
        let input = get_input();

        assert!(false);
        // const ANSWER: u32 = u32::MAX;
        // assert_eq!(process_2(input), ANSWER2);
    }
}

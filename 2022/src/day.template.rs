#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../puzzle_input/day.txt");
    const EXAMPLE_INPUT: [&str; 5] = ["", "", "", "", ""];
    // const EXAMPLE_INPUT: &str = r"";

    const ANSWER: [usize; 2] = [0, 0];
    const EXAMPLE_ANSWER: [[usize; 2]; 5] = [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0]];

    #[test]
    #[ignore]
    fn example() {
        for (input, expected) in EXAMPLE_INPUT.iter().zip(EXAMPLE_ANSWER.iter()) {
            assert_eq!(todo(input), expected[0]);
            // assert_eq!(todo(input), expected[1]);
        }
    }

    #[test]
    #[ignore]
    fn problem() {
        assert_eq!(todo(INPUT), ANSWER[0]);
        // assert_eq!(todo(INPUT), ANSWER[1]);
    }
}

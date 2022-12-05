const fn create_stacks() -> [Vec<char>; 9] {
    [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ]
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, not_line_ending, space0},
        sequence::delimited,
        IResult,
    };

    /// consume one cell (three spaces) from input
    pub fn crate_cell(input: &str) -> IResult<&str, Option<char>> {
        let fn_crate_letter = delimited(char('['), alpha1, char(']'));
        let fn_empty = tag("   ");
        let mut fn_crate_cell = alt((fn_crate_letter, fn_empty));

        let (input, result) = fn_crate_cell(input)?;

        let result = match result {
            "   " => None,
            value => {
                let character = value.chars().next().unwrap();
                Some(character)
            }
        };

        Ok((input, result))
    }

    pub fn crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
        let (mut input, mut line) = not_line_ending(input)?;
        let mut output = vec![];

        while !line.is_empty() {
            // parse one cell
            let (line2, optional_character) = match crate_cell(line) {
                Ok((input, optional_character)) => (input, optional_character),
                Err(why) => panic!("{}", why),
            };
            line = line2;
            output.push(optional_character);
            // between each cell there is a space but not on the end
            (input, _) = space0(input)?;
        }

        Ok((input, output))
    }

    #[cfg(test)]
    mod tests {
        use crate::day_5::parser::{crate_cell, crate_line};

        #[test]
        fn test_parse_one_crate_cell() {
            let input = "[Z]";
            let expected = Some('Z');

            let output = match crate_cell(input) {
                Ok((_remaining_input, optional_character)) => optional_character,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);

            let input = "   ";
            let expected = None;

            let output = match crate_cell(input) {
                Ok((_remaining_input, optional_character)) => optional_character,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }

        #[test]
        fn test_parse_once_crate_line() {
            // regular input
            let input = "[A] [B] [C]";
            let expected = vec![Some('A'), Some('B'), Some('C')];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }
        #[test]
        fn test_parse_once_crate_line_space_end() {
            // space at the end
            let input = "[F] [U] [C] [K]    ";
            let expected = vec![Some('F'), Some('U'), Some('C'), Some('K'), None];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }
        #[test]
        fn test_parse_once_crate_line_space_front() {
            // space at front
            let input = "    [F] [U] [C] [K]";
            let expected = vec![None, Some('F'), Some('U'), Some('C'), Some('K')];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }
        #[test]
        fn test_parse_once_crate_line_space_front_and_back() {
            // space at front
            let input = "    [F] [U] [C] [K]    ";
            let expected = vec![None, Some('F'), Some('U'), Some('C'), Some('K'), None];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    fn get_input() -> &'static str {
        include_str!("../puzzle_input/day_5.txt")
    }

    const fn get_example_input() -> &'static str {
        r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn test_parse_stack() {
        // const INPUT: &str = get_example_input();
        // let expected: [Vec<char>; 9] = [
        //     vec!['Z', 'N'],
        //     vec!['M', 'C', 'D'],
        //     vec!['P'],
        //     vec![],
        //     vec![],
        //     vec![],
        //     vec![],
        //     vec![],
        //     vec![],
        // ];
        // let output = super::create_stacks();
        // super::parse_stack(&mut output, INPUT);
        // assert_eq!(expected, output);
    }

    #[test]
    #[ignore]
    fn test_example_1() {
        // const INPUT: &str = super::tests::get_example_input();
        // const ANSWER: &str = "CMZ";
        // assert_eq!(super::run(INPUT), ANSWER);
    }

    #[test]
    #[ignore]
    fn test_problem_1() {
        // let input = get_input();
        // const ANSWER1: u32 = u32::MAX;
        // assert_eq!(process_1(input), ANSWER1);
    }

    #[test]
    #[ignore]
    fn test_example_2() {
        // const INPUT: &str = super::tests::get_example_input();
        // const ANSWER1: u32 = u32::MAX;
        // assert_eq!(process_1(INPUT), ANSWER1);
    }

    #[test]
    #[ignore]
    fn test_problem_2() {
        // let input = get_input();
        // const ANSWER: u32 = u32::MAX;
        // assert_eq!(process_2(input), ANSWER2);
    }
}

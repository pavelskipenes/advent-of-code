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

pub struct Instruction {
    repetitions: u32,
    src: u8,
    dest: u8,
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::{
            complete::{alpha1, anychar, char, digit1, not_line_ending},
            streaming::space1,
        },
        sequence::{delimited, tuple},
        IResult,
    };

    use super::Instruction;

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

    /// Consumes one line from `input` and returns the remaining of input without parsed line
    /// and a vec of chars
    pub fn crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
        let (input, mut line) = not_line_ending(input)?;
        let mut output = vec![];

        while !line.is_empty() {
            // parse one cell
            let optional_character;
            (line, optional_character) = match crate_cell(line) {
                Ok((input, optional_character)) => (input, optional_character),
                Err(why) => panic!("{}", why),
            };
            output.push(optional_character);

            // consume one space if present
            if !line.is_empty() {
                let result = anychar(line)?;
                let (trimmed_line, character) = result;
                line = if character == ' ' { trimmed_line } else { line };
            }
        }

        Ok((input, output))
    }

    pub fn instruction(input: &str) -> IResult<&str, Instruction> {
        fn move_string_parser(input: &str) -> IResult<&str, &str> {
            tag("move")(input)
        }
        let (input, line) = not_line_ending(input)?;

        // let fn_move_str = tag("move");
        let fn_from_str = tag("from");
        let fn_to_str = tag("to");

        // example "move 2 from 2 to 8";
        let instruction = match tuple((
            move_string_parser,
            space1,
            digit1,
            space1,
            fn_from_str,
            space1,
            digit1,
            space1,
            fn_to_str,
            space1,
            digit1,
        ))(line)
        {
            Ok((_remaining_line, (_, _, repetitions, _, _, _, src, _, _, _, dest))) => {
                // Yolo, what can go wrong 😅?
                let repetitions = repetitions.parse::<u32>().unwrap();
                let src = src.parse::<u8>().unwrap();
                let dest = dest.parse::<u8>().unwrap();

                Instruction {
                    repetitions,
                    src,
                    dest,
                }
            }
            Err(why) => panic!("{}", why),
        };

        Ok((input, instruction))
    }

    #[cfg(test)]
    mod tests {
        use crate::day_5::parser::{crate_cell, crate_line, instruction};

        #[test]
        fn parse_one_crate_cell() {
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
        fn parse_one_crate_line_one_cell() {
            let input = "[A]";
            let expected = vec![Some('A')];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }

        #[test]
        fn parse_one_crate_line_two_cells() {
            let input = "[A] [B]";
            let expected = vec![Some('A'), Some('B')];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }

        #[test]
        fn parse_one_crate_line_space_end() {
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
        fn parse_one_crate_line_space_front() {
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
        fn parse_one_crate_line_space_front_and_back() {
            // space at front
            let input = "    [F] [U] [C] [K]    ";
            let expected = vec![None, Some('F'), Some('U'), Some('C'), Some('K'), None];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }

        #[test]
        fn parse_one_crate_line_space_front_back_and_in_between() {
            // space at front
            let input = "    [F] [U]     [C] [K]    ";
            let expected = vec![None, Some('F'), Some('U'), None, Some('C'), Some('K'), None];

            let output = match crate_line(input) {
                Ok((_remaining_input, vec_optional_chars)) => vec_optional_chars,
                Err(why) => panic!("{}", why),
            };
            assert_eq!(output, expected);
        }

        #[test]
        fn parse_instruction() {
            let input = "move 1 from 2 to 3";

            let (remaining_input, output_instruction) = match instruction(input) {
                Ok((remaining_input, generated_instruction)) => {
                    (remaining_input, generated_instruction)
                }
                Err(why) => panic!("{}", why),
            };

            assert_eq!(output_instruction.repetitions, 1);
            assert_eq!(output_instruction.src, 2);
            assert_eq!(output_instruction.dest, 3);
            assert_eq!(remaining_input, "");
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

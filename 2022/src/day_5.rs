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

fn transpose_and_reverse(matrix: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut transposed: Vec<Vec<char>> = vec![];
    for col in 0..matrix[0].len() {
        let mut transposed_row: Vec<char> = vec![];
        for row in 0..matrix.len() {
            if let Some(c) = matrix[row][col] {
                transposed_row.push(c);
            }
        }
        transposed.push(transposed_row);
    }
    transposed.iter_mut().for_each(|row| row.reverse());
    transposed
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    repetitions: u32,
    src: u8,
    dest: u8,
}

impl Instruction {
    fn execute_one_at_a_time(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.repetitions {
            let tmp = stacks[self.src as usize - 1].pop().unwrap();
            stacks[self.dest as usize - 1].push(tmp);
        }
    }
    fn execute_many_at_a_time(&self, stacks: &mut Vec<Vec<char>>) {
        let stack_size = stacks[self.src as usize - 1].len();
        let tmp = &stacks[self.src as usize - 1].split_off(stack_size - self.repetitions as usize);

        for character in tmp {
            stacks[self.dest as usize - 1].push(*character);
        }
    }

    fn get_top_stack_as_string(stacks: &Vec<Vec<char>>) -> String {
        let mut result = "".to_string();
        for stack in stacks {
            if let Some(character) = stack.last() {
                result.push(*character);
            };
        }
        result
    }
}

pub fn run_problem_1(input: &str) -> String {
    let (remaining_input, mut stacks) = parser::crate_init_rows(input);
    let (remaining_input, _trash) = parser::throw_away_trash(remaining_input).unwrap();
    let instructions = parser::instructions(remaining_input);

    for instruction in instructions {
        instruction.execute_one_at_a_time(&mut stacks);
    }
    Instruction::get_top_stack_as_string(&stacks)
}

pub fn run_problem_2(input: &str) -> String {
    let (remaining_input, mut stacks) = parser::crate_init_rows(input);
    let (remaining_input, _trash) = parser::throw_away_trash(remaining_input).unwrap();
    let instructions = parser::instructions(remaining_input);

    for instruction in instructions {
        instruction.execute_many_at_a_time(&mut stacks);
    }
    Instruction::get_top_stack_as_string(&stacks)
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{is_not, tag},
        character::{
            complete::{alpha1, anychar, char, digit1, line_ending, newline, not_line_ending},
            streaming::space1,
        },
        combinator::opt,
        sequence::{delimited, tuple},
        IResult,
    };

    use crate::day_5::transpose_and_reverse;

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
            if line == " " {
                break;
            }

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

    pub fn crate_init_rows(input: &str) -> (&str, Vec<Vec<char>>) {
        fn until_wrapper<'a>(input: &'a str, characters: &'a str) -> IResult<&'a str, &'a str> {
            is_not(characters)(input)
        }

        let (trash_and_instructions, creates_section) = until_wrapper(input, "1").unwrap();
        // strip off number line
        let mut matrix: Vec<Vec<Option<char>>> = vec![];
        for line in creates_section.lines() {
            let (_, row) = crate_line(line).unwrap();
            if !row.is_empty() {
                matrix.push(row);
            }
        }
        let transposed = transpose_and_reverse(matrix);
        // reverse order maybe?
        (trash_and_instructions, transposed)
    }

    pub fn instruction(input: &str) -> IResult<&str, Instruction> {
        fn move_string_parser(input: &str) -> IResult<&str, &str> {
            tag("move")(input)
        }

        // will fail if there is a line ending tho
        //let (input, line) = not_line_ending(input)?;

        // todo: add spaces to tags below and remove space wild cards from instruction var
        let fn_from_str = tag("from");
        let fn_to_str = tag("to");

        // example "move 2 from 2 to 8";
        let (remaining, instruction) = match tuple((
            opt(newline),
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
            opt(newline),
        ))(input)
        {
            Ok((remaining, (_, _, _, repetitions, _, _, _, src, _, _, _, dest, _))) => {
                // Yolo, what can go wrong 😅?

                let repetitions = repetitions.parse::<u32>().unwrap();
                let src = src.parse::<u8>().unwrap();
                let dest = dest.parse::<u8>().unwrap();

                (
                    remaining,
                    Instruction {
                        repetitions,
                        src,
                        dest,
                    },
                )
            }
            Err(why) => panic!("{}", why),
        };

        Ok((remaining, instruction))
    }

    pub fn throw_away_trash(input: &str) -> IResult<&str, &str> {
        fn get_crate_ids(input: &str) -> IResult<&str, &str> {
            not_line_ending(input)
        }
        fn match_line_ending(input: &str) -> IResult<&str, &str> {
            line_ending(input)
        }

        let (remainder, _) = get_crate_ids(input)?;
        let (remainder, _) = line_ending(remainder)?;
        let (remainder, matches) = line_ending(remainder)?;

        Ok((remainder, matches))
    }

    pub fn instructions(input: &str) -> Vec<Instruction> {
        let mut instructions = vec![];

        let mut input = input;
        while !input.is_empty() {
            let (remaining_input, tmp_instruction) = instruction(input).unwrap();
            instructions.push(tmp_instruction);
            input = remaining_input;
        }
        instructions
    }

    #[cfg(test)]
    mod tests {
        use super::{crate_init_rows, throw_away_trash};
        use crate::day_5::{
            parser::{crate_cell, crate_line, instruction},
            transpose_and_reverse, Instruction,
        };

        #[test]
        fn parse_one_crate_cell() {
            let input = "[Z]";
            let expected = Some('Z');

            let (_remaning_string, output) = crate_cell(input).unwrap();
            assert_eq!(output, expected);

            let input = "   ";
            let expected = None;

            let (_, output) = crate_cell(input).unwrap();
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

            let (remaining_input, output_instruction) = instruction(input).unwrap();

            assert_eq!(output_instruction.repetitions, 1);
            assert_eq!(output_instruction.src, 2);
            assert_eq!(output_instruction.dest, 3);
            assert_eq!(remaining_input, "");
        }

        #[test]
        fn test_transpose_and_reverse() {
            let original_matrix = vec![
                vec![None, Some('D'), None],
                vec![Some('N'), Some('C'), None],
                vec![Some('Z'), Some('M'), Some('P')],
            ];

            let expected_matrix = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

            assert_eq!(expected_matrix, transpose_and_reverse(original_matrix));
        }

        #[test]
        fn parse_crates() {
            let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
            let (_remainder, matrix) = crate_init_rows(input);
            let expected_matrix = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
            assert_eq!(expected_matrix, matrix);
        }

        #[test]
        fn test_instruction_parser() {
            let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

            let (remainder_and_trash, _matrix) = crate_init_rows(input);
            let (mut remainder, _trash) = throw_away_trash(remainder_and_trash).unwrap();
            let mut instructions = vec![];

            while !remainder.is_empty() {
                let (tmp_remainder, tmp_instruction) = instruction(remainder).unwrap();
                instructions.push(tmp_instruction);
                remainder = tmp_remainder;
            }

            let expected_instructions = vec![
                Instruction {
                    repetitions: 1,
                    src: 2,
                    dest: 1,
                },
                Instruction {
                    repetitions: 3,
                    src: 1,
                    dest: 3,
                },
                Instruction {
                    repetitions: 2,
                    src: 2,
                    dest: 1,
                },
                Instruction {
                    repetitions: 1,
                    src: 1,
                    dest: 2,
                },
            ];

            assert_eq!(expected_instructions, instructions);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::day_5::{run_problem_1, run_problem_2};

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
        const INPUT: &str = get_example_input();
    }

    #[test]
    fn test_example_1() {
        const INPUT: &str = super::tests::get_example_input();
        const ANSWER: &str = "CMZ";
        assert_eq!(super::run_problem_1(INPUT), ANSWER.to_string());
    }

    #[test]
    fn test_problem_1() {
        let input = get_input();
        const ANSWER1: &str = "ZRLJGSCTR";
        assert_eq!(run_problem_1(input), ANSWER1.to_string());
    }

    #[test]
    fn test_example_2() {
        const INPUT: &str = super::tests::get_example_input();
        const ANSWER1: &str = "MCD";
        assert_eq!(run_problem_2(INPUT), ANSWER1.to_string());
    }

    #[test]
    fn test_problem_2() {
        let input = get_input();
        const ANSWER: &str = "PRTTGRFPB";
        assert_eq!(run_problem_2(input), ANSWER.to_string());
    }
}

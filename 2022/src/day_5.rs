#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    repetitions: u32,
    src: u8,
    dest: u8,
}

impl Instruction {
    fn execute_one_at_a_time(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.repetitions {
            let block = stacks[self.src as usize - 1].pop().unwrap();
            stacks[self.dest as usize - 1].push(block);
        }
    }
    fn execute_many_at_a_time(&self, stacks: &mut [Vec<char>]) {
        let remaining_size = stacks[self.src as usize - 1].len() - self.repetitions as usize;
        let blocks = &stacks[self.src as usize - 1].split_off(remaining_size);

        for block in blocks {
            stacks[self.dest as usize - 1].push(*block);
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

pub enum Crane {
    CrateMover9000,
    CrateMover9001,
}

/// # Panics
/// Can panic yes
#[must_use]
pub fn simulate_crane(input: &str, crane: &Crane) -> String {
    let (remaining_input, mut stacks) = parser::parse_crate_setup(input);
    let (remaining_input, _trash) = parser::parse_trash(remaining_input).unwrap();
    let instructions = parser::parse_instructions(remaining_input);

    match crane {
        Crane::CrateMover9000 => {
            for instruction in instructions {
                instruction.execute_one_at_a_time(&mut stacks);
            }
        }
        Crane::CrateMover9001 => {
            for instruction in instructions {
                instruction.execute_many_at_a_time(&mut stacks);
            }
        }
    }

    Instruction::get_top_stack_as_string(&stacks)
}

fn transpose_and_reverse(matrix: &[Vec<Option<char>>]) -> Vec<Vec<char>> {
    let mut transposed: Vec<Vec<char>> = vec![];
    for col in 0..matrix[0].len() {
        let mut transposed_row: Vec<char> = vec![];
        (0..matrix.len()).for_each(|row| {
            if let Some(c) = matrix[row][col] {
                transposed_row.push(c);
            }
        });
        transposed.push(transposed_row);
    }
    transposed.iter_mut().for_each(|row| row.reverse());
    transposed
}

mod parser {
    use super::Instruction;
    use crate::day_5::transpose_and_reverse;
    use nom::{
        branch::alt,
        bytes::complete::{is_not, tag},
        character::complete::{
            alpha1, anychar, char, digit1, line_ending, newline, not_line_ending,
        },
        combinator::opt,
        error::Error,
        sequence::{delimited, tuple},
        IResult,
    };

    /// consume one cell (three spaces) from input
    fn crate_cell(input: &str) -> IResult<&str, Option<char>> {
        let (remainder, consumed) =
            alt((delimited(char('['), alpha1, char(']')), tag("   ")))(input)?;

        let cell = match consumed {
            "   " => None,
            value => {
                let character = value.chars().next().unwrap();
                Some(character)
            }
        };

        Ok((remainder, cell))
    }

    /// Consumes one line from `input` and returns the remaining of input without parsed line
    /// and a vec of chars
    fn crate_line(input: &str) -> IResult<&str, Vec<Option<char>>, Error<&str>> {
        let mut skewed_stack = vec![];

        // get everything before a line ending
        let (remainder, mut line) = not_line_ending(input)?;

        while !line.is_empty() {
            if line == " " {
                break;
            }

            // parse one cell
            let cell;
            (line, cell) = crate_cell(line)?;
            skewed_stack.push(cell);

            // consume one space if present
            if !line.is_empty() {
                let result = anychar(line)?;
                let (trimmed_line, character) = result;
                line = if character == ' ' { trimmed_line } else { line };
            }
        }

        Ok((remainder, skewed_stack))
    }

    fn instruction(input: &str) -> IResult<&str, Instruction, Error<&str>> {
        // example "move 2 from 2 to 8";

        let (remaining, consumed) = tuple((
            opt(newline),
            tag("move "),
            digit1, // repetitions
            tag(" from "),
            digit1, // src
            tag(" to "),
            digit1, // dest
            opt(newline),
        ))(input)?;
        let (_, _, repetitions, _, src, _, dest, _) = consumed;

        // todo handle errors. Require converting errors
        let repetitions = repetitions.parse::<u32>().unwrap();
        let src = src.parse::<u8>().unwrap();
        let dest = dest.parse::<u8>().unwrap();

        let instruction = Instruction {
            repetitions,
            src,
            dest,
        };

        Ok((remaining, instruction))
    }

    pub fn parse_crate_setup(input: &str) -> (&str, Vec<Vec<char>>) {
        fn until_wrapper<'a>(input: &'a str, characters: &'a str) -> IResult<&'a str, &'a str> {
            is_not(characters)(input)
        }
        // strip off number line
        let (remainder, creates_section) = until_wrapper(input, "1").unwrap();

        let mut matrix: Vec<Vec<Option<char>>> = vec![];
        for line in creates_section.lines() {
            let (_, row) = crate_line(line).unwrap();
            if !row.is_empty() {
                matrix.push(row);
            }
        }
        let transposed = transpose_and_reverse(&matrix);
        (remainder, transposed)
    }

    pub fn parse_trash(input: &str) -> IResult<&str, &str> {
        let (remainder, _consumed) = not_line_ending(input)?;
        let (remainder, _consumed) = line_ending(remainder)?;
        let (remainder, consumed) = line_ending(remainder)?;

        Ok((remainder, consumed))
    }

    pub fn parse_instructions(input: &str) -> Vec<Instruction> {
        let mut instructions = vec![];

        let mut input = input;
        while !input.is_empty() {
            let (remaining, tmp_instruction) = instruction(input).unwrap();
            instructions.push(tmp_instruction);
            input = remaining;
        }
        instructions
    }

    #[cfg(test)]
    mod tests {
        use super::{parse_crate_setup, parse_trash};
        use crate::day_5::{
            parser::{crate_line, instruction},
            tests::EXAMPLE_INPUT,
            transpose_and_reverse, Instruction,
        };

        #[test]
        fn test_create_line() {
            const INPUT: [&str; 6] = [
                "[A]",
                "[A] [B]",
                "[F] [U] [C] [K]    ",
                "    [F] [U] [C] [K]",
                "    [F] [U] [C] [K]    ",
                "    [F] [U]     [C] [K]    ",
            ];
            let expected = [
                vec![Some('A')],
                vec![Some('A'), Some('B')],
                vec![Some('F'), Some('U'), Some('C'), Some('K'), None],
                vec![None, Some('F'), Some('U'), Some('C'), Some('K')],
                vec![None, Some('F'), Some('U'), Some('C'), Some('K'), None],
                vec![None, Some('F'), Some('U'), None, Some('C'), Some('K'), None],
            ];

            for (i, expected_output) in expected.iter().enumerate() {
                let (_, output) = crate_line(INPUT[i]).unwrap();
                assert_eq!(&output, expected_output);
            }
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

            assert_eq!(expected_matrix, transpose_and_reverse(&original_matrix));
        }

        #[test]
        fn parse_crates() {
            let expected_matrix = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
            let (_, matrix) = parse_crate_setup(EXAMPLE_INPUT);
            assert_eq!(expected_matrix, matrix);
        }

        #[test]
        fn test_instruction_parser() {
            let (remainder_and_trash, _matrix) = parse_crate_setup(EXAMPLE_INPUT);
            let (mut remainder, _trash) = parse_trash(remainder_and_trash).unwrap();
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
    use crate::day_5::{simulate_crane, Crane};

    const INPUT: &str = include_str!("../puzzle_input/day_5.txt");
    pub const EXAMPLE_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    const ANSWER: [&str; 2] = ["ZRLJGSCTR", "PRTTGRFPB"];
    const EXAMPLE_ANSWER: [&str; 2] = ["CMZ", "MCD"];

    #[test]
    fn example() {
        assert_eq!(
            simulate_crane(EXAMPLE_INPUT, &Crane::CrateMover9000),
            EXAMPLE_ANSWER[0]
        );
        assert_eq!(
            simulate_crane(EXAMPLE_INPUT, &Crane::CrateMover9001),
            EXAMPLE_ANSWER[1]
        );
    }

    #[test]
    fn problem() {
        assert_eq!(
            simulate_crane(INPUT, &Crane::CrateMover9000),
            ANSWER[0].to_string()
        );
        assert_eq!(
            simulate_crane(INPUT, &Crane::CrateMover9001),
            ANSWER[1].to_string()
        );
    }
}

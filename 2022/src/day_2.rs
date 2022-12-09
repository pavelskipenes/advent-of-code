//!
//!  # Day 2: Rock Paper Scissors
//!  The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
//!  Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//!  Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//!  The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//!  The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (`1` for `Rock`, `2` for `Paper`, and `3` for `Scissors`) plus the score for the outcome of the round (`0` if you lost, `3` if the round was a draw, and `6` if you won).
//!  Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//!   For example, suppose you were given the following strategy guide:
//!  ```
//!  let input = r"
//!  A Y
//!  B X
//!  C Z
//!  ";
//!  ```
//!  This strategy guide predicts and recommends the following:
//!  In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//!  In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
//!  The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
//!  In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//!  What would your total score be if everything goes exactly according to your strategy guide?

//! # Part Two
//! The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
//!
//! The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//!
//! In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
//! In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
//! In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
//! Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
//!
//! Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

#[derive(Debug)]
pub enum Error {
    CannotCreateShape,
    CannotCreateRoundOutcome,
}

pub enum DecryptionMethod {
    NextAction,
    NextOutcome,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl TryFrom<i32> for Shape {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissors),
            _ => Err(Error::CannotCreateShape),
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(Error::CannotCreateShape),
        }
    }
}

impl Shape {
    #[must_use]
    pub fn battle(&self, me: &Self) -> RoundOutcome {
        use RoundOutcome as RO;
        let opponent = self;
        match me {
            Self::Rock => match opponent {
                Self::Rock => RO::Draw,
                Self::Paper => RO::Loss,
                Self::Scissors => RO::Victory,
            },
            Self::Paper => match opponent {
                Self::Paper => RO::Draw,
                Self::Scissors => RO::Loss,
                Self::Rock => RO::Victory,
            },
            Self::Scissors => match opponent {
                Self::Scissors => RO::Draw,
                Self::Rock => RO::Loss,
                Self::Paper => RO::Victory,
            },
        }
    }

    #[must_use]
    pub fn get_shape(&self, wanted_outcome: &RoundOutcome) -> Self {
        use RoundOutcome as RO;
        let opponent = self;
        match wanted_outcome {
            RO::Draw => opponent.clone(),
            RO::Loss => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            RO::Victory => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }
}

pub enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Victory = 6,
}

impl TryFrom<char> for RoundOutcome {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Victory),
            _ => Err(Error::CannotCreateRoundOutcome),
        }
    }
}

impl TryFrom<i32> for RoundOutcome {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Loss),
            3 => Ok(Self::Draw),
            6 => Ok(Self::Victory),
            _ => Err(Error::CannotCreateRoundOutcome),
        }
    }
}

/// # Panics
///
/// Valid input is new line separated lines where each line contains two space separated character.
/// First character can only be one of 'A', 'B' or 'C'
/// Second character can only by one of 'X', 'Y' or 'Z'
/// Empty lines are valid but will be ignored.
/// lines will be trimmed so prepended and appended white space is valid and will be ignored.
/// panics on invalid input
///
#[must_use]
pub fn decrypt(input: &str, decryption_method: &DecryptionMethod) -> i32 {
    let (_opponent_points, player_points) = input
        .lines()
        .map(str::trim)
        .skip_while(|line| line.is_empty())
        // calculate points for each round
        .fold((0, 0), |(opponent_total, player_total), string| {
            // points this round
            let (opponent, player) = play({
                // extract first and last character
                let (first_char, last_char) =
                    match &string.chars().collect::<Vec<char>>() as &[char] {
                        [first_char, ' ', last_char] => (*first_char, *last_char),
                        trash => {
                            panic!(
                                "expected two space separated characters, received {:#?}",
                                trash
                            )
                        }
                    };

                // convert character into Shapes
                let opponent = Shape::try_from(first_char).unwrap();
                let player = match decryption_method {
                    DecryptionMethod::NextAction => Shape::try_from(last_char).unwrap(),
                    DecryptionMethod::NextOutcome => {
                        let wanted_outcome = RoundOutcome::try_from(last_char).unwrap();
                        opponent.get_shape(&wanted_outcome)
                    }
                };

                // use shapes and calculate outcome
                (opponent, player)
            });

            (opponent_total + opponent, player_total + player)
        });
    player_points
}

#[must_use]
pub fn play((p1, p2): (Shape, Shape)) -> (i32, i32) {
    use RoundOutcome as RO;
    // add outcome points
    let (points1, points2) = match p1.battle(&p2) {
        RO::Victory => (RO::Loss as i32, RO::Victory as i32),
        RO::Draw => (RO::Draw as i32, RO::Draw as i32),
        RO::Loss => (RO::Victory as i32, RO::Loss as i32),
    };
    // add action points
    (points1 + p1 as i32, points2 + p2 as i32)
}

#[cfg(test)]
mod tests {
    use super::RoundOutcome as RO;
    use crate::day_2::{decrypt, DecryptionMethod as DM, Shape};

    const INPUT: &str = include_str!("../puzzle_input/day_2.txt");
    const EXAMPLE_INPUT: &str = r"
A Y
B X
C Z";
    const ANSWER: [i32; 2] = [10595, 9541];
    const EXAMPLE_ANSWER: [i32; 2] = [15, 12];

    #[test]
    fn types() {
        assert_eq!(Shape::Rock as i32, 1);
        assert_eq!(Shape::Paper as i32, 2);
        assert_eq!(Shape::Scissors as i32, 3);

        assert_eq!(RO::Loss as i32, 0);
        assert_eq!(RO::Draw as i32, 3);
        assert_eq!(RO::Victory as i32, 6);
    }

    #[test]
    fn draw() {
        const DEC_METHOD: &DM = &DM::NextAction;

        assert_eq!(
            Shape::Rock as i32 + RO::Draw as i32,
            decrypt("A X", DEC_METHOD)
        );
        assert_eq!(
            Shape::Paper as i32 + RO::Draw as i32,
            decrypt("B Y", DEC_METHOD)
        );
        assert_eq!(
            Shape::Scissors as i32 + RO::Draw as i32,
            decrypt("C Z", DEC_METHOD)
        );
    }

    #[test]
    fn victory() {
        const DEC_METHOD: &DM = &DM::NextAction;

        assert_eq!(
            Shape::Paper as i32 + RO::Victory as i32,
            decrypt("A Y", DEC_METHOD)
        );
        assert_eq!(
            Shape::Scissors as i32 + RO::Victory as i32,
            decrypt("B Z", DEC_METHOD)
        );
        assert_eq!(
            Shape::Rock as i32 + RO::Victory as i32,
            decrypt("C X", DEC_METHOD)
        );
    }

    #[test]
    fn loss() {
        const DEC_METHOD: &DM = &DM::NextAction;

        assert_eq!(
            Shape::Scissors as i32 + RO::Loss as i32,
            decrypt("A Z", DEC_METHOD)
        );
        assert_eq!(
            Shape::Rock as i32 + RO::Loss as i32,
            decrypt("B X", DEC_METHOD)
        );
        assert_eq!(
            Shape::Paper as i32 + RO::Loss as i32,
            decrypt("C Y", DEC_METHOD)
        );
    }

    #[test]
    fn example() {
        for (index, method) in [DM::NextAction, DM::NextOutcome].iter().enumerate() {
            let points = decrypt(EXAMPLE_INPUT, method);
            assert_eq!(points, EXAMPLE_ANSWER[index]);
        }
    }

    #[test]
    fn problem() {
        for (index, method) in [DM::NextAction, DM::NextOutcome].iter().enumerate() {
            let points = decrypt(INPUT, method);
            assert_eq!(points, ANSWER[index]);
        }
    }
}

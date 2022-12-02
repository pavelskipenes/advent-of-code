#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::style)]
#![deny(clippy::undocumented_unsafe_blocks)]

pub mod day_2 {
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

    pub enum RoundOutcome {
        Loss = 0,
        Draw = 3,
        Victory = 6,
    }

    pub enum DecryptionMethod {
        NextAction,
        NextOutcome,
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

    impl Into<i32> for RoundOutcome {
        fn into(self) -> i32 {
            match self {
                Self::Loss => 0,
                Self::Draw => 3,
                Self::Victory => 6,
            }
        }
    }

    #[derive(Debug)]
    pub enum Error {
        CannotCreateAction,
        CannotCreateRoundOutcome,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Action {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    impl TryFrom<char> for Action {
        type Error = Error;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            // X,Y,Z means next action
            match value.to_ascii_uppercase() {
                'A' | 'X' => Ok(Self::Rock),
                'B' | 'Y' => Ok(Self::Paper),
                'C' | 'Z' => Ok(Self::Scissors),
                _ => Err(Error::CannotCreateAction),
            }
        }
    }

    impl Action {
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
        pub fn get_next_move(&self, wanted_outcome: &RoundOutcome) -> Self {
            use RoundOutcome as RO;
            let opponent = self;
            match wanted_outcome {
                RO::Draw => opponent.clone(),
                RO::Loss => match opponent {
                    Action::Rock => Action::Scissors,
                    Action::Paper => Action::Rock,
                    Action::Scissors => Action::Paper,
                },
                RO::Victory => match opponent {
                    Action::Rock => Action::Paper,
                    Action::Paper => Action::Scissors,
                    Action::Scissors => Action::Rock,
                },
            }
        }
    }

    #[must_use]
    pub fn decrypt(input: &str, decryption_method: &DecryptionMethod) -> i32 {
        let result =
            input
                .lines()
                .skip_while(|val| val.trim() == "")
                .fold((0, 0), |(p1, p2), string| {
                    let (out_1, out_2) = battle(match decryption_method {
                        DecryptionMethod::NextAction => decrypt_line_next_action(string.trim()),
                        DecryptionMethod::NextOutcome => decrypt_line_next_outcome(string.trim()),
                    });

                    (p1 + out_1, p2 + out_2)
                });
        result.1
    }

    fn decrypt_line_next_action(line: &str) -> (Action, Action) {
        let mut split = line.split(' ');
        let p1 = split.next().unwrap().chars().next().unwrap();
        let p2 = split.next().unwrap().chars().next().unwrap();
        let p1 = Action::try_from(p1).unwrap();
        let p2 = Action::try_from(p2).unwrap();

        (p1, p2)
    }

    #[must_use]
    pub fn battle((p1, p2): (Action, Action)) -> (i32, i32) {
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

    /// # Panics
    #[must_use]
    pub fn decrypt_line_next_outcome(input: &str) -> (Action, Action) {
        let mut split = input.split(' ');

        let opponents_move = split.next().unwrap().chars().next().unwrap();
        let wanted_outcome = split.next().unwrap().chars().next().unwrap();
        let opponents_move = Action::try_from(opponents_move).unwrap();
        let wanted_outcome = RoundOutcome::try_from(wanted_outcome).unwrap();

        let next_move = opponents_move.get_next_move(&wanted_outcome);

        (opponents_move, next_move)
    }

    #[cfg(test)]
    mod tests {
        // problem 1
        #[test]
        fn test_types() {
            use super::RoundOutcome as RO;
            use super::*;

            assert_eq!(Action::Rock as i32, 1);
            assert_eq!(Action::Paper as i32, 2);
            assert_eq!(Action::Scissors as i32, 3);

            assert_eq!(RO::Loss as i32, 0);
            assert_eq!(RO::Draw as i32, 3);
            assert_eq!(RO::Victory as i32, 6);
        }

        #[test]
        fn test_decryption_next_action() {
            use super::*;
            assert_eq!(
                (Action::Rock, Action::Rock),
                decrypt_line_next_action("A X")
            );
            assert_eq!(
                (Action::Rock, Action::Paper),
                decrypt_line_next_action("A Y")
            );
            assert_eq!(
                (Action::Rock, Action::Scissors),
                decrypt_line_next_action("A Z")
            );
            assert_eq!(
                (Action::Paper, Action::Rock),
                decrypt_line_next_action("B X")
            );
            assert_eq!(
                (Action::Paper, Action::Paper),
                decrypt_line_next_action("B Y")
            );
            assert_eq!(
                (Action::Paper, Action::Scissors),
                decrypt_line_next_action("B Z")
            );
            assert_eq!(
                (Action::Scissors, Action::Rock),
                decrypt_line_next_action("C X")
            );
            assert_eq!(
                (Action::Scissors, Action::Paper),
                decrypt_line_next_action("C Y")
            );
            assert_eq!(
                (Action::Scissors, Action::Scissors),
                decrypt_line_next_action("C Z")
            );
        }

        #[test]
        fn tets_draw() {
            use super::RoundOutcome as RO;
            use super::*;

            let dec_method = DecryptionMethod::NextAction;

            assert_eq!(
                Action::Rock as i32 + RO::Draw as i32,
                decrypt("A X", &dec_method)
            );
            assert_eq!(
                Action::Paper as i32 + RO::Draw as i32,
                decrypt("B Y", &dec_method)
            );
            assert_eq!(
                Action::Scissors as i32 + RO::Draw as i32,
                decrypt("C Z", &dec_method)
            );
        }

        #[test]
        fn tets_victory() {
            use super::RoundOutcome as RO;
            use super::*;
            let dec_method = DecryptionMethod::NextAction;

            assert_eq!(
                Action::Paper as i32 + RO::Victory as i32,
                decrypt("A Y", &dec_method)
            );
            assert_eq!(
                Action::Scissors as i32 + RO::Victory as i32,
                decrypt("B Z", &dec_method)
            );
            assert_eq!(
                Action::Rock as i32 + RO::Victory as i32,
                decrypt("C X", &dec_method)
            );
        }

        #[test]
        fn tets_loss() {
            use super::RoundOutcome as RO;
            use super::*;
            let dec_method = DecryptionMethod::NextAction;

            assert_eq!(
                Action::Scissors as i32 + RO::Loss as i32,
                decrypt("A Z", &dec_method)
            );
            assert_eq!(
                Action::Rock as i32 + RO::Loss as i32,
                decrypt("B X", &dec_method)
            );
            assert_eq!(
                Action::Paper as i32 + RO::Loss as i32,
                decrypt("C Y", &dec_method)
            );
        }

        #[test]
        fn test_problem_1() {
            // prelude
            use super::*;

            let string = include_str!("../puzzle_input/day_2/problem_1");
            let dec_method = DecryptionMethod::NextAction;
            let result = decrypt(string, &dec_method);
            assert_eq!(10595, result);
        }

        // problem 2
        #[test]
        fn test_example_2() {
            use super::*;
            let input = r"
            A Y
            B X
            C Z";

            let decryption_method = DecryptionMethod::NextOutcome;
            let points = decrypt(input, &decryption_method);
            assert_eq!(points, 12);
        }

        #[test]
        fn test_problem_2() {
            use super::*;
            let input = include_str!("../puzzle_input/day_2/problem_1");

            let decryption_method = DecryptionMethod::NextOutcome;
            let points = decrypt(input, &decryption_method);
            assert_eq!(9541, points);
        }
    }
}

pub mod day_1 {

    //! # Day 1: Calorie Counting
    //! ## Part 1
    //! Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
    //!
    //! To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
    //!
    //! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
    //!
    //! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
    //!
    //! The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
    //!
    //! For example, suppose the Elves finish writing their items' Calories and end up with the following list:
    //! ### Example
    //! ```rust
    //! use advent_of_code_2022::day_1::elf_carrying_most_calories;
    //! const INPUT: &str = r"
    //! 1000
    //! 2000
    //! 3000
    //!
    //! 4000
    //!
    //! 5000
    //! 6000
    //!
    //! 7000
    //! 8000
    //! 9000
    //!
    //! 10000";
    //!
    //! const ANSWER: &str = "24000";
    //!
    //! let result = elf_carrying_most_calories(INPUT, 1);
    //! assert_eq!(result, ANSWER);
    //! ```
    //! This list represents the Calories of the food carried by five Elves:
    //!
    //! The first Elf is carrying food with `1000`, `2000`, and `3000` Calories, a total of `6000` Calories.
    //! The second Elf is carrying one food item with `4000` Calories.
    //! The third Elf is carrying food with `5000` and `6000` Calories, a total of `11000` Calories.
    //! The fourth Elf is carrying food with `7000`, `8000`, and `9000` Calories, a total of `24000` Calories.
    //! The fifth Elf is carrying one food item with `10000` Calories.
    //! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is `24000` (carried by the fourth Elf).
    //!
    //! Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    //!
    //! ## Part 2
    //! By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.
    //!
    //! To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.
    //!
    //! In the example above, the top three Elves are the fourth Elf (with `24000` Calories), then the third Elf (with `11000` Calories), then the fifth Elf (with `10000` Calories). The sum of the Calories carried by these three elves is `45000`.
    //!
    //! Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    //! ### Example
    //! ```rust
    //! use advent_of_code_2022::day_1::elf_carrying_most_calories;
    //! const INPUT: &str = r"
    //! 1000
    //! 2000
    //! 3000
    //!
    //! 4000
    //!
    //! 5000
    //! 6000
    //!
    //! 7000
    //! 8000
    //! 9000
    //!
    //! 10000";
    //!
    //! const ANSWER: &str = "45000";
    //!
    //! let result = elf_carrying_most_calories(INPUT, 3);
    //! assert_eq!(result, ANSWER);
    //! ```

    #[must_use]
    /// Returns number of calories `top` elfes are carrying as string. Elfes separate their own
    /// inventory in `inventory_all` from each other (if any) with double new line feed `\n\n`
    /// and they separate calories for each of their products by a single new line feed `\n`.
    pub fn elf_carrying_most_calories(inventory_all: &str, top: usize) -> String {
        let mut calories_carried_per_elfs = inventory_all
            .split("\n\n")
            .map(|inventory_one| -> u64 {
                let calories: Vec<u64> = inventory_one
                    .lines()
                    .skip_while(|line| line.trim() == "")
                    .map(|line| -> u64 { line.trim().parse::<u64>().unwrap_or(0) })
                    .collect();
                calories.iter().sum::<u64>()
            })
            .collect::<Vec<u64>>();

        calories_carried_per_elfs.sort_unstable();
        calories_carried_per_elfs.reverse();

        calories_carried_per_elfs
            .iter()
            .take(top)
            .sum::<u64>()
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test() {
            // prelude
            use super::*;

            let buffer = include_str!("../puzzle_input/day_1/problem_1");

            // run tests
            let result = elf_carrying_most_calories(buffer, 1);
            assert_eq!(result, "69289");
            let result = elf_carrying_most_calories(buffer, 3);
            assert_eq!(result, "205615");
        }
    }
}
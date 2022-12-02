
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

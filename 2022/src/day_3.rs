//! One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
//! # Day 3: Rucksack Reorganization
//! Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, `a` and `A` refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
//!
//! For example, suppose you have the following list of contents from six rucksacks:
//!
//! ```
//! const INPUT: &str = r"
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ";
//! ```
//! The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
//! The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and `rsFMfFZSrLrFZsSL`. The only item type that appears in both compartments is uppercase L.
//! The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the only common item type is uppercase P.
//! The fourth rucksack's compartments only share item type `v`.
//! The fifth rucksack's compartments only share item type `t`.
//! The sixth rucksack's compartments only share item type `s`.
//!
//! To help prioritize item rearrangement, every item type can be converted to a priority:
//!
//! Lowercase item types `a` through `z` have priorities 1 through 26.
//! Uppercase item types `A` through `Z` have priorities 27 through 52.
//! In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (`p`), 38 (`L`), 42 (`P`), 22 (`v`), 20 (`t`), and 19 (`s`); the sum of these is 157.
//!
//! Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
//!
//! # Part Two
//! As you finish identifying the misplaced items, the Elves come to you with another issue.
//!
//! For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type `B`, then all three Elves will have item type `B` somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.
//!
//! The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.
//!
//! Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.
//!
//! Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:
//!
//! `vJrwpWtwJgWrhcsFMMfFFhFp`
//! `jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL`
//! `PmmdzqPrVvPwwTWBwg`
//! And the second group's rucksacks are the next three lines:
//!
//! `wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn`
//! `ttgJtRGJQctTZtZT`
//! `CrZsJsPPZsGzwwsLwLmpwMDw`
//! In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.
//!
//! Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (`r`) for the first group and 52 (`Z`) for the second group. The sum of these is 70.
//!
//! Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
//!

use std::collections::{hash_map::RandomState, HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Rucksack<'a> {
    compartment1: &'a str,
    compartment2: &'a str,
}

impl<'a> Rucksack<'a> {
    fn get_inventory(&self) -> Vec<char> {
        let mut output = self.compartment1.chars().collect::<Vec<char>>();
        self.compartment2
            .chars()
            .for_each(|character| output.push(character));

        // maybe return this in stead?
        // output
        // self.compartment1.chars().chain(self.compartment2.chars())

        output
    }

    fn from_str(line: &'a str) -> Self {
        let first = &line[0..line.len() / 2];
        let second = &line[line.len() / 2..];
        Rucksack {
            compartment1: first,
            compartment2: second,
        }
    }

    fn find_common_char(&self) -> char {
        let mut common_character = None;

        self.compartment2.chars().for_each(|character| {
            if self.compartment1.contains(character) {
                common_character = Some(character);
            }
        });
        common_character.unwrap()
    }
    fn sum_priorities(&self) -> u32 {
        get_priority(self.find_common_char())
    }
}

pub struct Group<'a> {
    elfes: [Rucksack<'a>; 3],
}

impl<'a> Group<'a> {
    fn from_str(rucksacks: [&'a str; 3]) -> Self {
        Group {
            elfes: rucksacks.map(Rucksack::from_str),
        }
    }

    fn find_common_char(&self) -> char {
        let binding = self.elfes.clone().map(|elf| elf.get_inventory());
        let rucksacks_iter = binding.iter().map(|characters| {
            let mut hash_set = HashSet::<char>::new();
            for character in characters.iter() {
                hash_set.insert(*character);
            }
            hash_set
        });

        let mut rucksacks = [
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
        ];
        for (i, hash_set) in rucksacks_iter.enumerate() {
            rucksacks[i] = hash_set;
        }

        // count occurrence of characters
        let mut occurrences: HashMap<char, usize> = HashMap::with_hasher(RandomState::new());

        rucksacks.iter().flatten().for_each(|character| {
            let value = occurrences.get(character).unwrap_or(&0) + 1;
            occurrences.insert(*character, value);
        });

        for (character, count) in occurrences {
            if count == 3 {
                return character;
            }
        }
        unreachable!();
    }

    #[must_use]
    pub fn sum_priorities(&self) -> u32 {
        let [g1, g2, g3] = self.elfes.clone().map(|elf| elf.sum_priorities());
        g1 + g2 + g3
    }
}

#[must_use]
pub fn sum_groups(groups: &[Group]) -> u32 {
    groups
        .iter()
        .map(Group::find_common_char)
        .map(get_priority)
        .sum()
}

#[must_use]
pub fn create_groups(input: &str) -> Vec<Group> {
    // trim and filter out empty lines
    let lines = input
        .lines()
        .map(str::trim)
        .skip_while(|line| line.is_empty());

    let mut groups = vec![];

    for chunk in lines.collect::<Vec<&str>>().chunks(3) {
        groups.push(Group::from_str([chunk[0], chunk[1], chunk[2]]));
    }
    groups
}

#[must_use]
pub fn get_priority(character: char) -> u32 {
    match character {
        'a'..='z' => character as u8 - b'a' + 1,
        'A'..='Z' => character as u8 - b'A' + 27,
        _ => unreachable!(),
    }
    .into()
}

#[must_use]
pub fn sum_priorities(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .skip_while(|&line| line.is_empty())
        .map(|line| {
            let rucksack = Rucksack::from_str(line);
            get_priority(rucksack.find_common_char())
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        include_str!("../puzzle_input/day_3.txt")
    }

    #[test]
    fn test_priority() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('P'), 42);
        assert_eq!(get_priority('v'), 22);
        assert_eq!(get_priority('t'), 20);
        assert_eq!(get_priority('s'), 19);
    }
    #[test]
    fn test_common_char() {
        const INPUT: &str = r"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        const ANSWERS: [char; 6] = ['p', 'L', 'P', 'v', 't', 's'];

        let lines: Vec<&str> = INPUT
            .lines()
            .map(str::trim)
            .skip_while(|&line| str::is_empty(line))
            .collect();

        for i in 0..6 {
            assert_eq!(Rucksack::from_str(lines[i]).find_common_char(), ANSWERS[i]);
        }
    }

    #[test]
    fn test_example_1() {
        const INPUT: &str = r"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_priorities(INPUT), 157);
    }

    #[test]
    fn test_problem_1() {
        let input = get_input();
        assert_eq!(sum_priorities(input), 8515);
    }

    #[test]
    fn test_example_2() {
        const INPUT: &str = r"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let groups = create_groups(INPUT);

        assert_eq!(groups[0].find_common_char(), 'r');
        assert_eq!(groups[1].find_common_char(), 'Z');
        assert_eq!(get_priority(groups[0].find_common_char()), 18);
        assert_eq!(get_priority(groups[1].find_common_char()), 52);

        assert_eq!(sum_groups(&groups), 70);
    }

    #[test]
    fn test_problem_2() {
        let input = get_input();
        let groups = create_groups(input);
        let sum = sum_groups(&groups);
        assert_eq!(sum, 2434);
    }
}

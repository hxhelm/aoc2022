extern crate core;

use std::collections::HashSet;
use std::str::{FromStr};
use itertools::{Itertools};

#[derive(Debug)]
struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

impl Rucksack {
    fn get_common_item(&self) -> char {
        *self.first_compartment.intersection(&self.second_compartment)
            .next()
            .unwrap()
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_part, second_part) = s.split_at(s.len() / 2);

        Ok(Rucksack {
            first_compartment: first_part
                .chars()
                .collect(),
            second_compartment: second_part
                .chars()
                .collect(),
        })
    }
}

fn ascii_char_to_priority (character: char) -> u32 {
    let priority = match character {
        // ascii 'A' starts at 65, mapping to values 27-52
        _ if character.is_ascii_uppercase() => character as u32 - 65 + 27,
        // ascii 'a' starts at 97, mapping to values 0-26
        _ if character.is_ascii_lowercase() => character as u32 - 97 + 1,
        _ => panic!("Unsupported character"),
    };

    priority
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| Rucksack::from_str(line).unwrap())
        .map(|rucksack| ascii_char_to_priority(rucksack.get_common_item()))
        .sum();

    Some(result)
}

// ------------------------
//          Part 2
// ------------------------

struct Group {
    first_elf: HashSet<char>,
    second_elf: HashSet<char>,
    third_elf: HashSet<char>,
}

impl Group {
    fn get_common_item(&self) -> char {
        let intersection : HashSet<char> = self.first_elf.intersection(
            &self.second_elf
        ).copied().collect();

        let intersection: HashSet<char> = intersection.intersection(
            &self.third_elf
        ).copied().collect();

        *intersection.iter().next().unwrap()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups: Vec<Group> = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| -> Group {
            Group {
                first_elf: chunk.next().unwrap().chars().collect(),
                second_elf: chunk.next().unwrap().chars().collect(),
                third_elf: chunk.next().unwrap().chars().collect(),
            }
        })
        .collect();

    let result = groups
        .iter()
        .map(|group| ascii_char_to_priority(group.get_common_item()))
        .sum();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

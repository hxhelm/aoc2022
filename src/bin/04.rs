use std::ops::{RangeInclusive};
use std::str::FromStr;

#[derive(Debug)]
struct SectionRange {
    range: RangeInclusive<i32>,
}

impl FromStr for SectionRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once("-") {
            Ok(SectionRange {
                range: (start.parse::<i32>().unwrap()..=end.parse::<i32>().unwrap())
            })
        } else {
            panic!("Could not split move");
        }
    }
}

#[derive(Debug)]
struct Pair {
    first: SectionRange,
    second: SectionRange,
}

impl Pair {
    fn is_one_fully_contained(&self) -> bool {
        (self.first.range.start() >= self.second.range.start()
            && self.first.range.end() <= self.second.range.end())
        || (self.second.range.start() >= self.first.range.start()
            && self.second.range.end() <= self.first.range.end())
    }

    fn is_overlapping(&self) -> bool {
        self.first.range.start() <= self.second.range.end()
        && self.first.range.end() >= self.second.range.start()
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first_range, second_range)) = s.split_once(",") {
            Ok(Pair {
                first: SectionRange::from_str(first_range).unwrap(),
                second: SectionRange::from_str(second_range).unwrap(),
            })
        } else {
            panic!("Could not split move");
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let contained_pairs: Vec<Pair> = input
        .lines()
        .map(|line| Pair::from_str(line).unwrap())
        .filter(|pair| pair.is_one_fully_contained())
        .collect();

    Some(u32::try_from(contained_pairs.len()).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let overlapping_pairs: Vec<Pair> = input
        .lines()
        .map(|line| Pair::from_str(line).unwrap())
        .filter(|pair| pair.is_overlapping())
        .collect();

    Some(u32::try_from(overlapping_pairs.len()).unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

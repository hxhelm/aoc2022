use std::collections::VecDeque;
use itertools::Itertools;

fn search_distinct_character_block(input: &str, block_length: usize) -> Option<u32> {
    if input.len() < block_length {
        return None;
    }

    let mut deq = VecDeque::from_iter(input[0..block_length].chars());
    let mut block_start = u32::try_from(block_length).unwrap();

    for char in input.chars().skip(block_length) {
        if deq.iter().all_unique() {
            break;
        }

        deq.push_back(char);
        deq.pop_front();
        block_start += 1;
    }

    if usize::try_from(block_start).unwrap() < input.len() {
        Some(block_start)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    search_distinct_character_block(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    search_distinct_character_block(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}

use std::collections::{BTreeMap, HashMap};
use itertools::enumerate;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::{complete, is_alphabetic};
use nom::character::complete::{alpha1, anychar, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};

#[derive(Copy, Clone, Debug)]
struct MoveInstruction {
    from: u32,
    to: u32,
    amount: u32,
}

fn parse_instruction(input: &str) -> IResult<&str, MoveInstruction> {
    let (input, (_, amount, _, from, _, to)) = tuple((
        tag("move "),
        complete::u32,
        tag(" from "),
        complete::u32,
        tag(" to "),
        complete::u32,
    ))(input)?;

    Ok((input, MoveInstruction{from, to, amount}))
}

fn parse_instruction_lines(input: &str) -> IResult<&str, Vec<MoveInstruction>> {
    let (input, instructions)
        = separated_list1(newline, parse_instruction)(input)?;

    Ok((input, instructions))
}

#[derive(Debug)]
struct CrateStacks {
    stacks: BTreeMap<u32, Vec<char>>,
}

impl CrateStacks {
    fn from_grid(grid: Vec<Vec<Option<char>>>) -> Self {
        let mut stacks: BTreeMap<u32, Vec<char>> = BTreeMap::new();

        for stack in grid.iter().rev() {
            for (index, &crate_character) in stack.iter().enumerate() {
                let index = u32::try_from(index).unwrap() + 1;
                if crate_character != None {
                    if let Some(col) = stacks.get_mut(&index) {
                        col.push(crate_character.unwrap());
                    } else {
                        stacks.insert(
                            index,
                            vec![crate_character.unwrap()]
                        );
                    }
                }
            }
        }

        CrateStacks {
            stacks
        }
    }

    fn execute_instruction_single(&mut self, instruction: &MoveInstruction) {
        let mut queue = vec![];

        for _ in 1..=instruction.amount {
            queue.push(
                self.stacks.get_mut(&instruction.from)
                    .unwrap()
                    .pop()
            )
        }

        let target = self.stacks.get_mut(&instruction.to).unwrap();

        for crate_character in queue.iter() {
            if let Some(crate_character) = crate_character {
                target.push(*crate_character);
            }
        }
    }

    fn execute_instruction_multiple(&mut self, instruction: &MoveInstruction) {
        let mut tower = vec![];

        for _ in 1..=instruction.amount {
            tower.push(
                self.stacks.get_mut(&instruction.from)
                    .unwrap()
                    .pop()
            )
        }

        let target = self.stacks.get_mut(&instruction.to).unwrap();

        for crate_character in tower.iter().rev() {
            if let Some(crate_character) = crate_character {
                target.push(*crate_character);
            }
        }
    }

    fn top_row(&self) -> String {
        let mut message = vec![];
        for (_index, stack) in self.stacks.iter() {
            if let Some(&character) = stack.last() {
                message.push(character);
            }
        }

        message.iter().collect()
    }
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = alt((
        tag("   "),
        delimited(
            complete::char('['),
            alpha1,
            complete::char(']')
        ),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => {
            let (input, crate_char) = anychar(value)?;
            Some(crate_char)
        }
    };
    Ok((input, result))
}

fn parse_crate_stacks(input: &str) -> IResult<&str, CrateStacks> {
    let (input, stacks) = separated_list1(
        newline,
        separated_list1(
            tag(" "),
            parse_crate,
        ),
    )(input)?;

    let crate_stacks = CrateStacks::from_grid(stacks);

    Ok((input, crate_stacks))
}

pub fn part_one(input: &str) -> Option<String> {
    let (crate_stacks, instructions) = input.split_once("\n\n").unwrap();

    let (_, mut crate_stacks) = parse_crate_stacks(crate_stacks).unwrap();

    let (_, instructions) = parse_instruction_lines(instructions).unwrap();

    for instruction in instructions.iter() {
        crate_stacks.execute_instruction_single(instruction);
    }

    Some(crate_stacks.top_row())
}

pub fn part_two(input: &str) -> Option<String> {
    let (crate_stacks, instructions) = input.split_once("\n\n").unwrap();

    let (_, mut crate_stacks) = parse_crate_stacks(crate_stacks).unwrap();

    let (_, instructions) = parse_instruction_lines(instructions).unwrap();

    for instruction in instructions.iter() {
        crate_stacks.execute_instruction_multiple(instruction);
    }

    Some(crate_stacks.top_row())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(&part_one(&input).unwrap(), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(&part_two(&input).unwrap(), "MCD");
    }
}

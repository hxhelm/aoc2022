use std::str::FromStr;
use crate::Move::{Paper, Rock, Scissors};

#[derive(Copy, Clone, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Unknown move"),
        })
    }
}

trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Move {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

struct Round {
    opponent: Move,
    player: Move,
}

impl Round {
    fn get_move_score(&self) -> u32 {
        self.player as u32
    }

    fn get_battle_score(&self) -> u32 {
        match self {
            _ if self.opponent.beats() == self.player => 0,
            _ if self.player.beats() == self.opponent => 6,
            _ => 3
        }
    }

    fn get_player_score(&self) -> u32 {
        self.get_move_score() + self.get_battle_score()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|s| -> Round {
            if let Some((opponent_move, player_move)) = s.split_once(" ") {
                Round {
                    opponent: Move::from_str(opponent_move).unwrap(),
                    player: Move::from_str(player_move).unwrap(),
                }
            } else {
                panic!("Could not split move");
            }
        })
        .map(|round| round.get_player_score())
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input.lines()
        .map(|s| -> Round {
            if let Some((opponent_move, player_move)) = s.split_once(" ") {
                Round {
                    opponent: Move::from_str(opponent_move).unwrap(),
                    player: match player_move {
                        "X" => Move::from_str(opponent_move).unwrap().beats(),
                        "Y" => Move::from_str(opponent_move).unwrap(),
                        "Z" => match Move::from_str(opponent_move).unwrap() {
                            Rock => Paper,
                            Paper => Scissors,
                            Scissors => Rock,
                        },
                        _ => panic!("Unhandled move"),
                    },
                }
            } else {
                panic!("Could not split move");
            }
        })
        .map(|round| round.get_player_score())
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

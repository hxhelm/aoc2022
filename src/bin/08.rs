use grid::Grid;
use itertools::enumerate;
use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult, Parser,
};
use std::cmp;

fn parse_grid(input: &str) -> IResult<&str, Grid<u32>> {
    let (input, rows) = separated_list1(
        newline,
        digit1.map(|digits_row: &str| {
            digits_row
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        }),
    )(input)?;

    let mut grid = Grid::new(0, 0);

    for row in rows {
        grid.push_row(row)
    }

    Ok((input, grid))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_grid(input).unwrap();

    let mut visible_count = 0;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if row == 0 || row == grid.rows() - 1 || col == 0 || col == grid.cols() - 1 {
                visible_count += 1;
                continue;
            }

            let tree: u32 = *grid.get(row, col).unwrap();

            let mut from_left = enumerate(grid.iter_row(row))
                .filter(|&(i, &t)| i < col && t >= tree)
                .peekable();
            if from_left.peek().is_none() {
                visible_count += 1;
                continue;
            }

            let mut from_right = enumerate(grid.iter_row(row))
                .rev()
                .filter(|&(i, &t)| i > col && t >= tree)
                .peekable();
            if from_right.peek().is_none() {
                visible_count += 1;
                continue;
            }

            let mut from_down = enumerate(grid.iter_col(col))
                .rev()
                .filter(|&(i, &t)| i > row && t >= tree)
                .peekable();
            if from_down.peek().is_none() {
                visible_count += 1;
                continue;
            }

            let mut from_top = enumerate(grid.iter_col(col))
                .filter(|&(i, &t)| i < row && t >= tree)
                .peekable();
            if from_top.peek().is_none() {
                visible_count += 1;
                continue;
            }
        }
    }

    Some(visible_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, grid) = parse_grid(input).unwrap();

    let mut top_scenic_score = 0;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let tree: u32 = *grid.get(row, col).unwrap();

            let check_left = enumerate(grid.iter_row(row))
                .rev()
                .filter(|&(i, _)| i < col)
                .collect::<Vec<(_, _)>>();
            let mut left_score = 0;
            for (_, &t) in check_left {
                left_score += 1;
                if t >= tree {
                    break;
                }
            }

            let check_right = enumerate(grid.iter_row(row))
                .filter(|&(i, _)| i > col)
                .collect::<Vec<(_, _)>>();
            let mut right_score = 0;
            for (_, &t) in check_right {
                right_score += 1;
                if t >= tree {
                    break;
                }
            }

            let check_down = enumerate(grid.iter_col(col))
                .filter(|&(i, _)| i > row)
                .collect::<Vec<(_, _)>>();
            let mut down_score = 0;
            for (_, &t) in check_down {
                down_score += 1;
                if t >= tree {
                    break;
                }
            }

            let check_top = enumerate(grid.iter_col(col))
                .rev()
                .filter(|&(i, _)| i < row)
                .collect::<Vec<(_, _)>>();
            let mut top_score = 0;
            for (_, &t) in check_top {
                top_score += 1;
                if t >= tree {
                    break;
                }
            }

            let scenic_score = {
                cmp::max(left_score, 1)
                    * cmp::max(right_score, 1)
                    * cmp::max(down_score, 1)
                    * cmp::max(top_score, 1)
            };

            if scenic_score > top_scenic_score {
                top_scenic_score = scenic_score;
            }
        }
    }

    Some(top_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(16));
    }
}
